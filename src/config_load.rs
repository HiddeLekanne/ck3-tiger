use std::path::PathBuf;

use strum::IntoEnumIterator;

use crate::block::{Block, Comparator, Eq::*, BV};
use crate::report::{
    err, error, set_predicate, set_show_loaded_mods, set_show_vanilla, Confidence, ErrorKey,
    ErrorLoc, FilterRule, PointedMessage, Severity,
};
use crate::token::Token;

/// Checks for legacy ignore blocks (that no longer work) and report an error if they are present.
pub fn check_for_legacy_ignore(config: &Block) {
    // First, report errors if legacy ignore blocks are detected:
    let pointers: Vec<_> = config
        .iter_items()
        .filter_map(|(key, _, _)| key.as_ref())
        .filter(|key| key.as_str() == "ignore")
        .map(|key| PointedMessage::new(key.into_loc()))
        .collect();
    if !pointers.is_empty() {
        err(ErrorKey::Config)
            .strong()
            .msg("`ignore` is deprecated, consider using `filter` instead.")
            .info("Check out the filter.md guide on GitHub for tips on how to migrate.")
            .pointers(pointers)
            .push();
    }
}

pub fn load_filter(config: &Block) {
    assert_one_key("filter", config);
    if let Some(filter) = config.get_field_block("filter") {
        assert_one_key("trigger", filter);
        assert_one_key("show_vanilla", filter);
        assert_one_key("show_loaded_mods", filter);
        set_show_vanilla(filter.get_field_bool("show_vanilla").unwrap_or(false));
        set_show_loaded_mods(filter.get_field_bool("show_loaded_mods").unwrap_or(false));
        if let Some(trigger) = filter.get_field_block("trigger") {
            set_predicate(FilterRule::Conjunction(load_rules(trigger)));
        } else {
            set_predicate(FilterRule::default());
        }
    }
}

/// Load a vector of rules from the given block.
fn load_rules(block: &Block) -> Vec<FilterRule> {
    block
        .iter_items()
        .filter_map(|(key, operator, value)| load_rule(key, *operator, value))
        .collect()
}

/// Load a vector of rules from a value.
/// This first checks that the value is a block. If so, it loads a `Vec` of `FilterRule`s.
fn load_rules_from_value(value: &BV) -> Option<Vec<FilterRule>> {
    match value {
        BV::Block(block) => Some(load_rules(block)),
        BV::Value(_) => {
            error(value, ErrorKey::Config, "Expected a trigger block. Example usage: `AND = { }`");
            None
        }
    }
}

/// Load a single rule.
fn load_rule(key: &Option<Token>, comparator: Comparator, value: &BV) -> Option<FilterRule> {
    if key.is_none() {
        error(value, ErrorKey::Config, "Missing key. Loose values are not valid here.");
        return None;
    }
    let key = key.as_ref().expect("Should exist.");
    let key_str = key.as_str();
    if key_str != "severity"
        && key_str != "confidence"
        && !matches!(comparator, Comparator::Equals(Single))
    {
        error(
            key,
            ErrorKey::Config,
            &format!("Unexpected operator `{comparator}`, only `=` is valid here."),
        );
        return None;
    }
    match key_str {
        "severity" => load_rule_severity(comparator, value),
        "confidence" => load_rule_confidence(comparator, value),
        "key" => load_rule_key(value),
        "file" => load_rule_file(value),
        "always" => load_rule_always(value),
        "ignore_keys_in_files" => load_ignore_keys_in_files(value),
        "NOT" => load_not(value),
        "AND" => Some(FilterRule::Conjunction(load_rules_from_value(value)?)),
        "OR" => Some(FilterRule::Disjunction(load_rules_from_value(value)?)),
        "NAND" => Some(FilterRule::Negation(Box::new(FilterRule::Conjunction(
            load_rules_from_value(value)?,
        )))),
        "NOR" => Some(FilterRule::Negation(Box::new(FilterRule::Disjunction(
            load_rules_from_value(value)?,
        )))),
        _ => {
            error(key, ErrorKey::Config, "Unexpected key");
            None
        }
    }
}

/// This loads a NOT block.
/// In paradox script, NOT is actually an implicit NOR.
/// Load the children, if more than one exists, it returns a NOR block, otherwise a NOT.
fn load_not(value: &BV) -> Option<FilterRule> {
    let mut children = load_rules_from_value(value)?;
    if children.is_empty() {
        error(
            value,
            ErrorKey::Config,
            "This NOT block contains no valid triggers. It will be ignored.",
        );
        None
    } else if children.len() == 1 {
        Some(FilterRule::Negation(Box::new(children.remove(0))))
    } else {
        Some(FilterRule::Negation(Box::new(FilterRule::Disjunction(children))))
    }
}

