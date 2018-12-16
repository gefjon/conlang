#![feature(arbitrary_self_types)]
#![feature(trace_macros)]
#![feature(try_trait)]

mod ast;
mod parser;
use std::io::{self, prelude::*};

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut reader = parser::ConlangReader::new(rl.iter("? ").map(Result::unwrap));
    rep(&mut reader, &mut io::stdout())
}

fn rep<I, O>(reader: &mut I, printer: &mut O)
where
    I: Iterator<Item = ast::Value>,
    O: Write,
{
    loop {
        match reader.next() {
            Some(v) => writeln!(printer, "{:?}", v).expect("rep: error on write"),
            None => return,
        }
    }
}
