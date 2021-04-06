use crate::{enums::SerializeForm, structs::LangsDictionary, traits::Serializer, Error, prelude::format};

impl Serializer for StandartSerializer {
    fn serialize(
        &self,
        string: &str,
        serialize_format: &SerializeForm,
    ) -> Result<LangsDictionary, Error> {//Option<LangsDictionary> {//
        match serialize_format {
            #[cfg(feature = "std")]
            SerializeForm::Toml => toml::from_str(string)
                .map_err(|x| format!("disp: |{}|, debug: |{:?}|", x, x).into()),
            SerializeForm::Json => serde_json::from_str(string)
                .map_err(|x| format!("disp: |{}|, debug: |{:?}|", x, x).into()),
            SerializeForm::Another(_) => Err("".into()), //{None}//
            
        }
    }
}

pub struct StandartSerializer;

impl StandartSerializer {
    pub fn new() -> Self {
        Self
    }
}
impl Default for StandartSerializer {
    fn default() -> Self {
        Self::new()
    }
}

/*
pub struct Serializer {
    langs_dictionary: Option<LangsDictionary>,
    serialize_form: SerializeForm
}
impl Serializer {
    pub fn new() -> Self {
        Self{langs_dictionary: None, serialize_form: SerializeForm::Toml}
    }
    pub fn create(&mut self, string : &str) -> Option<()> {
        let new = match self.serialize_form {
            SerializeForm::Toml => {toml::from_str(string).ok()?}
            SerializeForm::Json => {serde_json::from_str(string).ok()?}
            SerializeForm::Another(_) => {None?}
        };
        self.langs_dictionary = Some(new);
        Some(())
    }
    pub fn create_from_fn(&self, string : &str, f: &dyn Fn(&SerializeForm, &str) -> Option<LangsDictionary>) -> Option<LangsDictionary> {
        f(&self.serialize_form, string)
    }

    pub fn set_serialize_form_from_str(&mut self, a : &str) {
        //use Self::*;
        self.serialize_form = match a {
            "toml" => SerializeForm::Toml,
            "json" => SerializeForm::Json,
            s => SerializeForm::Another(s.to_string()),
        };
    }
    #[allow(dead_code, unreachable_patterns)]
    pub fn get_serialize_form_as_str<'a>(&'a self) -> &'a str {
        //use Self::*;
        match self.serialize_form {
            SerializeForm::Toml => "toml",
            SerializeForm::Json => "json",
            SerializeForm::Another( ref s ) => s,
            //_ => unimplemented!(),
        }
    }

    /*
    #[cfg(feature = "std")]
    pub fn get(&self, key : &str, current_lang : &Option<String>) -> Option<String> {
        match self {
            SerializeForm::Toml => {}
            SerializeForm::Json => {None}
            _ => panic!()
            //_ /*| SerializeForm::Json*/ => { unimplemented!() }
        }
    }
    #[cfg(feature = "std")]
    pub fn get1(&self, key : &str) -> Vec<String> {
        match self {
            SerializeForm::Toml => {}
            SerializeForm::Json => {}

        }
    }
    */
}
*/
