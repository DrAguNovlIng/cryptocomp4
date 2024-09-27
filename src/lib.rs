use rand::Rng;
use std::ops::BitXor;

pub struct Alice {
    input_a: bool,
    input_b: bool,
    input_r: bool,
}

fn translate_input(input: u8) -> (bool, bool, bool) {
    let a = (input & 4) > 0;
    let r = (input & 1) > 0;
    let b = (input & 2) > 0;
    (a, b, r)
}

impl Alice {
    pub fn new() -> Self {
        Self { input_a: false, input_b: false, input_r: false }
    }

    pub fn choose(&mut self, input: u8) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
            let (a, b, r) = translate_input(input);
            self.input_a = a;
            self.input_b = b;
            self.input_r = r;
            todo!()
        }

    pub fn retrieve(&mut self, m2: (u8, u8, u8, u8, u8, u8, u8, u8)) -> u8 {
        todo!()
    }
}

pub struct Bob {
    input_a: bool,
    input_b: bool,
    input_r: bool,
}

impl Bob {
    pub fn new() -> Self {
        Self { input_a: false, input_b: false, input_r: false }
    }

    pub fn transfer(&mut self, input: u8, m1_from_alice: (u8, u8, u8, u8, u8, u8, u8, u8)) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        let (a, b, r) = translate_input(input);
        self.input_a = a;
        self.input_b = b;
        self.input_r = r;
        todo!()
    }
}

pub struct ElGamal {
    
}

impl ElGamal {
    //The constructure creates a group of prime order q and a generator g
    pub fn new() -> Self {
        todo!()
    }
}