use rand::Rng;
use core::panic;
use std::{io::empty, ops::BitXor, ptr::null};

// STRUCTS
#[derive(Debug, Copy, Clone)]
pub struct SecretSharingPair {
    alice: u8,
    bob: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct RandomnessTriple {
    v: u8,
    u: u8,
    w: u8
}

pub struct TrustedDealer {
    randomness_for_alice: [RandomnessTriple; 5],
    randomness_for_bob: [RandomnessTriple; 5]
}

pub struct Alice {
    z_own_share: u8,
    z_their_share: u8,
    z_temp_own_share: u8,
    z_temp_their_share: u8,
    has_output: bool,
    input_alice_a: u8,
    input_alice_b: u8,
    input_alice_r: u8,
    randomness_from_dealer: [RandomnessTriple; 5],
    progress: u8,
    input_alice_a_own_share: u8,
    input_alice_b_own_share: u8,
    input_alice_r_own_share: u8,
    input_alice_a_their_share: u8,
    input_alice_b_their_share: u8,
    input_alice_r_their_share: u8,
    input_bob_a_own_share: u8,
    input_bob_b_own_share: u8,
    input_bob_r_own_share: u8,
    e_own_share: u8,
    d_own_share: u8,
    e_their_share: u8,
    d_their_share: u8,
    d: u8,
    e: u8,
}

pub struct Bob {
    input_bob_a: u8,
    input_bob_b: u8,
    input_bob_r: u8,
    z_own_share: u8,
    z_their_share: u8,
    z_temp_own_share: u8,
    z_temp_their_share: u8,
    progress: u8,
    input_bob_a_own_share: u8,
    input_bob_b_own_share: u8,
    input_bob_r_own_share: u8,
    input_bob_a_their_share: u8,
    input_bob_b_their_share: u8,
    input_bob_r_their_share: u8,
    input_alice_a_own_share: u8,
    input_alice_b_own_share: u8,
    input_alice_r_own_share: u8,
    e_own_share: u8,
    d_own_share: u8,
    e_their_share: u8,
    d_their_share: u8,
    d: u8,
    e: u8,
    randomness_from_dealer: [RandomnessTriple; 5],
}

// IMPLEMENTATIONS
impl SecretSharingPair {
    pub fn new(value: u8) -> SecretSharingPair {
        let mut rng = rand::thread_rng();
        let alice_share = rng.gen_range(0..=1);
        let bob_share = value.bitxor(alice_share);
        SecretSharingPair {alice: alice_share, bob: bob_share}
    }

    pub fn value(&self) -> u8 {
        return self.alice.bitxor(self.bob)
    }

}

impl TrustedDealer {
    pub fn new() -> TrustedDealer {
        TrustedDealer {
            randomness_for_alice: [RandomnessTriple{u: 0, v: 0, w:0}; 5],
            randomness_for_bob: [RandomnessTriple{u: 0, v: 0, w:0}; 5],
        }
    }

    //Generate all random values to be used during the whole protocol (we need 5 ANDS)
    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..5 {
            let u = rng.gen_range(0..=1);
            let v = rng.gen_range(0..=1);
            let w = u * v;
            let u_secret = SecretSharingPair::new(u);
            let v_secret = SecretSharingPair::new(v);
            let w_secret = SecretSharingPair::new(w);

            self.randomness_for_alice[i] = RandomnessTriple{u: u_secret.alice, v: v_secret.alice, w: w_secret.alice};
            self.randomness_for_bob[i] = RandomnessTriple{u: u_secret.bob, v: v_secret.bob, w: w_secret.bob};
        }
    }

    // Output (r, M_a) to Alice
    pub fn rand_a(&self) -> [RandomnessTriple; 5] {
        self.randomness_for_alice
    }

    // Output (s, M_b) to Bob
    pub fn rand_b(&self) -> [RandomnessTriple; 5] {
        self.randomness_for_bob
    }
}

impl Alice {
    pub fn new() -> Alice {
        Alice {
            has_output: false,
            z_own_share: 0,
            z_their_share: 0,
            z_temp_own_share: 0,
            z_temp_their_share: 0,
            input_alice_a: 0,
            input_alice_b: 0,
            input_alice_r: 0,
            progress: 0,
            input_alice_a_own_share: 0,
            input_alice_b_own_share: 0,
            input_alice_r_own_share: 0,
            input_alice_a_their_share: 0,
            input_alice_b_their_share: 0,
            input_alice_r_their_share: 0,
            input_bob_a_own_share: 0,
            input_bob_b_own_share: 0,
            input_bob_r_own_share: 0,
            d_own_share: 0,
            e_own_share: 0,
            d_their_share: 0,
            e_their_share: 0,
            d: 0,
            e: 0,
            randomness_from_dealer: [RandomnessTriple {u: 0, v: 0, w: 0}; 5],
        }
    }

