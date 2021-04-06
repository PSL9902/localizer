use crate::{
    enums::SerializeForm,
    prelude::{Box, String},
    structs::LangsDictionary,
    traits::{Res, Serializer},
    Error,
};

pub enum Resource {
    RawRes(Box<dyn Res>, Box<dyn Serializer>, SerializeForm),
    LD(LangsDictionary),
    None,
}

impl Resource {
    pub fn new_raw_res(
        res: impl Res + 'static,
        serializer: impl Serializer + 'static,
        serialize_format: Option<SerializeForm>,
    ) -> Self {
        Self::RawRes(
            Box::new(res),
            Box::new(serializer),
            serialize_format.unwrap_or(SerializeForm::Toml),
        )
    }
    pub fn new_langs_dictionary(res: LangsDictionary) -> Self {
        Self::LD(res)
    }
    pub fn new_none() -> Self {
        Self::None
    }

    pub fn set_res(&mut self, val: impl Res + 'static) {
        #[allow(clippy::single_match)]
        match self {
            Self::RawRes(ref mut res, ..) => {
                *res = Box::new(val);
            }
            _ => (),
        }
    }
    pub fn get_res(&self) -> Option<&dyn Res> {
        match self {
            Self::RawRes(ref res, ..) => Some(res.as_ref()),
            _ => None,
        }
    }
    pub fn get_mut_res(&mut self) -> Option<&mut dyn Res> {
        match self {
            Self::RawRes(ref mut res, ..) => Some(res.as_mut()),
            _ => None,
        }
    }

    pub fn set_serializer(&mut self, val: impl Serializer + 'static) {
        if let Self::RawRes(_, ref mut serializer, _) = self {
            *serializer = Box::new(val);
        }
        /*match self {
            Self::RawRes(_, ref mut serializer, _) => {
                *serializer = Box::new(val);
            }
            _ => (),
        }*/
    }
    pub fn get_serializer(&self) -> Option<&dyn Serializer> {
        match self {
            Self::RawRes(_, ref serializer, _) => Some(serializer.as_ref()),
            _ => None,
        }
    }
    pub fn get_mut_serializer(&mut self) -> Option<&mut dyn Serializer> {
        match self {
            Self::RawRes(_, ref mut serializer, _) => Some(serializer.as_mut()),
            _ => None,
        }
    }

    pub fn set_serialize_form(&mut self, val: Option<SerializeForm>) {
        if let Self::RawRes(_, _, ref mut serialize_format) = self {
            *serialize_format = val.unwrap_or(SerializeForm::Toml);
        }
        /*match self {
            Self::RawRes(_, _, ref mut serialize_format) => {
                *serialize_format = val.unwrap_or(SerializeForm::Toml);
            }
            _ => (),
        }*/
    }
    pub fn get_serialize_form(&self) -> Option<&SerializeForm> {
        match self {
            Self::RawRes(_, _, ref serialize_format) => Some(serialize_format),
            _ => None,
        }
    }
    pub fn get_mut_serialize_form(&mut self) -> Option<&mut SerializeForm> {
        match self {
            Self::RawRes(_, _, ref mut serialize_format) => Some(serialize_format),
            _ => None,
        }
    }

    pub fn get_ld(&self) -> Option<&LangsDictionary> {
        match self {
            Self::LD(ref ld) => Some(&ld),
            _ => None,
        }
    }

    pub fn res_into_ld(&mut self) -> Result<(), Error> {
        let a = match self {
            Self::RawRes(res, serializer, serialize_format) => {
                let s = if let Some(s) = res.get_str() {
                    s
                } else {
                    return Err("res.get_str() return None".into());
                };
                serializer.serialize(s, &serialize_format)?
            }
            _ => return Ok(()),
        };
        *self = Resource::LD(a);
        Ok(())
    }

    pub fn res_set_string(&mut self, string: Option<String>) {
        if let Self::RawRes(ref mut res, _, _) = self {
            res.as_mut().set_string(string);
        }
        /*match self {
            Self::RawRes(ref mut res, _, _) => res.as_mut().set_string(string),
            _ => (),
        }*/
    }

    pub fn res_into_string(&mut self) -> Option<()> {
        match self {
            Resource::RawRes(ref mut res, _, _) => res.as_mut().stringify(),
            _ => None,
        }
    }
}
