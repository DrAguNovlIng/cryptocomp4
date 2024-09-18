use rand::Rng;
use core::panic;
use std::ops::BitXor;

// STRUCTS
pub struct TrustedDealer {
    randoms: [[u8; 6]; 5],
}

pub struct Alice {
    z_own_share: u8,
    z_their_share: u8,
    z_temp_own_share: u8,
    z_temp_their_share: u8,
    has_output: bool,
    x_a: u8,
    x_b: u8,
    x_r: u8,
    randoms: [[u8; 3]; 5],
    progress: u8,
    x_a_own_share: u8,
    x_b_own_share: u8,
    x_r_own_share: u8,
    x_a_their_share: u8,
    x_b_their_share: u8,
    x_r_their_share: u8,
    y_a_own_share: u8,
    y_b_own_share: u8,
    y_r_own_share: u8,
    e_own_share: u8,
    d_own_share: u8,
    e_their_share: u8,
    d_their_share: u8,
    d: u8,
    e: u8,
}

pub struct Bob {
    y_a: u8,
    y_b: u8,
    y_r: u8,
    z_own_share: u8,
    z_their_share: u8,
    z_temp_own_share: u8,
    z_temp_their_share: u8,
    randoms: [[u8; 3]; 5],
    progress: u8,
    y_a_own_share: u8,
    y_b_own_share: u8,
    y_r_own_share: u8,
    y_a_their_share: u8,
    y_b_their_share: u8,
    y_r_their_share: u8,
    x_a_own_share: u8,
    x_b_own_share: u8,
    x_r_own_share: u8,
    e_own_share: u8,
    d_own_share: u8,
    e_their_share: u8,
    d_their_share: u8,
    d: u8,
    e: u8,
}

// IMPLEMENTATIONS
impl TrustedDealer {
    pub fn new() -> TrustedDealer {
        TrustedDealer {
            randoms: [[0; 6]; 5],
        }
    }

