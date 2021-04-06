pub use std::{
    str::FromStr,
    boxed::Box,
    collections::BTreeMap,
    convert::Into,
    fmt::{/*Error, */ self},
    //error::Error;
    string::{String, ToString},
    sync::RwLock,
    vec::Vec,
};

pub(crate) use crate::{
    enums::{res_keeper, Resource},
    structs::{Localizer, StandartSerializer},
    FnUser,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LANGS : RwLock<Localizer> = RwLock::new(
    Localizer::create().user(&|x| {x.set_res(Resource::new_raw_res(res_keeper::new_file_from_path(None), StandartSerializer::new(), None)); x.get_mut_properties().set_current_lang(Some("en".to_string())); x.get_mut_res().res_into_string();})
    );
    //pub static ref LANGS : RwLock<Localizer> = RwLock::new(Localizer::create().user(&|x| {x.get_mut_properties().set_current_lang(Some("en".to_string())); }));
}
