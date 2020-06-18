use super::Tape;
use std::io::prelude::*;

pub fn exec(ins: &[u8], tape: &mut impl Tape, out: &mut impl Write, din: &mut impl Read) {
    let mut idx = 0;
    loop {
        if idx == ins.len() {
            break;
        }
        let b = ins[idx];
        match b {
            b'>' => tape.inc_ptr(),
            b'<' => tape.dec_ptr(),
            b'+' => tape.inc(),
            b'-' => tape.dec(),
            b'.' => {
                write!(out, "{}", tape.curr() as char).unwrap();
                out.flush().unwrap();
            }
            b',' => {
                let ptr = tape.ptr();
                let r = ptr..ptr + 1;
                let b = &mut tape.tape_mut()[r];
                din.read_exact(b).unwrap();
            }
            b'[' => {
                let mut loop_count = 1;
                let mut idx_end = idx + 1;
                loop {
                    let b = ins[idx_end];
                    if b == b']' {
                        loop_count -= 1;
                        if loop_count == 0 {
                            break;
                        }
                    } else if b == b'[' {
                        loop_count += 1;
                    }
                    idx_end += 1;
                }
                let loop_body = &ins[idx + 1..idx_end];
                loop {
                    if tape.curr() == 0 {
                        idx = idx_end;
                        break;
                    }
                    exec(loop_body, tape, out, din);
                }
            }
            _ => (),
        }
        idx += 1;
    }
}