fn load_rule_always(value: &BV) -> Option<FilterRule> {
    match value {
        BV::Block(_) => {
            error(
                value,
                ErrorKey::Config,
                "`always` can't open a block. Valid values are `yes` and `no`.",
            );
            None
        }
        BV::Value(token) => match token.as_str() {
            "yes" => Some(FilterRule::Tautology),
            "no" => Some(FilterRule::Contradiction),
            _ => {
                error(
                    value,
                    ErrorKey::Config,
                    "`always` value not recognised. Valid values are `yes` and `no`.",
                );
                None
            }
        },
    }
}

/// Loads the `ignore_keys_in_files` trigger.
/// This is syntactic sugar for a NAND wrapping an OR of keys and an OR of files.
fn load_ignore_keys_in_files(value: &BV) -> Option<FilterRule> {
    if let BV::Value(_) = value {
        err(ErrorKey::Config)
            .strong()
            .msg("This trigger should open a block.")
            .info("Usage: ignore_keys_in_files = { keys = {} files = {} }")
            .loc(value)
            .push();
        return None;
    }
    let block = value.expect_block().expect("Should be ok");

    let mut keys = None;
    let mut files = None;

    for (key, comparator, value) in block.iter_items() {
        if key.is_none() {
            err(ErrorKey::Config)
                .strong()
                .msg("Didn't expect a loose value here.")
                .info("Usage: ignore_keys_in_files = { keys = {} files = {} }")
                .loc(value)
                .push();
            return None;
        }
        let key = key.as_ref().expect("Should exist.");
        let key_str = key.as_str();
        if key_str != "keys" && key_str != "files" {
            err(ErrorKey::Config)
                .strong()
                .msg("This key isn't valid here.")
                .info("Usage: ignore_keys_in_files = { keys = {} files = {} }")
                .loc(value)
                .push();
            return None;
        }
        if !matches!(comparator, Comparator::Equals(_)) {
            err(ErrorKey::Config)
                .strong()
                .msg("Expected `=` here.")
                .info("Usage: ignore_keys_in_files = { keys = {} files = {} }")
                .loc(key)
                .push();
            return None;
        }
        if let BV::Value(_) = value {
            err(ErrorKey::Config)
                .strong()
                .msg("This should open a block.")
                .info("Usage: ignore_keys_in_files = { keys = {} files = {} }")
                .loc(value)
                .push();
            return None;
        }
        let array_block = value.expect_block().expect("Should be ok");
        if key_str == "keys" {
            keys = load_keys_array(array_block);
        }
        if key_str == "files" {
            files = load_files_array(array_block);
        }
    }
    if keys.is_none() {
        err(ErrorKey::Config)
            .strong()
            .msg("There are no valid keys. This `ignore_keys_in_files` trigger will be ignored.")
            .info(
                "Add at least one key. Example: ignore_keys_in_files = { keys = { unknown-field }",
            )
            .loc(block)
            .push();
        None
    } else if files.is_none() {
        err(ErrorKey::Config)
            .strong()
            .msg("There are no valid files. This `ignore_keys_in_files` trigger will be ignored.")
            .info("Add at least one file. Example: ignore_keys_in_files = { files = { common/ }")
            .loc(block)
            .push();
        None
    } else {
        Some(FilterRule::Negation(Box::new(FilterRule::Conjunction(vec![
            keys.expect("Should exist."),
            files.expect("Should exist."),
        ]))))
    }
}