    //Generate all random values to be used during the whole protocol (we need 5 ANDS)
    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..5 {
            let u = rng.gen_range(0..=1);
            let v = rng.gen_range(0..=1);
            let w = u * v;
            let u_a = rng.gen_range(0..=1);
            let u_b = u_a.bitxor(u);
            let v_a = rng.gen_range(0..=1);
            let v_b = v_a.bitxor(v);
            let w_a = rng.gen_range(0..=1);
            let w_b = w_a.bitxor(w);
            self.randoms[i] = [u_a, u_b, v_a, v_b, w_a, w_b];
        }
    }

    // Output (r, M_a) to Alice
    pub fn rand_a(&self) -> [[u8; 3]; 5] {
        let mut result: [[u8; 3]; 5] = [[0; 3]; 5];
        for i in 0..5 {
            result[i][0] = self.randoms[i][0];
            result[i][2] = self.randoms[i][1];
            result[i][4] = self.randoms[i][2];
        }
        result
    }

    // Output (s, M_b) to Bob
    pub fn rand_b(&self) -> [[u8; 3]; 5] {
        let mut result: [[u8; 3]; 5] = [[0; 3]; 5];
        for i in 0..5 {
            result[i][1] = self.randoms[i][0];
            result[i][3] = self.randoms[i][1];
            result[i][5] = self.randoms[i][2];
        }
        result
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
            x_a: 0,
            x_b: 0,
            x_r: 0,
            randoms: [[0; 3]; 5],
            progress: 0,
            x_a_own_share: 0,
            x_b_own_share: 0,
            x_r_own_share: 0,
            x_a_their_share: 0,
            x_b_their_share: 0,
            x_r_their_share: 0,
            y_a_own_share: 0,
            y_b_own_share: 0,
            y_r_own_share: 0,
            d_own_share: 0,
            e_own_share: 0,
            d_their_share: 0,
            e_their_share: 0,
            d: 0,
            e: 0,
        }
    }

    pub fn init(&mut self, x: u8, randoms: [[u8; 3]; 5]) {
        let mut rng = rand::thread_rng();
        self.x_a = x & 1;
        self.x_b = x & 2;
        self.x_r = x & 4;
        self.randoms = randoms;
        self.x_a_their_share = rng.gen_range(0..=1);
        self.x_a_own_share = self.x_a.bitxor(self.x_a_their_share);
        self.x_b_their_share = rng.gen_range(0..=1);
        self.x_b_own_share = self.x_b.bitxor(self.x_b_their_share);
        self.x_r_their_share = rng.gen_range(0..=1);
        self.x_r_own_share = self.x_r.bitxor(self.x_r_their_share);
    }

    pub fn send(&mut self) -> u8 {
        self.progress += 1;
        match self.progress {
            1 => {
                let temp = 1.bitxor(self.x_a_own_share);
                self.d_own_share = temp.bitxor(self.randoms[0][0]);

                self.e_own_share = self.y_a_own_share.bitxor(self.randoms[0][1]);

                self.d_own_share
            }
            2 => {
                self.e_own_share
            }
            3 => {
                let temp = 1.bitxor(self.x_b_own_share);
                self.d_own_share = temp.bitxor(self.randoms[1][0]);

                self.e_own_share = self.y_b_own_share.bitxor(self.randoms[1][1]);

                self.d_own_share
            }
            4 => {
                self.e_own_share
            }
            5 => {
                self.d_own_share = self.z_own_share.bitxor(self.randoms[2][0]);

                self.e_own_share = self.z_temp_own_share.bitxor(self.randoms[2][1]);

                self.d_own_share
            }
            6 => {
                self.e_own_share
            }
            7 => {
                let temp = 1.bitxor(self.x_r_own_share);
                self.d_own_share = temp.bitxor(self.randoms[3][0]);

                self.e_own_share = self.y_r_own_share.bitxor(self.randoms[3][1]);

                self.d_own_share
            }
            8 => {
                self.e_own_share
            }
            9 => {
                self.d_own_share = self.z_own_share.bitxor(self.randoms[4][0]);

                self.e_own_share = self.z_temp_own_share.bitxor(self.randoms[4][1]);

                self.d_own_share
            }
            10 => {
                self.e_own_share
            }
            _ => {
                panic!("More than 5 iterations of send!")
            }
        }
    }
    pub fn send_input_share(&self) -> (u8, u8, u8) {
        (self.x_a_their_share, self.x_b_their_share, self.x_r_their_share)
    }

    pub fn receive_input_share(&mut self, shares: (u8, u8, u8)) {
        self.y_a_own_share = shares[0];
        self.y_b_own_share = shares[1];
        self.y_r_own_share = shares[2];
    }

    pub fn receive(&mut self, input: u8) {
        match self.progress + 1 {
            1 => {
                self.d = self.d_own_share.bitxor(input);
            }
            2 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randoms[0][2];
                let ex_term = self.e * self.x_a_own_share;
                let dy_term = self.d * self.y_a_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = 1.bitxor(w_term).bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }

            3 => {
                self.d = self.d_own_share.bitxor(input);
            }
            4 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randoms[1][2];
                let ex_term = self.e * self.x_b_own_share;
                let dy_term = self.d * self.y_b_own_share;
                let ed_term = self.e * self.d;
                self.z_temp_own_share = 1.bitxor(w_term).bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            5 => {
                self.d = self.d_own_share.bitxor(input);
            }
            6 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randoms[2][2];
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

                let w_term = self.randoms[3][2];
                let ex_term = self.e * self.x_r_own_share;
                let dy_term = self.d * self.y_r_own_share;
                let ed_term = self.e * self.d;
                self.z_temp_own_share = 1.bitxor(w_term).bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            9 => {
                self.d = self.d_own_share.bitxor(input);
            }
            10 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randoms[2][2];
                let ex_term = self.e * self.z_own_share;
                let dy_term = self.d * self.z_temp_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            _ => {
                panic!("More than 5 iterations of receive!")
            }
        }
    }

    pub fn output(&self) -> u8 {
        self.z
    }
}

impl Bob {
    pub fn new() -> Bob {
        Bob {
            y_a: 0,
            y_b: 0,
            y_r: 0,
            z_own_share: 0,
            z_their_share: 0,
            z_temp_own_share: 0,
            z_temp_their_share: 0,
            randoms: [[0; 3]; 5],
            progress: 0,
            y_a_own_share: 0,
            y_b_own_share: 0,
            y_r_own_share: 0,
            y_a_their_share: 0,
            y_b_their_share: 0,
            y_r_their_share: 0,
            x_a_own_share: 0,
            x_b_own_share: 0,
            x_r_own_share: 0,
            d: 0,
            e: 0,
            d_own_share: 0,
            e_own_share: 0,
            d_their_share: 0,
            e_their_share: 0,
        }
    }

