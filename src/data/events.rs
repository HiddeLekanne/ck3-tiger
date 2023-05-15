use fnv::FnvHashMap;
use std::path::{Path, PathBuf};

use crate::block::validator::Validator;
use crate::block::{Block, BlockOrValue, DefinitionItem};
use crate::context::ScopeContext;
use crate::data::scripted_effects::Effect;
use crate::data::scripted_triggers::Trigger;
use crate::desc::validate_desc;
use crate::effect::{validate_effect, validate_normal_effect};
use crate::errorkey::ErrorKey;
use crate::errors::{error, error_info, warn, warn_info};
use crate::everything::Everything;
use crate::fileset::{FileEntry, FileHandler};
use crate::helpers::dup_error;
use crate::item::Item;
use crate::pdxfile::PdxFile;
use crate::scopes::{scope_from_snake_case, Scopes};
use crate::token::Token;
use crate::trigger::{validate_normal_trigger, validate_target};
use crate::validate::{
    validate_cooldown, validate_theme_background, validate_theme_icon, validate_theme_sound,
    ListType,
};

#[derive(Clone, Debug, Default)]
pub struct Events {
    events: FnvHashMap<String, Event>,
    triggers: FnvHashMap<(PathBuf, String), Trigger>,
    effects: FnvHashMap<(PathBuf, String), Effect>,

    // These events are known to exist, so don't warn abour them not being found,
    // but they had errors on validation.
    error_events: FnvHashMap<String, Token>,
}

impl Events {
    fn load_event(&mut self, key: &Token, block: &Block, namespaces: &[&str]) {
        let mut namespace_ok = false;
        if namespaces.is_empty() {
            error(
                key,
                ErrorKey::EventNamespace,
                "Event files must start with a namespace declaration",
            );
        } else if let Some((key_a, key_b)) = key.as_str().split_once('.') {
            if key_b.chars().all(|c| c.is_ascii_digit()) {
                if namespaces.contains(&key_a) {
                    namespace_ok = true;
                } else {
                    warn_info(key, ErrorKey::EventNamespace, "Event name should start with namespace", "If the event doesn't match its namespace, the game can't properly find the event when triggering it.");
                }
            } else {
                warn_info(key, ErrorKey::EventNamespace, "Event names should be in the form NAMESPACE.NUMBER", "where NAMESPACE is the namespace declared at the top of the file, and NUMBER is a series of digits.");
            }
        } else {
            warn_info(key, ErrorKey::EventNamespace, "Event names should be in the form NAMESPACE.NUMBER", "where NAMESPACE is the namespace declared at the top of the file, and NUMBER is a series of digits.");
        }

        if namespace_ok {
            if let Some(other) = self.events.get(key.as_str()) {
                dup_error(key, &other.key, "event");
            }
            self.events
                .insert(key.to_string(), Event::new(key.clone(), block.clone()));
        } else {
            self.error_events.insert(key.to_string(), key.clone());
        }
    }

    fn load_scripted_trigger(&mut self, key: Token, block: &Block) {
        let index = (key.loc.pathname.to_path_buf(), key.to_string());
        if let Some(other) = self.triggers.get(&index) {
            dup_error(&key, &other.key, "scripted trigger");
        }
        self.triggers
            .insert(index, Trigger::new(key, block.clone()));
    }

    fn load_scripted_effect(&mut self, key: Token, block: &Block) {
        let index = (key.loc.pathname.to_path_buf(), key.to_string());
        if let Some(other) = self.effects.get(&index) {
            dup_error(&key, &other.key, "scripted effect");
        }
        self.effects.insert(index, Effect::new(key, block.clone()));
    }

    pub fn trigger_exists(&self, key: &Token) -> bool {
        let index = (key.loc.pathname.to_path_buf(), key.to_string());
        self.triggers.contains_key(&index)
    }

    pub fn get_trigger(&self, key: &Token) -> Option<&Trigger> {
        let index = (key.loc.pathname.to_path_buf(), key.to_string());
        self.triggers.get(&index)
    }

    pub fn effect_exists(&self, key: &Token) -> bool {
        let index = (key.loc.pathname.to_path_buf(), key.to_string());
        self.effects.contains_key(&index)
    }

    pub fn get_effect(&self, key: &Token) -> Option<&Effect> {
        let index = (key.loc.pathname.to_path_buf(), key.to_string());
        self.effects.get(&index)
    }

    pub fn exists(&self, key: &str) -> bool {
        self.events.contains_key(key) || self.error_events.contains_key(key)
    }

