#![allow(non_camel_case_types)]

use std::str::FromStr;
use strum_macros::{Display, EnumString};

use crate::token::Token;

// Validate the "code" blocks in localization files and in the gui files.
// The include/ files are converted from the game's data_type_* output files.

include!("include/datatypes.rs");

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, Debug)]
pub enum Args {
    NoArgs,
    Arg(Datatype),
    Arg2(Datatype, Datatype),
    Arg3(Datatype, Datatype, Datatype),
    Arg4(Datatype, Datatype, Datatype, Datatype),
    Arg5(Datatype, Datatype, Datatype, Datatype, Datatype),
}

impl Args {
    pub fn nargs(self) -> usize {
        match self {
            NoArgs => 0,
            Arg(_) => 1,
            Arg2(_, _) => 2,
            Arg3(_, _, _) => 3,
            Arg4(_, _, _, _) => 4,
            Arg5(_, _, _, _, _) => 5,
        }
    }
}

pub enum LookupResult {
    NotFound,
    WrongType,
    Found(Args, Datatype),
}

pub fn lookup_global_promote(lookup_name: &Token) -> Option<(Args, Datatype)> {
    if let Ok(idx) =
        GLOBAL_PROMOTES.binary_search_by_key(&lookup_name.as_str(), |(name, _, _)| name)
    {
        let (_name, args, rtype) = GLOBAL_PROMOTES[idx];
        return Some((args, rtype));
    }

    // Datatypes can be used directly as global promotes, taking their value from the gui context.
    if let Ok(dtype) = Datatype::from_str(lookup_name.as_str()) {
        return Some((Args::NoArgs, dtype));
    }

    None
}

pub fn lookup_global_function(lookup_name: &Token) -> Option<(Args, Datatype)> {
    if let Ok(idx) =
        GLOBAL_FUNCTIONS.binary_search_by_key(&lookup_name.as_str(), |(name, _, _)| name)
    {
        let (_name, args, rtype) = GLOBAL_FUNCTIONS[idx];
        return Some((args, rtype));
    }
    None
}

fn lookup_promote_or_function(
    lookup_name: &Token,
    ltype: Datatype,
    global: &[(&str, Datatype, Args, Datatype)],
) -> LookupResult {
    let lname = lookup_name.as_str();
    let start = global.partition_point(|(name, _, _, _)| name < &lname);
    let mut found_any = false;
    let mut possible_args = None;
    let mut possible_rtype = None;
    for i in start..global.len() {
        let (name, intype, args, rtype) = global[i];
        if lname != name {
            break;
        }
        found_any = true;
        if ltype == Datatype::Unknown {
            if possible_rtype.is_none() {
                possible_args = Some(args);
                possible_rtype = Some(rtype);
            } else if possible_rtype != Some(rtype) {
                possible_rtype = Some(Datatype::Unknown);
            }
        } else if ltype == intype {
            return LookupResult::Found(args, rtype);
        }
    }

    if found_any {
        if ltype == Datatype::Unknown {
            LookupResult::Found(possible_args.unwrap(), possible_rtype.unwrap())
        } else {
            LookupResult::WrongType
        }
    } else {
        LookupResult::NotFound
    }
}

pub fn lookup_promote(lookup_name: &Token, ltype: Datatype) -> LookupResult {
    lookup_promote_or_function(lookup_name, ltype, PROMOTES)
}

pub fn lookup_function(lookup_name: &Token, ltype: Datatype) -> LookupResult {
    lookup_promote_or_function(lookup_name, ltype, FUNCTIONS)
}

use Args::*;
use Datatype::*;

const GLOBAL_PROMOTES: &[(&str, Args, Datatype)] = include!("include/data_global_promotes.rs");

const GLOBAL_FUNCTIONS: &[(&str, Args, Datatype)] = include!("include/data_global_functions.rs");

const PROMOTES: &[(&str, Datatype, Args, Datatype)] = include!("include/data_promotes.rs");

const FUNCTIONS: &[(&str, Datatype, Args, Datatype)] = include!("include/data_functions.rs");
