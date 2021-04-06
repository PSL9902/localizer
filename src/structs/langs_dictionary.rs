use crate::prelude::{BTreeMap, String, ToString, Vec};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LangsDictionary {
    langs: BTreeMap<String, BTreeMap<String, String>>,
}

impl LangsDictionary {
    pub fn get(&self, key: &str, current_lang: &str) -> Option<String> {
        self.langs
            .get(current_lang)?
            .get(key)
            .map(|x| x.to_string())
    }
    pub fn get1(&self, key: &str) -> Vec<String> {
        self.langs
            .iter()
            .filter_map(|(_, val)| val.get(key).map(|val| val.into()))
            .collect()
    }
    pub fn get_langs(&self) -> &BTreeMap<String, BTreeMap<String, String>> {
        &self.langs
    }
}
