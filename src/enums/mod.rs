pub mod resource;
pub use resource::Resource;

#[cfg(feature = "std")]
pub mod standart;
#[cfg(feature = "std")]
pub use standart::res_keeper;

pub mod serialize_form;
pub use serialize_form::SerializeForm;
