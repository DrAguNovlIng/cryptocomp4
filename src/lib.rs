use rand::Rng;
use std::ops::BitXor;

// STRUCTS
pub struct TrustedDealer {
    r: i32,
    s: i32,
    tt: [[u8; 8]; 8],
    m_a: [[u8; 8]; 8],
    m_b: [[u8; 8]; 8],
}

pub struct Alice {
    m_a: [[u8; 8]; 8],
    u: i32,
    z: u8,
}

pub struct Bob {
    m_b: [[u8; 8]; 8],
    v: i32,
    z_b: u8,
}

// IMPLEMENTATIONS
impl TrustedDealer {
    pub fn new(tt: [[u8; 8]; 8]) -> TrustedDealer {
        TrustedDealer {
            tt,
            r: 0,
            s: 0,
            m_a: [[0; 8]; 8],
            m_b: [[0; 8]; 8],
        }
    }

    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();

        // choose shifters
        self.r = rng.gen_range(0..8);
        self.s = rng.gen_range(0..8);

        // choose random M_b
        for i in 0..8 {
            for j in 0..8 {
                self.m_b[i][j] = rng.gen_range(0..=1);
            }
        }

        // Compute M_a
        // M_a[i, j] = M_b[i, j] xor T[i - r mod 2^n, j - s mod 2^n]
        for i in 0..8 {
            for j in 0..8 {
                let shifted_i = modulo(i as i32 - self.r, 8);
                let shifted_j = modulo(j as i32 - self.s, 8);
                self.m_a[i][j] =
                    self.m_b[i][j].bitxor(self.tt[shifted_i as usize][shifted_j as usize]);
            }
        }
    }

    // Output (r, M_a) to Alice
    pub fn rand_a(&self) -> (i32, [[u8; 8]; 8]) {
        (self.r, self.m_a)
    }

    // Output (s, M_b) to Bob
    pub fn rand_b(&self) -> (i32, [[u8; 8]; 8]) {
        (self.s, self.m_b)
    }
}

impl Alice {
    pub fn new() -> Alice {
        Alice {
            m_a: [[0; 8]; 8],
            u: 0,
            z: 0,
        }
    }

    pub fn init(&mut self, x: i32, (r, m_a): (i32, [[u8; 8]; 8])) {
        self.m_a = m_a;

        // u = x + r mod 2^n
        self.u = modulo(x + r, 8);
    }

    pub fn send(&self) -> i32 {
        self.u
    }

    pub fn receive(&mut self, (v, z_b): (i32, u8)) {
        // z = M_a[u, v] xor z_b
        self.z = self.m_a[self.u as usize][v as usize].bitxor(z_b);
    }

    pub fn output(&self) -> u8 {
        self.z
    }
}

impl Bob {
    pub fn new() -> Bob {
        Bob {
            m_b: [[0; 8]; 8],
            v: 0,
            z_b: 0,
        }
    }

    pub fn init(&mut self, y: i32, (s, m_b): (i32, [[u8; 8]; 8])) {
        self.m_b = m_b;

        // v = y + s mod 2^n
        self.v = modulo(y + s, 8);
    }

    pub fn receive(&mut self, u: i32) {
        // z_b = M_b[u, v]
        self.z_b = self.m_b[u as usize][self.v as usize];
    }

    pub fn send(&self) -> (i32, u8) {
        (self.v, self.z_b)
    }
}

// We made our own modulo function as Rust doesn't have one by default
fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}
