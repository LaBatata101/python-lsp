//! This crate implements internal macros for the `ruff` library.

use crate::newtype_index::generate_newtype_index;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

mod newtype_index;

/// Derives a newtype wrapper that can be used as an index.
/// The wrapper can represent indices up to `u32::MAX - 1`.
///
/// The `u32::MAX - 1` is an optimization so that `Option<Index>` has the same size as `Index`.
///
/// Can store at most `u32::MAX - 1` values
///
/// ## Warning
///
/// Additional `derive` attributes must come AFTER this attribute:
///
/// Good:
///
/// ```rust
/// #[newtype_index]
/// #[derive(Ord, PartialOrd)]
/// struct MyIndex;
/// ```
#[proc_macro_attribute]
pub fn newtype_index(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);

    let output = match generate_newtype_index(item) {
        Ok(output) => output,
        Err(err) => err.to_compile_error(),
    };

    TokenStream::from(output)
}
