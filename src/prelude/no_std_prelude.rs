extern crate alloc;
pub use alloc::{
    boxed::Box,
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
pub use core::{
    str::FromStr,
    convert::Into,
    fmt::{/*Error, */ self},
};

pub(crate) use crate::{FnUser, Localizer};

use lazy_static::lazy_static;
use spin::rwlock::RwLock;

lazy_static! {
    pub static ref LANGS: RwLock<Localizer> = RwLock::new(Localizer::create().user(&|x| {
        x.get_mut_properties()
            .set_current_lang(Some("en".to_string()));
    }));
}
