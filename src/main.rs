use brainrust::{naive, optimized, StaticTape};
use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let buf = std::fs::read_to_string("prog.bf").unwrap();
    //let buf = {
    //    let mut buf = String::new();
    //    handle.read_to_string(&mut buf).unwrap();
    //    buf
    //};
    let mut tape = StaticTape::<102400>::new();
    //let mut out = io::stdout();
    let (parsed, has_input) = optimized::parse(buf.as_bytes());
    let mut out_buf = Vec::with_capacity(102400);
    let mut out = io::Cursor::new(&mut out_buf as &mut Vec<u8>);
    //naive::exec(buf.as_bytes(), &mut tape, &mut out, &mut handle);
    optimized::exec(&parsed, &mut tape, &mut out, &mut handle, has_input);
    io::stdout().write_all(&out_buf).unwrap();
}
