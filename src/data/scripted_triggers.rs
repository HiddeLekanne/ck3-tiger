use fnv::FnvHashMap;
use std::path::{Path, PathBuf};

use crate::block::Block;
use crate::context::ScopeContext;
use crate::errorkey::ErrorKey;
use crate::errors::warn;
use crate::everything::Everything;
use crate::fileset::{FileEntry, FileHandler};
use crate::helpers::dup_error;
use crate::macrocache::MacroCache;
use crate::pdxfile::PdxFile;
use crate::scopes::{scope_from_snake_case, Scopes};
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::trigger::validate_normal_trigger;

#[derive(Clone, Debug, Default)]
pub struct Triggers {
    scope_overrides: FnvHashMap<String, Scopes>,
    triggers: FnvHashMap<String, Trigger>,
}

impl Triggers {
    fn load_item(&mut self, key: &Token, block: &Block) {
        if let Some(other) = self.triggers.get(key.as_str()) {
            if other.key.loc.kind >= key.loc.kind {
                dup_error(key, &other.key, "scripted trigger");
            }
        }
        let scope_override = self.scope_overrides.get(key.as_str()).copied();
        self.triggers.insert(
            key.to_string(),
            Trigger::new(key.clone(), block.clone(), scope_override),
        );
    }

    pub fn exists(&self, key: &str) -> bool {
        self.triggers.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Trigger> {
        self.triggers.get(key)
    }

    pub fn validate(&self, data: &Everything) {
        for item in self.triggers.values() {
            item.validate(data);
        }
    }
}

impl FileHandler for Triggers {
    fn config(&mut self, config: &Block) {
        if let Some(block) = config.get_field_block("scope_override") {
            for (key, token) in block.iter_assignments() {
                let mut scopes = Scopes::empty();
                if token.lowercase_is("all") {
                    scopes = Scopes::all();
                } else {
                    for part in token.split('|') {
                        if let Some(scope) = scope_from_snake_case(part.as_str()) {
                            scopes |= scope;
                        } else {
                            let msg = format!("unknown scope type `{part}`");
                            warn(part, ErrorKey::Config, &msg);
                        }
                    }
                }
                self.scope_overrides
                    .insert(key.as_str().to_string(), scopes);
            }
        }
    }

    fn subpath(&self) -> PathBuf {
        PathBuf::from("common/scripted_triggers")
    }

    fn handle_file(&mut self, entry: &FileEntry, fullpath: &Path) {
        if !entry.filename().to_string_lossy().ends_with(".txt") {
            return;
        }

        let Some(block) = PdxFile::read(entry, fullpath) else { return };
        for (key, b) in block.iter_definitions_warn() {
            self.load_item(key, b);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Trigger {
    pub key: Token,
    block: Block,
    cache: MacroCache<ScopeContext>,
    scope_override: Option<Scopes>,
}

impl Trigger {
    pub fn new(key: Token, block: Block, scope_override: Option<Scopes>) -> Self {
        Self {
            key,
            block,
            cache: MacroCache::default(),
            scope_override,
        }
    }

    pub fn validate(&self, data: &Everything) {
        // We could let triggers get "naturally" validated by being called from other places,
        // but we want to also validate triggers that aren't called from anywhere yet.
        if self.block.source.is_none() {
            let mut sc = ScopeContext::new_unrooted(Scopes::all(), self.key.clone());
            sc.set_strict_scopes(false);
            if self.scope_override.is_some() {
                sc.set_no_warn(true);
            }
            self.validate_call(&self.key, data, &mut sc, Tooltipped::No);
        }
    }

    pub fn validate_call(
        &self,
        key: &Token,
        data: &Everything,
        sc: &mut ScopeContext,
        tooltipped: Tooltipped,
    ) {
        if !self.cached_compat(key, &[], tooltipped, sc) {
            let mut our_sc = ScopeContext::new_unrooted(Scopes::all(), self.key.clone());
            our_sc.set_strict_scopes(false);
            if self.scope_override.is_some() {
                our_sc.set_no_warn(true);
            }
            self.cache.insert(key, &[], tooltipped, our_sc.clone());
            validate_normal_trigger(&self.block, data, &mut our_sc, tooltipped);
            if let Some(scopes) = self.scope_override {
                our_sc = ScopeContext::new_unrooted(scopes, key.clone());
                our_sc.set_strict_scopes(false);
            }
            sc.expect_compatibility(&our_sc, key);
            self.cache.insert(key, &[], tooltipped, our_sc);
        }
    }

    pub fn macro_parms(&self) -> Vec<&str> {
        self.block.macro_parms()
    }

    pub fn cached_compat(
        &self,
        key: &Token,
        args: &[(&str, Token)],
        tooltipped: Tooltipped,
        sc: &mut ScopeContext,
    ) -> bool {
        self.cache.perform(key, args, tooltipped, |our_sc| {
            sc.expect_compatibility(our_sc, key);
        })
    }

    pub fn validate_macro_expansion(
        &self,
        key: &Token,
        args: Vec<(&str, Token)>,
        data: &Everything,
        sc: &mut ScopeContext,
        tooltipped: Tooltipped,
    ) {
        // Every invocation is treated as different even if the args are the same,
        // because we want to point to the correct one when reporting errors.
        if !self.cached_compat(key, &args, tooltipped, sc) {
            if let Some(block) = self.block.expand_macro(&args, key) {
                let mut our_sc = ScopeContext::new_unrooted(Scopes::all(), self.key.clone());
                our_sc.set_strict_scopes(false);
                if self.scope_override.is_some() {
                    our_sc.set_no_warn(true);
                }
                // Insert the dummy sc before continuing. That way, if we recurse, we'll hit
                // that dummy context instead of macro-expanding again.
                self.cache.insert(key, &args, tooltipped, our_sc.clone());
                validate_normal_trigger(&block, data, &mut our_sc, tooltipped);
                if let Some(scopes) = self.scope_override {
                    our_sc = ScopeContext::new_unrooted(scopes, key.clone());
                    our_sc.set_strict_scopes(false);
                }
                sc.expect_compatibility(&our_sc, key);
                self.cache.insert(key, &args, tooltipped, our_sc);
            }
        }
    }
}
