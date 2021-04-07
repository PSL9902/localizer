#![cfg_attr(not(feature = "std"), no_std)]
//!Simple library for localization.

//! [![Current Crates.io Version](https://img.shields.io/crates/v/localizer.svg)](https://crates.io/crates/localizer)
//! [![Document](https://img.shields.io/badge/doc-localizer-green.svg)](https://docs.rs/localizer)
//! # no unsafe code
//! # Example:
//! ```
//! use localizer::*;
//! const langs_file : &str = "
//! [langs.ru]
//! \"ex1\" = \"ПРН{}\"
//!
//! [langs.en]
//! \"ex1\" = \"IKA{}\"";
//! localizer::set_loc_string(langs_file);
//! //localizator::set_localizer(Localizer::create().file(None).current_lang(Some("ru".to_string())));
//! localizer::change_localizer(&|x|{x.set_current_lang(Some("ru".to_string()));});
//! println!("{:?}", localizer::get_by_key(&"ex1"));
//! let loc = Localizer::create().file(None).current_lang(Some("ru".to_string()));
//! println!("{:?}", loc.get(&"ex1"));
//! ```
//! 
//! 
//! # Further examples:
//! > cargo run --example %example%
//! ## 1:
//! [ex1](https://github.com/PSL9902/localizer/blob/master/examples/ex1.rs)
//! ## 2:
//! [ex2](https://github.com/PSL9902/localizer/blob/master/examples/ex2.rs)
//! ## 3:
//! [Tic_Tac_Toe game1](https://github.com/PSL9902/localizer/blob/master/examples/tic-tac-toe.rs)
//! [Tic_Tac_Toe game2](https://github.com/PSL9902/rust_Tic_Tac_Toe/tree/master)
//! # No-Std:
//! > feature = "no_std"
//! ## Cargo.toml:
//! > localizer = {version = ..., default-features = false, features = ["no_std"]}



pub mod constants;

pub mod structs;
pub use structs::localizer::Localizer;

pub mod enums;

pub mod prelude;
pub use prelude::*;

pub mod error;
pub use error::Error;

#[cfg(feature = "format")]
pub mod formatter;
//#[cfg(feature = "format")]
//pub use form::*;

pub mod traits;
pub use traits::{FnUser};

impl<T> traits::FnUser for T {}

//#[cfg(not(feature = "no_std"))]
//use enums::standart::res_keeper;
