#![allow(dead_code)]

mod command;
mod errors;
mod oranda_impl;
mod repo;

pub use errors::*;
pub use oranda_impl::*;

/// Taken from cargo-insta to avoid copy-paste errors
///
/// Gets the ~name of the function running this macro
#[macro_export]
macro_rules! _function_name {
    () => {{
        fn f() {}
        fn type_name_of_val<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut name = type_name_of_val(f).strip_suffix("::f").unwrap_or("");
        while let Some(rest) = name.strip_suffix("::{{closure}}") {
            name = rest;
        }
        name.split_once("::")
            .map(|(_module, func)| func)
            .unwrap_or(name)
    }};
}
