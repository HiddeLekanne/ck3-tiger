use fnv::FnvHashMap;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::block::Block;
use crate::data::provinces::ProvId;
use crate::errorkey::ErrorKey;
use crate::errors::{error, warn};
use crate::everything::Everything;
use crate::fileset::{FileEntry, FileHandler};
use crate::helpers::dup_error;
use crate::pdxfile::PdxFile;
use crate::token::Token;

#[derive(Clone, Debug, Default)]
pub struct Titles {
    titles: FnvHashMap<String, Rc<Title>>,
    baronies: FnvHashMap<ProvId, Rc<Title>>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tier {
    Barony,
    County,
    Duchy,
    Kingdom,
    Empire,
}

impl TryFrom<&Token> for Tier {
    type Error = std::fmt::Error;
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        let s = value.as_str();
        if s.starts_with("b_") {
            Ok(Tier::Barony)
        } else if s.starts_with("c_") {
            Ok(Tier::County)
        } else if s.starts_with("d_") {
            Ok(Tier::Duchy)
        } else if s.starts_with("k_") {
            Ok(Tier::Kingdom)
        } else if s.starts_with("e_") {
            Ok(Tier::Empire)
        } else {
            Err(std::fmt::Error)
        }
    }
}

impl TryFrom<Token> for Tier {
    type Error = std::fmt::Error;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        Tier::try_from(&value)
    }
}

impl Display for Tier {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Tier::Barony => write!(f, "barony"),
            Tier::County => write!(f, "county"),
            Tier::Duchy => write!(f, "duchy"),
            Tier::Kingdom => write!(f, "kingdom"),
            Tier::Empire => write!(f, "empire"),
        }
    }
}

impl Titles {
    pub fn load_item(
        &mut self,
        key: Token,
        block: &Block,
        parent: Option<&str>,
        is_county_capital: bool,
    ) {
        if let Some(other) = self.titles.get(key.as_str()) {
            if other.key.loc.kind >= key.loc.kind {
                dup_error(&key, &other.key, "title");
            }
        }
        let title = Rc::new(Title::new(
            key.clone(),
            block.clone(),
            parent,
            is_county_capital,
        ));
        self.titles.insert(key.to_string(), title.clone());

        let parent_tier = Tier::try_from(&key).unwrap(); // guaranteed by caller
        if parent_tier == Tier::Barony {
            if let Some(provid) = block.get_field_integer("province") {
                if let Ok(provid) = ProvId::try_from(provid) {
                    self.baronies.insert(provid, title);
                } else {
                    error(
                        block.get_field_value("province").unwrap(),
                        ErrorKey::Validation,
                        "province id out of range",
                    );
                }
            } else {
                error(&key, ErrorKey::Validation, "barony without province id");
            }
        }

        let mut is_county_capital = parent_tier == Tier::County;
        for (k, v) in block.iter_pure_definitions() {
            if let Ok(tier) = Tier::try_from(k) {
                if tier >= parent_tier {
                    let msg = format!("can't put a {tier} inside a {parent_tier}");
                    error(k, ErrorKey::Validation, &msg);
                }
                self.load_item(k.clone(), v, Some(key.as_str()), is_county_capital);
                is_county_capital = false;
            }
        }
        if is_county_capital {
            error(key, ErrorKey::Validation, "county with no baronies!");
        }
    }

    pub fn exists(&self, key: &str) -> bool {
        self.titles.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<Rc<Title>> {
        self.titles.get(key).cloned()
    }

    pub fn validate(&self, data: &Everything) {
        for item in self.titles.values() {
            item.validate(data);
        }
    }

    pub fn capital_of(&self, prov: ProvId) -> Option<&str> {
        self.baronies.get(&prov).and_then(|b| b.capital_of())
    }
}

impl FileHandler for Titles {
    fn subpath(&self) -> PathBuf {
        PathBuf::from("common/landed_titles")
    }

    fn handle_file(&mut self, entry: &FileEntry, fullpath: &Path) {
        if !entry.filename().to_string_lossy().ends_with(".txt") {
            return;
        }

        let Some(block) = PdxFile::read(entry, fullpath) else { return };
        for (key, block) in block.iter_pure_definitions_warn() {
            if Tier::try_from(key).is_ok() {
                self.load_item(key.clone(), block, None, false);
            } else {
                warn(key, ErrorKey::Validation, "expected title");
            }
        }
    }

    fn finalize(&mut self) {
        for title in self.titles.values() {
            if let Some(capital) = title.block.get_field_value("capital") {
                if self.titles.get(capital.as_str()).is_none() {
                    error(
                        capital,
                        ErrorKey::Validation,
                        "capital is not defined as a title",
                    );
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Title {
    key: Token,
    block: Block,
    pub tier: Tier,
    pub parent: Option<String>,
    is_county_capital: bool, // for baronies
}

impl Title {
    pub fn new(key: Token, block: Block, parent: Option<&str>, is_county_capital: bool) -> Self {
        let tier = Tier::try_from(&key).unwrap(); // guaranteed by caller
        let parent = parent.map(String::from);
        Self {
            key,
            block,
            tier,
            parent,
            is_county_capital,
        }
    }

    pub fn validate(&self, data: &Everything) {
        // NOTE: There used to be a check that non-barony titles existed in the
        // title history, but that seems to be optional.
        data.localization.verify_exists(&self.key);
        // TODO: figure out when to recommend adding _adj or _pre titles
        // The _adj key is optional
        // The _pre key is optional

        if let Some(names) = self.block.get_field_block("cultural_names") {
            for (_, t) in names.get_assignments() {
                data.localization.verify_exists(t);
                // The _adj key is optional
            }
        }
    }

    fn capital_of(&self) -> Option<&str> {
        if self.is_county_capital {
            self.parent.as_deref()
        } else {
            None
        }
    }
}
