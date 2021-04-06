use crate::prelude::{String, Vec};

#[cfg(feature = "format")]
use crate::{
    formatter::{FormatterArgsTrait, FormatterTrait},
    Error,
};
//pub use crate::constants::*;

//#[cfg(feature = "std")]
use crate::enums::Resource;

use crate::structs::Properties;

//#[derive(Debug, Clone, PartialEq)]
pub struct Localizer {
    pub res: Resource,
    pub properties: Properties,
    #[cfg(feature = "format")]
    pub formatter: Option<Box<dyn FormatterTrait>>,
}

impl Localizer {
    #[cfg(feature = "format")]
    pub fn new(
        res: Resource,
        properties: Properties,
        formatter: Option<Box<dyn FormatterTrait>>,
    ) -> Self {
        Self {
            res,
            properties,
            formatter,
        }
    }
    #[cfg(not(feature = "format"))]
    pub fn new(res: Resource, properties: Properties) -> Self {
        Self { res, properties }
    }

    #[cfg(feature = "format")]
    pub fn create() -> Self {
        Self {
            res: Resource::None,
            properties: Properties::new(),
            formatter: None,
        }
    }
    #[cfg(not(feature = "format"))]
    pub fn create() -> Self {
        Self {
            res: Resource::None,
            properties: Properties::new(),
        }
    }

    pub fn get_res(&self) -> &Resource {
        &self.res
    }
    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    #[cfg(feature = "format")]
    pub fn get_formatter(&self) -> &Option<Box<dyn FormatterTrait>> {
        &self.formatter
    }

    pub fn set_res(&mut self, val: Resource) {
        self.res = val;
    }

    pub fn set_properties(&mut self, val: Properties) {
        self.properties = val;
    }

    #[cfg(feature = "format")]
    pub fn set_formatter(&mut self, val: Option<impl FormatterTrait + 'static>) {
        self.formatter = match val {
            Some(x) => Some(Box::new(x)),
            _ => None,
        };
        //self.formatter = val.map(|val| Box::new(val));
    }

    pub fn get_mut_res(&mut self) -> &mut Resource {
        &mut self.res
    }
    pub fn get_mut_properties(&mut self) -> &mut Properties {
        &mut self.properties
    }

    #[cfg(feature = "format")]
    pub fn get_mut_formatter(&mut self) -> &mut Option<Box<dyn FormatterTrait>> {
        &mut self.formatter
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let current_lang = self
            .properties
            .get_current_lang()
            .as_ref()
            .unwrap_or(&self.properties.standart_lang);

        self.res
            .get_ld()
            .map(|ld| ld.get(key, &current_lang))
            .flatten()
    }
    #[cfg(feature = "format")]
    //pub fn get_f(&self, key : &str, args : &dyn FormatterArgsTrait) -> Option<String> {
    pub fn get_f(&self, key: &str, args: impl FormatterArgsTrait) -> Result<String, Error> {//Option<String> {//
        let res = self.get(key);
        match self.formatter {
            Some(ref form) => form.get_formatted_string(
                res.ok_or_else(|| format!("Key {} doesn`t exists", key).into())?
                    .as_ref(),
                &args,
            ), //.ok(),
            None => Err("formatter doesn`t exists".into()),
            //None => res,
        }
    }
    pub fn get1(&self, key: &str) -> Vec<String> {
        self.res
            .get_ld()
            .map(|ld| ld.get1(key))
            .unwrap_or_default()
            //.unwrap_or(Vec::new())
    }
}

impl Default for Localizer {
    fn default() -> Self {
        Self::create()
    }
}
