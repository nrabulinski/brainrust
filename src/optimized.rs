use super::Tape;
use std::io::{Read, Write};

#[derive(Debug)]
pub enum Ins {
    Shift(isize),
    Add(u8),
    Output,
    Input,
    Loop(Vec<Ins>),
}

pub fn parse(ins: &[u8]) -> (Vec<Ins>, bool) {
    let mut idx = 0;
    parse_ins(ins, &mut idx, false)
}

pub fn parse_ins(ins: &[u8], idx: &mut usize, is_loop: bool) -> (Vec<Ins>, bool) {
    let mut res = Vec::new();
    let mut has_input = false;
    match ins[*idx] {
        b'>' => res.push(Ins::Shift(1)),
        b'<' => res.push(Ins::Shift(-1)),
        b'+' => res.push(Ins::Add(1)),
        b'-' => res.push(Ins::Add(!0)),
        b'.' => res.push(Ins::Output),
        b',' => { res.push(Ins::Input); has_input = true; },
        b'[' => {
            *idx += 1;
            let loop_body = parse_ins(ins, idx, true);
            res.push(Ins::Loop(loop_body.0));
        }
        b']' if is_loop => return (res, false),
        b']' => panic!("Unexpected symbol @ {}: You're not in a loop", idx),
        _ => (),
    }
    *idx += 1;
    loop {
        if *idx == ins.len() {
            if is_loop {
                panic!("Syntax error @ {}: Unterminated loop", idx);
            } else {
                break;
            }
        }
        let b = ins[*idx];
        match b {
            b'>' => {
                if let Some(Ins::Shift(s)) = res.last_mut() {
                    *s += 1;
                } else {
                    res.push(Ins::Shift(1));
                }
            }
            b'<' => {
                if let Some(Ins::Shift(s)) = res.last_mut() {
                    *s -= 1;
                } else {
                    res.push(Ins::Shift(-1));
                }
            }
            b'+' => {
                if let Some(Ins::Add(s)) = res.last_mut() {
                    *s += 1;
                } else {
                    res.push(Ins::Add(1));
                }
            }
            b'-' => {
                if let Some(Ins::Add(s)) = res.last_mut() {
                    *s -= 1;
                } else {
                    res.push(Ins::Add(!0));
                }
            }
            b'.' => res.push(Ins::Output),
            b',' => { res.push(Ins::Input); has_input = true; },
            b'[' => {
                *idx += 1;
                let loop_body = parse_ins(ins, idx, true);
                res.push(Ins::Loop(loop_body.0));
            }
            b']' if is_loop => return (res, false),
            b']' => panic!("Unexpected symbol @ {}: You're not in a loop", idx),
            _ => (),
        }
        *idx += 1;
    }
    //res.into_iter().filter(|op| if let Ins::Add(0) = op { false } else { true }).collect()
    (res, has_input)
}

pub fn exec(ins: &[Ins], tape: &mut impl Tape, out: &mut impl Write, din: &mut impl Read, has_input: bool) {
    for b in ins {
        match b {
            Ins::Shift(s) => tape.add_ptr(*s),
            Ins::Add(c) => tape.add_dat(*c),
            Ins::Output => {
                write!(out, "{}", tape.curr() as char).unwrap();
                if has_input { out.flush().unwrap(); }
            }
            Ins::Input if has_input => {
                let ptr = tape.ptr();
                let r = ptr..ptr + 1;
                let b = &mut tape.tape_mut()[r];
                din.read_exact(b).unwrap();
            }
            Ins::Loop(body) => {
                while tape.curr() != 0 {
                    exec(body, tape, out, din, has_input);
                }
            }
            _ => (),
        }
    }
}
