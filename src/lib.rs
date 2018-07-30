//! An experimental implementation of [Rust RFC#2256 libsyntax2.0][rfc#2256].
//!
//! The intent is to be an IDE-ready parser, i.e. one that offers
//!
//! - easy and fast incremental re-parsing,
//! - graceful handling of errors, and
//! - maintains all information in the source file.
//!
//! For more information, see [the RFC][rfc#2265], or [the working draft][RFC.md].
//!
//!   [rfc#2256]: <https://github.com/rust-lang/rfcs/pull/2256>
//!   [RFC.md]: <https://github.com/matklad/libsyntax2/blob/master/docs/RFC.md>

#![forbid(
    missing_debug_implementations,
    unconditional_recursion,
    future_incompatible
)]
#![deny(bad_style, missing_docs)]
#![allow(missing_docs)]
//#![warn(unreachable_pub)] // rust-lang/rust#47816

extern crate text_unit;
extern crate unicode_xid;

mod lexer;
mod parser;
mod syntax_kinds;
mod yellow;
/// Utilities for simple uses of the parser.
pub mod utils;
pub mod ast;

pub use {
    lexer::{tokenize, Token},
    syntax_kinds::SyntaxKind,
    text_unit::{TextRange, TextUnit},
    yellow::{SyntaxNode, SyntaxNodeRef, TreeRoot, SyntaxRoot},
    ast::File,
};

pub(crate) use yellow::SyntaxError;

pub fn parse(text: String) -> SyntaxNode {
    let tokens = tokenize(&text);
    parser::parse::<yellow::GreenBuilder>(text, &tokens)
}