    pub fn has_output(&self) -> bool {
        self.has_output
    }

    pub fn init(&mut self, x: u8, randoms: [RandomnessTriple; 5]) {
        let mut rng = rand::thread_rng();
        self.input_alice_a = x & 1;
        self.input_alice_b = x & 2;
        self.input_alice_r = x & 4;
        self.randomness_from_dealer = randoms;
        self.input_alice_a_their_share = rng.gen_range(0..=1);
        self.input_alice_a_own_share = self.input_alice_a.bitxor(self.input_alice_a_their_share);
        self.input_alice_b_their_share = rng.gen_range(0..=1);
        self.input_alice_b_own_share = self.input_alice_b.bitxor(self.input_alice_b_their_share);
        self.input_alice_r_their_share = rng.gen_range(0..=1);
        self.input_alice_r_own_share = self.input_alice_r.bitxor(self.input_alice_r_their_share);
    }

    pub fn send(&mut self) -> u8 {
        self.progress += 1;
        match self.progress {
            1 => {
                let randomness = self.randomness_from_dealer[0];
                let temp = 1.bitxor(self.input_alice_a_own_share);
                self.d_own_share = temp.bitxor(randomness.u);

                self.e_own_share = self.input_bob_a_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            2 => {
                self.e_own_share
            }
            3 => {
                let randomness = self.randomness_from_dealer[1];
                let temp = 1.bitxor(self.input_alice_b_own_share);
                self.d_own_share = temp.bitxor(randomness.u);

                self.e_own_share = self.input_bob_b_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            4 => {
                self.e_own_share
            }
            5 => {
                let randomness = self.randomness_from_dealer[2];
                self.d_own_share = self.z_own_share.bitxor(randomness.u);

                self.e_own_share = self.z_temp_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            6 => {
                self.e_own_share
            }
            7 => {
                let randomness = self.randomness_from_dealer[3];

                let temp = 1.bitxor(self.input_alice_r_own_share);
                self.d_own_share = temp.bitxor(randomness.u);

                self.e_own_share = self.input_bob_r_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            8 => {
                self.e_own_share
            }
            9 => {
                let randomness = self.randomness_from_dealer[4];
                self.d_own_share = self.z_own_share.bitxor(randomness.u);

                self.e_own_share = self.z_temp_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            10 => {
                self.e_own_share
            }
            11 => {
                0 //Dummy value
            }
            _ => {
                panic!("Nothing to send!")
            }
        }
    }
    pub fn send_input_share(&self) -> (u8, u8, u8) {
        (self.input_alice_a_their_share, self.input_alice_b_their_share, self.input_alice_r_their_share)
    }

    pub fn receive_input_share(&mut self, shares: (u8, u8, u8)) {
        self.input_bob_a_own_share = shares.0;
        self.input_bob_b_own_share = shares.1;
        self.input_bob_r_own_share = shares.2;
    }

    pub fn receive(&mut self, input: u8) {
        match self.progress + 1 {
            1 => {
                self.d = self.d_own_share.bitxor(input);
            }
            2 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[0].w;
                let ex_term = self.e * self.input_alice_a_own_share;
                let dy_term = self.d * self.input_bob_a_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = 1.bitxor(w_term).bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }

            3 => {
                self.d = self.d_own_share.bitxor(input);
            }
            4 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[1].w;
                let ex_term = self.e * self.input_alice_b_own_share;
                let dy_term = self.d * self.input_bob_b_own_share;
                let ed_term = self.e * self.d;
                self.z_temp_own_share = 1.bitxor(w_term).bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            5 => {
                self.d = self.d_own_share.bitxor(input);
            }
            6 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[2].w;
                let ex_term = self.e * self.z_own_share;
                let dy_term = self.d * self.z_temp_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            7 => {
                self.d = self.d_own_share.bitxor(input);
            }
            8 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[3].w;
                let ex_term = self.e * self.input_alice_r_own_share;
                let dy_term = self.d * self.input_bob_r_own_share;
                let ed_term = self.e * self.d;
                self.z_temp_own_share = 1.bitxor(w_term).bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            9 => {
                self.d = self.d_own_share.bitxor(input);
            }
            10 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[4].w;
                let ex_term = self.e * self.z_own_share;
                let dy_term = self.d * self.z_temp_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            11 => {
                self.z_their_share = input;
                self.has_output = true;
            }
            _ => {
                //do nothing
            }
        }
    }

    pub fn output(&self) -> u8 {
        self.z_own_share.bitxor(self.z_their_share)
    }
}

impl Bob {
    pub fn new() -> Bob {
        Bob {
            input_bob_a: 0,
            input_bob_b: 0,
            input_bob_r: 0,
            z_own_share: 0,
            z_their_share: 0,
            z_temp_own_share: 0,
            z_temp_their_share: 0,
            progress: 0,
            input_bob_a_own_share: 0,
            input_bob_b_own_share: 0,
            input_bob_r_own_share: 0,
            input_bob_a_their_share: 0,
            input_bob_b_their_share: 0,
            input_bob_r_their_share: 0,
            input_alice_a_own_share: 0,
            input_alice_b_own_share: 0,
            input_alice_r_own_share: 0,
            d: 0,
            e: 0,
            d_own_share: 0,
            e_own_share: 0,
            d_their_share: 0,
            e_their_share: 0,
            randomness_from_dealer: [RandomnessTriple {u: 0, v: 0, w: 0}; 5],
        }
    }

    pub fn init(&mut self, y: u8, randoms: [RandomnessTriple; 5]) {
        let mut rng = rand::thread_rng();
        self.input_bob_a = y & 1;
        self.input_bob_b = y & 2;
        self.input_bob_r = y & 4;
        self.randomness_from_dealer = randoms;
        self.input_bob_a_their_share = rng.gen_range(0..=1);
        self.input_bob_a_own_share = self.input_bob_a.bitxor(self.input_bob_a_their_share);
        self.input_bob_b_their_share = rng.gen_range(0..=1);
        self.input_bob_b_own_share = self.input_bob_b.bitxor(self.input_bob_b_their_share);
        self.input_bob_r_their_share = rng.gen_range(0..=1);
        self.input_bob_r_own_share = self.input_bob_r.bitxor(self.input_bob_r_their_share);
    }
    pub fn send_input_share(&self) -> (u8, u8, u8) {
        (self.input_bob_a_their_share, self.input_bob_b_their_share, self.input_bob_r_their_share)
    }

    pub fn receive_input_share(&mut self, shares: (u8, u8, u8)) {
        self.input_alice_a_own_share = shares.0;
        self.input_alice_b_own_share = shares.1;
        self.input_alice_r_own_share = shares.2;
    }

    pub fn send(&mut self) -> u8 {
        self.progress += 1;
        match self.progress {
            1 => {
                let randomness = self.randomness_from_dealer[0];
                self.d_own_share = self.input_alice_a_own_share.bitxor(randomness.u);

                self.e_own_share = self.input_bob_a_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            2 => {
                self.e_own_share
            }
            3 => {
                let randomness = self.randomness_from_dealer[1];
                self.d_own_share = self.input_alice_b_own_share.bitxor(randomness.u);

                self.e_own_share = self.input_bob_b_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            4 => {
                self.e_own_share
            }
            5 => {
                let randomness = self.randomness_from_dealer[2];
                self.d_own_share = self.z_own_share.bitxor(randomness.u);

                self.e_own_share = self.z_temp_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            6 => {
                self.e_own_share
            }
            7 => {
                let randomness = self.randomness_from_dealer[3];
                self.d_own_share = self.input_alice_r_own_share.bitxor(randomness.u);

                self.e_own_share = self.input_bob_r_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            8 => {
                self.e_own_share
            }
            9 => {
                let randomness = self.randomness_from_dealer[4];
                self.d_own_share = self.z_own_share.bitxor(randomness.u);

                self.e_own_share = self.z_temp_own_share.bitxor(randomness.v);

                self.d_own_share
            }
            10 => {
                self.e_own_share
            }
            11 => {
                self.z_own_share
            }
            _ => {
                panic!("Nothing to send!")
            }
        }
    }

    pub fn receive(&mut self, input: u8) {
        match self.progress + 1 {
            1 => {
                self.d = self.d_own_share.bitxor(input);
            }
            2 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[0].w;
                let ex_term = self.e * self.input_alice_a_own_share;
                let dy_term = self.d * self.input_bob_a_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }

            3 => {
                self.d = self.d_own_share.bitxor(input);
            }
            4 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[1].w;
                let ex_term = self.e * self.input_alice_b_own_share;
                let dy_term = self.d * self.input_bob_b_own_share;
                let ed_term = self.e * self.d;
                self.z_temp_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            5 => {
                self.d = self.d_own_share.bitxor(input);
            }
            6 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[2].w;
                let ex_term = self.e * self.z_own_share;
                let dy_term = self.d * self.z_temp_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            7 => {
                self.d = self.d_own_share.bitxor(input);
            }
            8 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[3].w;
                let ex_term = self.e * self.input_alice_r_own_share;
                let dy_term = self.d * self.input_bob_r_own_share;
                let ed_term = self.e * self.d;
                self.z_temp_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            9 => {
                self.d = self.d_own_share.bitxor(input);
            }
            10 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randomness_from_dealer[4].w;
                let ex_term = self.e * self.z_own_share;
                let dy_term = self.d * self.z_temp_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            _ => {
                //do nothing
            }
        }
    }
}
