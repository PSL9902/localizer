use crate::{Localizer, prelude::{LANGS, ToString, String, Vec}};

#[cfg(feature = "format")]
use crate::Error;

#[cfg(feature = "format")]
use crate::{formatter::FormatterArgsTrait};

///returns value from dictionary if it exists otherwise returns None
pub fn get_by_key(key: &str) -> Option<String> {
    #[cfg(feature = "std")]
    let lngs = LANGS.read().unwrap();
    #[cfg(not(feature = "std"))]
    let lngs = LANGS.read();

    lngs.get(key)
}
///returns value from dictionary if it exists otherwise returns key
pub fn get_by_key1(val0: String) -> String {
    get_by_key(&val0).unwrap_or(val0)
}
///returns value from dictionary if it exists otherwise returns second parameter
pub fn get_by_key2(key: &str, val2: String) -> String {
    get_by_key(key).unwrap_or(val2)
}

#[cfg(feature = "format")]
///returns value from dictionary if it exists otherwise returns second parameter
pub fn get_form_by_key(
    key: &str,
    val2: String,
    args: impl FormatterArgsTrait,
) -> String {
    #[cfg(feature = "std")]
    let lngs = LANGS.read().unwrap();
    #[cfg(not(feature = "std"))]
    let lngs = LANGS.read();

    lngs.get_f(key, args).unwrap_or(val2)
}

#[cfg(feature = "format")]
///returns value from dictionary if it exists otherwise returns None
pub fn get_form_by_key1(key: &str, args: impl FormatterArgsTrait) -> Result<String, Error> {
    //Option<String> {
    #[cfg(feature = "std")]
    let lngs = LANGS.read().unwrap();
    #[cfg(not(feature = "std"))]
    let lngs = LANGS.read();

    lngs.get_f(key, args)
}
///returns Vec of value from dictionary
pub fn get_by_key3(key: &str) -> Vec<String> {
    #[cfg(feature = "std")]
    let lngs = LANGS.read().unwrap();
    #[cfg(not(feature = "std"))]
    let lngs = LANGS.read();

    lngs.get1(key)
}

///changes language, dictionary, etc
pub fn change_localizer(f: &dyn Fn(&mut Localizer)) {
    #[cfg(feature = "std")]
    let mut lngs_mut = LANGS.write().unwrap();
    #[cfg(not(feature = "std"))]
    let mut lngs_mut = LANGS.write();

    f(&mut lngs_mut);
}

///changes string
pub fn set_loc_string(string: &str) {
    change_localizer(&|x| {
        x.get_mut_res().res_set_string(Some(string.to_string()));
    });
}

pub fn into_string() {
    change_localizer(&|x| {
        x.get_mut_res().res_into_string();
    });
}

pub fn set_localizer(new: Localizer) {
    #[cfg(feature = "std")]
    let mut lngs_mut = LANGS.write().unwrap();
    #[cfg(not(feature = "std"))]
    let mut lngs_mut = LANGS.write();

    *lngs_mut = new;
}
