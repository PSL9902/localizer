use crate::Error;
pub trait FormatterTrait: Sync + Send {
    //type FormArgsType;
    //fn get_formatted_string(&self, string: &str, args: /*&dyn*/impl FormatterArgsTrait) -> Result<String, Error> where Self: Sized;
    //fn get_formatted_string(&'a self, string: &str, args: &dyn FormatterArgsTrait) -> Result<String, Error> where Self: Sized;
    //fn get_formatted_string(&self, string: &str, args: Self::FormArgsType) -> Result<String, Error>;// where Self: Sized;
    fn get_formatted_string(
        &self,
        string: &str,
        args: &dyn FormatterArgsTrait,
    ) -> Result<String, Error>; // where Self: Sized;
}

pub trait FormatterArgsTrait /* : Sync + Send*//*<'a, FormArg>*/
{
    fn get_formatted_val(&self, string: &str) -> Result<String, Error>;
}
