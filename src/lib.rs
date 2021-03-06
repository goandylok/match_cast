//! This is a minimal crate which implements match through different types.
//!
//! Usage:
//!
//! ```rust
//! #[macro_use]
//! extern crate match_cast;
//! use std::panic;
//!
//! fn main() {
//!     let res = panic::catch_unwind(|| {
//!         panic!("Oh no!");
//!     });
//!
//!     let any = res.unwrap_err();
//!
//!     let result = match_cast!( any {
//!         val as Option<u8> => {
//!             format!("Option<u8> = {:?}", val)
//!         },
//!         val as String => {
//!             format!("String = {:?}", val)
//!         },
//!         val as &'static str => {
//!             format!("&'static str = {:?}", val)
//!         },
//!     });
//!
//!     assert_eq!(result.unwrap(), "&'static str = \"Oh no!\"");
//! }
//! ```
//!
//! To use pattern there is `match_down` macro:
//!
//! ```rust
//! #[macro_use]
//! extern crate match_cast;
//! use std::any::Any;
//!
//! struct Bar {
//!     x: u8,
//! }
//!
//! struct Foo {
//!     x: u8,
//! }
//!
//! fn main() {
//!
//!     let any: Box<Any> = Box::new(Foo { x: 45 });
//!
//!     let result = match_down!( any {
//!         Bar { x } => { x },
//!         Foo { x } => { x },
//!     });
//!
//!     assert_eq!(result.unwrap(), 45);
//! }
//! ```


#[macro_export]
macro_rules! match_cast {
    ($any:ident { $( $bind:ident as $patt:ty => $body:block , )+ }) => {{
        let downcast = || {
            $(
            if let Some($bind) = $any.downcast_ref::<$patt>() {
                return Some($body);
            }
            )+
            None
        };
        downcast()
    }};
}

#[macro_export]
macro_rules! match_down {
    ($any:ident { $( $pt:pat => $body:block , )+ }) => {{
        let downcast = || {
            $(
            if let Some(&$pt) = $any.downcast_ref() {
                return Some($body);
            }
            )+
            None
        };
        downcast()
    }};
}