    pub fn validate(&self, data: &Everything) {
        let mut vec = self.effects.values().collect::<Vec<&Effect>>();
        vec.sort_unstable_by_key(|item| &item.key.loc);
        for item in vec {
            item.validate(data);
        }

        let mut vec = self.triggers.values().collect::<Vec<&Trigger>>();
        vec.sort_unstable_by_key(|item| &item.key.loc);
        for item in vec {
            item.validate(data);
        }

        let mut vec = self.events.values().collect::<Vec<&Event>>();
        vec.sort_unstable_by_key(|item| &item.key.loc);
        for item in vec {
            item.validate(data);
        }
    }
}

impl FileHandler for Events {
    fn subpath(&self) -> PathBuf {
        PathBuf::from("events")
    }

    fn handle_file(&mut self, entry: &FileEntry, fullpath: &Path) {
        #[derive(Copy, Clone)]
        enum Expecting {
            Event,
            ScriptedTrigger,
            ScriptedEffect,
        }

        if !entry.filename().to_string_lossy().ends_with(".txt") {
            return;
        }

        let Some(block) = PdxFile::read(entry, fullpath) else { return };

        let mut namespaces = Vec::new();
        let mut expecting = Expecting::Event;

        for def in block.iter_definitions_warn() {
            match def {
                DefinitionItem::Assignment(key, value) if key.is("namespace") => {
                    namespaces.push(value.as_str());
                }
                DefinitionItem::Assignment(key, _)
                    if key.is("scripted_trigger") || key.is("scripted_effect") =>
                {
                    error(
                        key,
                        ErrorKey::Validation,
                        &format!("`{key}` should be used without `=`"),
                    );
                }
                DefinitionItem::Assignment(key, _) => {
                    error(key, ErrorKey::Validation, "unknown setting in event files");
                }
                DefinitionItem::Keyword(key)
                    if matches!(expecting, Expecting::Event) && key.is("scripted_trigger") =>
                {
                    expecting = Expecting::ScriptedTrigger;
                }
                DefinitionItem::Keyword(key)
                    if matches!(expecting, Expecting::Event) && key.is("scripted_effect") =>
                {
                    expecting = Expecting::ScriptedEffect;
                }
                DefinitionItem::Keyword(key) => error_info(
                    key,
                    ErrorKey::Validation,
                    "unexpected token",
                    "Did you forget an = ?",
                ),
                DefinitionItem::Definition(key, b) if key.is("namespace") => {
                    error(
                        b,
                        ErrorKey::EventNamespace,
                        "expected namespace to have a simple string value",
                    );
                }
                DefinitionItem::Definition(key, b) => match expecting {
                    Expecting::ScriptedTrigger => {
                        self.load_scripted_trigger(key.clone(), b);
                        expecting = Expecting::Event;
                    }
                    Expecting::ScriptedEffect => {
                        self.load_scripted_effect(key.clone(), b);
                        expecting = Expecting::Event;
                    }
                    Expecting::Event => {
                        self.load_event(key, b, &namespaces);
                    }
                },
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Event {
    key: Token,
    block: Block,
}

const EVENT_TYPES: &[&str] = &[
    "letter_event",
    "character_event",
    "court_event",
    "duel_event",
    "fullscreen_event",
    "activity_event",
];

// TODO: check if mods can add more window types to gui/event_windows/
const WINDOW_TYPES: &[&str] = &[
    "character_event",
    "duel_event",
    "fullscreen_event",
    "letter_event",
];

impl Event {
    pub fn new(key: Token, block: Block) -> Self {
        Self { key, block }
    }

    pub fn validate(&self, data: &Everything) {
        let mut vd = Validator::new(&self.block, data);

        let evtype = self
            .block
            .get_field_value("type")
            .map_or("character_event", |t| t.as_str());
        if evtype == "empty" {
            let msg = "`type = empty` has been replaced by `scope = none`";
            error(vd.field_value("type").unwrap(), ErrorKey::Validation, msg);
        } else {
            vd.field_choice("type", EVENT_TYPES);
        }

        if evtype == "character_event" {
            vd.field_choice("window", WINDOW_TYPES);
        } else if evtype == "activity_event" {
            // TODO: figure out the possible values for this
            vd.field_value("window");
        } else {
            vd.ban_field("window", || "character events");
        }

        let mut sc = ScopeContext::new_root(Scopes::Character, self.key.clone());
        if let Some(token) = vd.field_value("scope") {
            if let Some(scope) = scope_from_snake_case(token.as_str()) {
                sc = ScopeContext::new_root(scope, token.clone());
            } else {
                warn(token, ErrorKey::Scopes, "unknown scope type");
            }
        }

        vd.field_bool("hidden");
        vd.field_bool("major");
        vd.field_validated_block("major_trigger", |b, data| {
            validate_normal_trigger(b, data, &mut sc, false);
        });

        vd.field_validated_block("immediate", |b, data| {
            validate_normal_effect(b, data, &mut sc, true);
        });
        vd.field_validated_block("trigger", |b, data| {
            validate_normal_trigger(b, data, &mut sc, false);
        });
        vd.field_validated_block("on_trigger_fail", |b, data| {
            validate_normal_effect(b, data, &mut sc, false);
        });
        vd.field_block("weight_multiplier"); // modifier

        if let Some(bv) = vd.field("title") {
            validate_desc(bv, data, &mut sc);
        }

        if let Some(bv) = vd.field("desc") {
            validate_desc(bv, data, &mut sc);
        }

        if evtype == "letter_event" {
            if let Some(bv) = vd.field("opening") {
                validate_desc(bv, data, &mut sc);
            }
            vd.req_field("sender");
            vd.field_validated("sender", |bv, data| validate_portrait(bv, data, &mut sc));
        } else {
            vd.advice_field("opening", "only needed for letter_event");
            vd.advice_field("sender", "only needed for letter_event");
        }
        if evtype == "court_event" {
            vd.advice_field("left_portrait", "not needed for court_event");
            vd.advice_field("right_portrait", "not needed for court_event");
            vd.advice_field("center_portrait", "not needed for court_event");
        } else {
            vd.field_validated("left_portrait", |bv, data| {
                validate_portrait(bv, data, &mut sc);
            });
            vd.field_validated("right_portrait", |bv, data| {
                validate_portrait(bv, data, &mut sc);
            });
            vd.field_validated("center_portrait", |bv, data| {
                validate_portrait(bv, data, &mut sc);
            });
        }
        vd.field_validated("lower_left_portrait", |bv, data| {
            validate_portrait(bv, data, &mut sc);
        });
        vd.field_validated("lower_center_portrait", |bv, data| {
            validate_portrait(bv, data, &mut sc);
        });
        vd.field_validated("lower_right_portrait", |bv, data| {
            validate_portrait(bv, data, &mut sc);
        });
        // TODO: check that artifacts are not in the same position as a character
        vd.field_validated_blocks("artifact", |b, data| validate_artifact(b, data, &mut sc));
        vd.field_validated_block("court_scene", validate_court_scene);
        // TODO: check defined event themes
        vd.field_value("theme");
        // TODO: warn if more than one of each is defined with no trigger
        if evtype == "court_event" {
            vd.advice_field("override_background", "not needed for court_event");
        } else {
            vd.field_validated_bvs("override_background", validate_theme_background);
        }
        vd.field_validated_blocks("override_icon", validate_theme_icon);
        vd.field_validated_blocks("override_sound", validate_theme_sound);
        // Note: override_environment seems to be unused, and themes defined in
        // common/event_themes don't have environments. So I left it out even though
        // it's in the docs.

        if !self.block.get_field_bool("hidden").unwrap_or(false) {
            vd.req_field("option");
        }
        vd.field_validated_blocks("option", |b, data| validate_event_option(b, data, &mut sc));

        vd.field_validated_block("after", |b, data| {
            // TODO: check if this block is tooltipped
            validate_normal_effect(b, data, &mut sc, false);
        });
        vd.field_validated_block("cooldown", |b, data| validate_cooldown(b, data, &mut sc));
        vd.field_value("soundeffect");
        vd.field_bool("orphan");
        // TODO: validate widget
        vd.field("widget");
        vd.field_block("widgets");
    }
}

fn validate_event_option(block: &Block, data: &Everything, sc: &mut ScopeContext) {
    // TODO: warn if they use desc, first_valid, random_valid, or triggered_desc directly
    // in the name or tooltip.

    let mut vd = Validator::new(block, data);
    vd.field_validated_bvs("name", |bv, data| match bv {
        BlockOrValue::Token(t) => {
            data.localization.verify_exists(t);
        }
        BlockOrValue::Block(b) => {
            if let Some(trigger) = b.get_field("trigger") {
                if let Some(trigger) = trigger.expect_block() {
                    validate_normal_trigger(trigger, data, sc, false);
                }
            }
            if let Some(text) = b.get_field("text") {
                validate_desc(text, data, sc);
            } else {
                warn(b, ErrorKey::Validation, "event option name with no text");
            }
        }
    });
    vd.field_validated_bvs("custom_tooltip", |bv, data| match bv {
        BlockOrValue::Token(t) => {
            data.localization.verify_exists(t);
        }
        BlockOrValue::Block(b) => {
            if let Some(text) = b.get_field("text") {
                validate_desc(text, data, sc);
            } else {
                warn(b, ErrorKey::Validation, "event option tooltip with no text");
            }
        }
    });

    vd.field_validated_block("trigger", |b, data| {
        validate_normal_trigger(b, data, sc, false);
    });

    vd.field_validated_block("show_as_unavailable", |b, data| {
        validate_normal_trigger(b, data, sc, false);
    });

    vd.field_validated_bv("flavor", |b, data| validate_desc(b, data, sc));

    // "this option is available because you have the ... trait"
    vd.field_values_items("trait", Item::Trait);
    vd.field_values_items("skill", Item::Skill);

    // TODO: check what this is. script value? modifier?
    vd.field("ai_chance");

    // TODO: check what this does.
    vd.field_bool("exclusive");

    // TODO: check what this does.
    vd.field_bool("is_cancel_option");

    // If fallback = yes, the option is shown despite its trigger,
    // if there would otherwise be no other option
    vd.field_bool("fallback");

    if let Some(token) = vd.field_value("highlight_portrait") {
        validate_target(token, data, sc, Scopes::Character);
    }

    vd.field_bool("show_unlock_reason"); // TODO: what does this do?

    validate_effect("option", ListType::None, block, data, sc, vd, true);
}

fn validate_court_scene(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);

    vd.req_field("button_position_character");
    vd.field_value("button_position_character");
    vd.field_bool("court_event_force_open");
    vd.field_bool("show_timeout_info");
    vd.field_bool("should_pause_time");
    vd.field_value("court_owner");
    vd.field("scripted_animation");
    // TODO: validate roles
    vd.field_blocks("roles");
}

fn validate_artifact(block: &Block, data: &Everything, sc: &mut ScopeContext) {
    let mut vd = Validator::new(block, data);

    vd.req_field("target");
    vd.req_field("position");
    vd.field_value("target");
    vd.field_choice(
        "position",
        &[
            "lower_left_portrait",
            "lower_center_portrait",
            "lower_right_portrait",
        ],
    );
    vd.field_validated_block("trigger", |b, data| {
        validate_normal_trigger(b, data, sc, false);
    });
}

fn validate_triggered_animation(block: &Block, data: &Everything, sc: &mut ScopeContext) {
    let mut vd = Validator::new(block, data);

    vd.req_field("trigger");
    vd.req_field("animation");
    vd.field_validated_block("trigger", |b, data| {
        validate_normal_trigger(b, data, sc, false);
    });
    vd.field_value("animation");
}

fn validate_triggered_outfit(block: &Block, data: &Everything, sc: &mut ScopeContext) {
    let mut vd = Validator::new(block, data);

    // trigger is apparently optional
    vd.field_validated_block("trigger", |b, data| {
        validate_normal_trigger(b, data, sc, false);
    });
    // TODO: check that at least one of these is set?
    vd.field_list("outfit_tags");
    vd.field_bool("remove_default_outfit");
    vd.field_bool("hide_info");
}

fn validate_portrait(v: &BlockOrValue, data: &Everything, sc: &mut ScopeContext) {
    match v {
        BlockOrValue::Token(_) => (),
        BlockOrValue::Block(b) => {
            let mut vd = Validator::new(b, data);

            vd.req_field("character");
            vd.field_value("character");
            vd.field_validated_block("trigger", |b, data| {
                validate_normal_trigger(b, data, sc, false);
            });
            vd.field_value("animation");
            vd.field("scripted_animation");
            vd.field_validated_blocks("triggered_animation", |b, data| {
                validate_triggered_animation(b, data, sc);
            });
            vd.field_list("outfit_tags");
            vd.field_bool("remove_default_outfit");
            vd.field_bool("hide_info");
            vd.field_validated_blocks("triggered_outfit", |b, data| {
                validate_triggered_outfit(b, data, sc);
            });
            // TODO: is this only useful when animation is prisondungeon ?
            vd.field_bool("override_imprisonment_visuals");
            vd.field_bool("animate_if_dead");
        }
    }
}
