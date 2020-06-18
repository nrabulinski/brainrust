#![feature(const_generics)]
pub mod naive;
pub mod optimized;

pub trait Tape {
    fn add_ptr(&mut self, size: isize) { *self.ptr_mut() += size as usize }
    fn inc_ptr(&mut self) { self.add_ptr(1) }
    fn dec_ptr(&mut self) { self.add_ptr(-1) }
    fn add_dat(&mut self, c: u8) { *self.curr_mut() += c }
    fn inc(&mut self) { self.add_dat(1) }
    fn dec(&mut self) { self.add_dat(!0) }
    fn curr(&self) -> u8 { self.tape()[self.ptr()] }
    fn curr_mut(&mut self) -> &mut u8 {
        let ptr = self.ptr();
        self.tape_mut().get_mut(ptr).unwrap()
    }
    fn ptr_mut(&mut self) -> &mut usize;
    fn ptr(&self) -> usize;
    fn tape(&self) -> &[u8];
    fn tape_mut(&mut self) -> &mut [u8];
}

pub struct StaticTape<const N: usize> {
    tape: [u8; N],
    ptr: usize,
}

impl<const N: usize> Tape for StaticTape<N> {
    fn ptr_mut(&mut self) -> &mut usize { &mut self.ptr }

    fn ptr(&self) -> usize { self.ptr }

    fn tape(&self) -> &[u8] { &self.tape }

    fn tape_mut(&mut self) -> &mut [u8] { &mut self.tape }
}

impl<const N: usize> StaticTape<N> {
    pub fn new() -> Self {
        StaticTape {
            tape: [0u8; N],
            ptr: 0,
        }
    }
}

pub struct DynamicTape {
    tape: Vec<u8>,
    ptr: usize,
}

impl Tape for DynamicTape {
    fn ptr_mut(&mut self) -> &mut usize { &mut self.ptr }
    fn ptr(&self) -> usize { self.ptr }
    fn tape(&self) -> &[u8] { &self.tape }
    fn tape_mut(&mut self) -> &mut [u8] { &mut self.tape }
    fn add_ptr(&mut self, size: isize) {
        self.ptr += size as usize;
        if self.ptr >= self.tape.len() {
            self.tape.resize(self.ptr, 0);
        }
    }
}
