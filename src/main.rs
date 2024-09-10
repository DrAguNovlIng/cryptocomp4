use rand::Rng;
use std::ops::BitXor;

fn main() {
    // First bit indicates A, second bit B, last bit +/-
    // Example: 001 = O+, 110 = AB-
    // Truth table goes: 000  001  010  011  100  101  110  111 (left to right, and up to down)
    let truth_table: [[u8; 8]; 8] = [
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 0, 0, 0, 0, 0, 0],
        [1, 0, 1, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0],
        [1, 1, 0, 0, 1, 1, 0, 0],
        [1, 0, 1, 0, 1, 0, 1, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut dealer = TrustedDealer::new(truth_table);
    let mut alice = Alice::new();
    let mut bob = Bob::new();

    // Tests
    for i in 0..8 {
        for j in 0..8 {
            dealer.init();
            alice.init(i, dealer.rand_a());
            bob.init(j, dealer.rand_b());
            bob.receive(alice.send());
            alice.receive(bob.send());
            let z = alice.output();

            assert_eq!(z, truth_table[i as usize][j as usize]);
        }
    }
    println!("All tests passed successfully!")
}

// STRUCTS
struct TrustedDealer {
    r: i32,
    s: i32,
    tt: [[u8; 8]; 8],
    m_a: [[u8; 8]; 8],
    m_b: [[u8; 8]; 8],
}

struct Alice {
    m_a: [[u8; 8]; 8],
    u: i32,
    z: u8,
}

struct Bob {
    m_b: [[u8; 8]; 8],
    v: i32,
    z_b: u8,
}

// IMPLEMENTATIONS
impl TrustedDealer {
    fn new(tt: [[u8; 8]; 8]) -> TrustedDealer {
        TrustedDealer {
            tt,
            r: 0,
            s: 0,
            m_a: [[0; 8]; 8],
            m_b: [[0; 8]; 8],
        }
    }

    fn init(&mut self) {
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
    fn rand_a(&self) -> (i32, [[u8; 8]; 8]) {
        (self.r, self.m_a)
    }

    // Output (s, M_b) to Bob
    fn rand_b(&self) -> (i32, [[u8; 8]; 8]) {
        (self.s, self.m_b)
    }
}

impl Alice {
    fn new() -> Alice {
        Alice {
            m_a: [[0; 8]; 8],
            u: 0,
            z: 0,
        }
    }

    fn init(&mut self, x: i32, (r, m_a): (i32, [[u8; 8]; 8])) {
        self.m_a = m_a;

        // u = x + r mod 2^n
        self.u = modulo(x + r, 8);
    }

    fn send(&self) -> i32 {
        self.u
    }

    fn receive(&mut self, (v, z_b): (i32, u8)) {
        // z = M_a[u, v] xor z_b
        self.z = self.m_a[self.u as usize][v as usize].bitxor(z_b);
    }

    fn output(&self) -> u8 {
        self.z
    }
}

impl Bob {
    fn new() -> Bob {
        Bob {
            m_b: [[0; 8]; 8],
            v: 0,
            z_b: 0,
        }
    }

    fn init(&mut self, y: i32, (s, m_b): (i32, [[u8; 8]; 8])) {
        self.m_b = m_b;

        // v = y + s mod 2^n
        self.v = modulo(y + s, 8);
    }

    fn receive(&mut self, u: i32) {
        // z_b = M_b[u, v]
        self.z_b = self.m_b[u as usize][self.v as usize];
    }

    fn send(&self) -> (i32, u8) {
        (self.v, self.z_b)
    }
}

// We made our own modulo function as Rust doesn't have one by default
fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}
