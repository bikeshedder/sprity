use std::env;
use std::path::PathBuf;

use heck::ToUpperCamelCase;
use proc_macro_error::proc_macro_error;
use syn::{parse::Parse, parse_macro_input, LitStr};

extern crate proc_macro;

mod codegen;

struct AsepriteDeclaration {
    path: LitStr,
}

impl Parse for AsepriteDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(AsepriteDeclaration {
            path: input.parse()?,
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct AsepriteFile {
    name: String,
    path: PathBuf,
    tags: Vec<String>,
    layers: Vec<String>,
}

impl AsepriteFile {
    fn load(name: &str, path: &PathBuf) -> Self {
        Self {
            name: name.to_upper_camel_case(),
            path: path.to_owned(),
            tags: vec![],
            layers: vec![],
        }
    }
}

/// Include all `*.aseprite` files from the given directory.
///
/// **Important:** Unlike `include_str!` and `include_bytes!`
/// the given directory is looked up relative to the directory
/// containing the `Cargo.toml` (via the [`CARGO_MANIFEST_DIR`](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
/// environment variable). This directory is typically the
/// root directory of the crate.
///
/// As of Rust 1.59 there is no way _stable_ way of accessing the
/// directory of the caller. For that reason the `CARGO_MANIFEST_DIR`
/// is used instead which is the directory containing the `Cargo.toml`.
/// Future versions of this function might replace this by
/// `proc_macro::Span::source_file`.
#[proc_macro]
#[proc_macro_error]
pub fn aseprite_dir(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let AsepriteDeclaration { path } = parse_macro_input!(input as AsepriteDeclaration);
    let crate_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("No CARGO_MANIFEST_DIR in environment"),
    );
    proc_macro::TokenStream::from(codegen::aseprite_dir(
        crate_dir.join(path.value()).as_path(),
    ))
}
