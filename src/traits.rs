use crate::{enums::SerializeForm, prelude::String, structs::LangsDictionary, Error};

pub trait Res: Sync + Send {
    fn get_state(&self) -> &str;
    fn stringify(&mut self) -> Option<()>; //Result<(), LocError>;
    fn get_string(self) -> Option<String>;
    fn get_str(&self) -> Option<&str>;
    fn from_string(res: Option<String>) -> Self where Self: Sized;
    fn from_str(res: Option<&str>) -> Self where Self: Sized;
    fn set_string(&mut self, res: Option<String>);
    #[allow(patterns_in_fns_without_body)]
    fn user(mut self, f: &dyn Fn(&mut Self)) -> Self where Self: Sized;
}

pub trait Serializer: Sync + Send {
    fn serialize(
        &self,
        string: &str,
        serialize_format: &SerializeForm,
    ) -> Result<LangsDictionary, Error>; //Option<LangsDictionary>;//
}

pub trait FnUser
where
    Self: Sized,
{
    fn user(mut self, f: &dyn Fn(&mut Self)) -> Self {
        f(&mut self);
        self
    }
    fn im_user(self, f: &dyn Fn(&Self)) -> Self {
        f(&self);
        self
    }
}
