use std::ops::BitXor;
use rand;
use rand::Rng;

fn main() {
    // Truth table goes: 000  001  010  011  100  101  110  111
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

    dealer.init();
    alice.init(0b110, dealer.rand_a());
    bob.init(0b111, dealer.rand_b());
    bob.receive(alice.send());
    alice.receive(bob.send());
    let z = alice.output();

    println!("z is {}", z)

}

struct TrustedDealer {
    r: i32,
    s: i32,
    tt: [[u8; 8]; 8],
    m_a: [[u8; 8]; 8],
    m_b: [[u8; 8]; 8]
}

struct Alice {
    x: i32,
    r: i32,
    m_a: [[u8; 8]; 8],
    u: i32,
    z: u8
}

struct Bob {
    y: i32,
    s: i32,
    m_b: [[u8; 8]; 8],
    v: i32,
    z_b: u8
}

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

        for i in 0..8 {
            for j in 0..8 {
                let i_signed = i as i32;
                let j_signed = j as i32;

                let shifted_i = modulo(i_signed-self.r, 8);
                let shifted_j = modulo(j_signed-self.s, 8);
                self.m_a[i][j] = self.m_b[i][j].bitxor(self.tt[shifted_i as usize][shifted_j as usize]);
            }
        }
    }

    fn rand_a(&self) -> (i32, [[u8; 8];8]) {
        (self.r, self.m_a)
    }

    fn rand_b(&self) -> (i32, [[u8; 8];8]) {
        (self.s, self.m_b)
    }
}

impl Alice {
    fn new() -> Alice {
        Alice {
            x: 0,
            r: 0,
            m_a: [[0; 8]; 8],
            u: 0,
            z: 0
        }
    }

    fn init(&mut self, x: i32, (r, m_a): (i32, [[u8; 8];8])) {
        self.x = x;
        self.r = r;
        self.m_a = m_a;

        self.u = modulo(self.x + self.r, 8);
    }

    fn send(&self) -> i32 {
        self.u
    }

    fn receive(&mut self, (v, z_b): (i32, u8)) {
        self.z = self.m_a[self.u as usize][v as usize].bitxor(z_b);
    }

    fn output(&self) -> u8 {
        self.z
    }
}

impl Bob {
    fn new() -> Bob {
        Bob {
            y: 0,
            s: 0,
            m_b: [[0; 8]; 8],
            v: 0,
            z_b: 0,
        }
    }

    fn init(&mut self, y: i32, (s, m_b): (i32, [[u8; 8];8])) {
        self.y = y;
        self.s = s;
        self.m_b = m_b;

        self.v = modulo(y + s, 8);
    }

    fn receive(&mut self, u: i32) {
        self.z_b = self.m_b[u as usize][self.v as usize];
    }

    fn send(&self) -> (i32, u8) {
        (self.v, self.z_b)
    }
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