    pub fn init(&mut self, y: u8, randoms: [[u8; 3]; 5]) {
        let mut rng = rand::thread_rng();
        self.y_a = y & 1;
        self.y_b = y & 2;
        self.y_r = y & 4;
        self.randoms = randoms;
        self.y_a_their_share = rng.gen_range(0..=1);
        self.y_a_own_share = self.y_a.bitxor(self.y_a_their_share);
        self.y_b_their_share = rng.gen_range(0..=1);
        self.y_b_own_share = self.y_b.bitxor(self.y_b_their_share);
        self.y_r_their_share = rng.gen_range(0..=1);
        self.y_r_own_share = self.y_r.bitxor(self.y_r_their_share);
    }
    pub fn send_input_share(&self) -> (u8, u8, u8) {
        (self.y_a_their_share, self.y_b_their_share, self.y_r_their_share)
    }

    pub fn receive_input_share(&mut self, shares: (u8, u8, u8)) {
        self.x_a_own_share = shares[0];
        self.x_b_own_share = shares[1];
        self.x_r_own_share = shares[2];
    }

    pub fn send(&mut self) -> u8 {
        self.progress += 1;
        match self.progress {
            1 => {
                self.d_own_share = self.x_a_own_share.bitxor(self.randoms[0][0]);

                self.e_own_share = self.y_a_own_share.bitxor(self.randoms[0][1]);

                self.d_own_share
            }
            2 => {
                self.e_own_share
            }
            3 => {
                self.d_own_share = self.x_b_own_share.bitxor(self.randoms[1][0]);

                self.e_own_share = self.y_b_own_share.bitxor(self.randoms[1][1]);

                self.d_own_share
            }
            4 => {
                self.e_own_share
            }
            5 => {
                self.d_own_share = self.z_own_share.bitxor(self.randoms[2][0]);

                self.e_own_share = self.z_temp_own_share.bitxor(self.randoms[2][1]);

                self.d_own_share
            }
            6 => {
                self.e_own_share
            }
            7 => {
                self.d_own_share = self.x_r_own_share.bitxor(self.randoms[3][0]);

                self.e_own_share = self.y_r_own_share.bitxor(self.randoms[3][1]);

                self.d_own_share
            }
            8 => {
                self.e_own_share
            }
            9 => {
                self.d_own_share = self.z_own_share.bitxor(self.randoms[4][0]);

                self.e_own_share = self.z_temp_own_share.bitxor(self.randoms[4][1]);

                self.d_own_share
            }
            10 => {
                self.e_own_share
            }
            _ => {
                panic!("More than 5 iterations of send!")
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

                let w_term = self.randoms[0][2];
                let ex_term = self.e * self.x_a_own_share;
                let dy_term = self.d * self.y_a_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }

            3 => {
                self.d = self.d_own_share.bitxor(input);
            }
            4 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randoms[1][2];
                let ex_term = self.e * self.x_b_own_share;
                let dy_term = self.d * self.y_b_own_share;
                let ed_term = self.e * self.d;
                self.z_temp_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            5 => {
                self.d = self.d_own_share.bitxor(input);
            }
            6 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randoms[2][2];
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

                let w_term = self.randoms[3][2];
                let ex_term = self.e * self.x_r_own_share;
                let dy_term = self.d * self.y_r_own_share;
                let ed_term = self.e * self.d;
                self.z_temp_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            9 => {
                self.d = self.d_own_share.bitxor(input);
            }
            10 => {
                self.e = self.e_own_share.bitxor(input);

                let w_term = self.randoms[2][2];
                let ex_term = self.e * self.z_own_share;
                let dy_term = self.d * self.z_temp_own_share;
                let ed_term = self.e * self.d;
                self.z_own_share = w_term.bitxor(ex_term).bitxor(dy_term).bitxor(ed_term);
            }
            _ => {
                panic!("More than 5 iterations of receive!")
            }
        }
    }
}
/*
    Helper Functions
*/

fn local_xor(a: u8, b: u8) -> u8 {
    a.bitxor(b)
}


// We made our own modulo function as Rust doesn't have one by default
fn modulo(a: u8, b: u8) -> u8 {
    ((a % b) + b) % b
}