fn load_keys_array(array_block: &Block) -> Option<FilterRule> {
    let keys: Vec<_> = array_block.iter_items()
        .filter_map(|(key, _, value)| {
            if key.is_some() {
                err(ErrorKey::Config).strong()
                    .msg("Expected a sequence of values here, no need to write `key = `")
                    .info("Example: keys = { history unknown-field field-missing }")
                    .loc(key.as_ref().expect("Should be present."))
                    .push();
                None
            } else if value.is_block() {
                err(ErrorKey::Config).strong()
                    .msg("Expected a sequence of values here, no blocks.")
                    .info("Example: keys = { history unknown-field field-missing }")
                    .loc(value.get_block().expect("Should be present."))
                    .push();
                None
            } else if let Ok(error_key) = value.get_value().expect("Should be present.").as_str().parse() {
                Some(FilterRule::Key(error_key))
            } else {
                err(ErrorKey::Config).strong()
                    .msg("Invalid key. In the output, keys are listed between parentheses on the first line of each report. For example, in `Warning(missing-item)`, the key is `missing-item`.")
                    // .msg(&format!("`{}` is not a valid key", value.get_value().expect("Should be present.").as_str()))
                    .loc(value.get_value().expect("Should be present."))
                    .push();
                None
            }
        }).collect();
    if keys.is_empty() {
        None
    } else {
        Some(FilterRule::Disjunction(keys))
    }
}
fn load_files_array(array_block: &Block) -> Option<FilterRule> {
    let files: Vec<_> = array_block
        .iter_items()
        .filter_map(|(key, _, value)| {
            if key.is_some() {
                err(ErrorKey::Config)
                    .strong()
                    .msg("Expected a sequence of values here, no need to write `file = `")
                    .info("Example: files = { common/ history/ }")
                    .loc(key.as_ref().expect("Should be present."))
                    .push();
                None
            } else if value.is_block() {
                err(ErrorKey::Config)
                    .strong()
                    .msg("Expected a sequence of values here, no blocks.")
                    .info("Example: files = { common/ history/ }")
                    .loc(value.get_block().expect("Should be present."))
                    .push();
                None
            } else {
                Some(FilterRule::File(PathBuf::from(
                    value.get_value().expect("Should be present.").as_str(),
                )))
            }
        })
        .collect();
    if files.is_empty() {
        None
    } else {
        Some(FilterRule::Disjunction(files))
    }
}

fn load_rule_severity(comparator: Comparator, value: &BV) -> Option<FilterRule> {
    match value {
        BV::Block(_) => {
            error(
                value,
                ErrorKey::Config,
                "`severity` can't open a block. Example usage: `severity >= Warning`",
            );
            None
        }
        BV::Value(token) => {
            if let Ok(severity) = token.as_str().parse() {
                Some(FilterRule::Severity(comparator, severity))
            } else {
                error(
                    token,
                    ErrorKey::Config,
                    &format!(
                        "Invalid Severity value. Valid values: {:?}",
                        Severity::iter().collect::<Vec<_>>()
                    ),
                );
                None
            }
        }
    }
}

fn load_rule_confidence(comparator: Comparator, value: &BV) -> Option<FilterRule> {
    match value {
        BV::Block(_) => {
            error(
                value,
                ErrorKey::Config,
                "`confidence` can't open a block. Example usage: `confidence >= Reasonable`",
            );
            None
        }
        BV::Value(token) => {
            if let Ok(confidence) = token.as_str().parse() {
                Some(FilterRule::Confidence(comparator, confidence))
            } else {
                error(
                    token,
                    ErrorKey::Config,
                    &format!(
                        "Invalid Confidence value. Valid values: {:?}",
                        Confidence::iter().collect::<Vec<_>>()
                    ),
                );
                None
            }
        }
    }
}

fn load_rule_key(value: &BV) -> Option<FilterRule> {
    match value {
        BV::Block(_) => {
            error(
                value,
                ErrorKey::Config,
                "`key` can't open a block. Example usage: `key = missing-item`",
            );
            None
        }
        BV::Value(token) => {
            if let Ok(error_key) = token.as_str().parse() {
                Some(FilterRule::Key(error_key))
            } else {
                error(
                    token,
                    ErrorKey::Config,
                    "Invalid key. In the output, keys are listed between parentheses on the first line of each report. For example, in `Warning(missing-item)`, the key is `missing-item`.",
                );
                None
            }
        }
    }
}

fn load_rule_file(value: &BV) -> Option<FilterRule> {
    match value {
        BV::Block(_) => {
            error(
                value,
                ErrorKey::Config,
                "`file` can't open a block. Example usage: `file = common/traits/00_traits.txt`",
            );
            None
        }
        BV::Value(token) => Some(FilterRule::File(PathBuf::from(token.as_str()))),
    }
}

/// Assert that the given key occurs at most once within the given block.
/// If the assertion fails, an error report will be created. No other action will be taken.
pub fn assert_one_key(assert_key: &str, block: &Block) {
    let keys: Vec<_> = block
        .iter_items()
        .filter_map(|item| {
            if let (Some(key), _, _) = item {
                if key.as_str() == assert_key {
                    Some(key)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    if keys.len() > 1 {
        let pointers = keys
            .iter()
            .enumerate()
            .map(|(index, key)| PointedMessage {
                location: key.into_loc(),
                length: 1,
                msg: Some((if index == 0 { "It occurs here" } else { "and here" }).to_owned()),
            })
            .collect();
        err(ErrorKey::Config)
            .strong()
            .msg(format!("Detected more than one `{assert_key}`: there can be only one here!"))
            .pointers(pointers)
            .push();
    }
}