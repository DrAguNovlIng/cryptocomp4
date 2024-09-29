use std::{array::from_fn, fs::File, io::Write};
use rand::{prelude::Distribution, Rng, RngCore};
use num_bigint::{BigUint, RandomBits, ToBigUint};
use miller_rabin::is_prime;
use serde::{Deserialize, Serialize};

/*

    Implementation of everything Group related
    This includes saving and reading groups to and from files
    since generating a group is time consuming

*/


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub g: BigUint, //generator
    pub q: BigUint, //order of the group
    pub p: BigUint, //prime number
}

impl Group {
    pub fn new(group_size: u64) -> Self {
        generate_safe_prime_group(group_size)
    }

    // Methods to write and read groups to and from files, saving time when testing, instead of generating a new group every time
    pub fn new_from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let group: Group = serde_json::from_reader(file).unwrap();
        group
    }

    pub fn write_group_to_file(&self, path: &str) {
        let json = serde_json::to_string(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn gen_random_exponent(&self) -> BigUint {
        let rng = &mut rand::thread_rng();
        loop {
            let r: BigUint = RandomBits::new(self.q.bits()).sample(rng);
            if r < self.q {
                return r;
            }
        }
    }
}

/*
    Methods for generate primes and safe prime groups
*/

// Methods to generate a prime, this is done by picking a random number of the desired size and using the Miller-Rabin primality test
pub fn generate_prime(size: u64) -> BigUint {
    for _ in 0..10000 {
        let rng = &mut rand::thread_rng();
        let maybe_prime: BigUint = RandomBits::new(size).sample(rng);
        if is_prime(&maybe_prime, 10) {
            return maybe_prime;
        }
    }
    panic!("Could not generate prime number");
}

// Method to generate a safe prime group, this is done by generating a prime q and then checking if 2q+1 is also a prime
pub fn generate_safe_prime_group(size: u64) -> Group {
    for _ in 0..10000 {
        let q = generate_prime(size);
        let p = 2u8 * q.clone() + 1u8;
        if is_prime(&p, 10) {
            let g = generate_random_safe_prime_group_element(p.clone());
            return Group { g, q, p };
        }
    }
    panic!("Could not generate a safe prime group");
}

pub fn generate_random_safe_prime_group_element(p: BigUint) -> BigUint {
    //We do not imput r, but rely on the rand crate to sample for us.
    let rng = &mut rand::thread_rng();
    loop {
        let s: BigUint = RandomBits::new(p.bits()).sample(rng);
        if s < p {
            let h = s.sqrt() % p;
            return h;
        }
    }
}

/*
    Implementation of ElGamal
*/

pub type Ciphertext = (BigUint, BigUint);
pub type Plaintext = BigUint;
pub type SecretKey = BigUint;
pub type PublicKey = BigUint; //We simply omit sending the group along for simplicity (we assume they agree on the group)

pub struct ElGamal {
    group: Group,
}

impl ElGamal {
    //The constructure creates a group of prime order q and a generator g
    pub fn new(group: Group) -> Self {
        Self { group: group }
    }

    pub fn gen_sk(&self) -> SecretKey {
        self.group.gen_random_exponent()
    }

    //Takes a secret key and outputs a corresponding public key
    pub fn gen_pk(&self, sk: SecretKey) -> PublicKey {
        let h = self.group.g.modpow(&sk, &self.group.p);
        h
    }

    //Takes some randomness and outputs a random looking public key
    pub fn o_gen_pk(&self) -> PublicKey {
        generate_random_safe_prime_group_element(self.group.p.clone())
    }

    //Encrypts a message using a public key
    pub fn enc(&self, pk: PublicKey, m: Plaintext) -> Ciphertext {
        let p = &self.group.p;
        let r = self.group.gen_random_exponent();
        let c1 = self.group.g.modpow(&r, p);
        let hr = pk.modpow(&r, p);
        let c2 = ((m % p) * (hr % p)) % p; //Might be too slow
        (c1, c2)
    }

    //Decrypts a message using a secret key
    pub fn dec(&self, sk: SecretKey, c: Ciphertext) -> Plaintext {
        let p = &self.group.p;
        let c1 = c.0;
        let c2 = c.1;
        let sk_inv = sk.modinv(p).unwrap();
        c1.modpow(&sk_inv, p);
        let m = ((c2 % p) * (c1 % p)) % p; //Might be too slow
        m
    }
}

/*

    Imeplementation of Alice

*/
pub struct Alice {
    el_gamal: ElGamal,
    input_a: bool,
    input_b: bool,
    input_r: bool,
    sk: SecretKey
}

fn translate_input(input: u8) -> (bool, bool, bool) {
    let a = (input & 4) > 0;
    let b = (input & 2) > 0;
    let r = (input & 1) > 0;
    (a, b, r)
}

fn translate_input_back(a: bool, b: bool, r: bool) -> u8 {
    let mut res = 0;
    if a { res += 4}
    if b { res += 2}
    if r { res += 1}
    res
}

impl Alice {
    pub fn new(common_group: Group) -> Self {
        Self { el_gamal: ElGamal::new(common_group), input_a: false, input_b: false, input_r: false, sk: BigUint::from(0u8) }
    }

    pub fn choose(&mut self, input: u8) -> [PublicKey; 8] {
            let (a, b, r) = translate_input(input);
            self.input_a = a;
            self.input_b = b;
            self.input_r = r;
            self.sk = self.el_gamal.gen_sk();
            let mut res: [PublicKey; 8] = from_fn(|_| {BigUint::from(0u8)});
            for i in 0..8 {
                if i != input {
                    res[i as usize] = self.el_gamal.o_gen_pk()
                }
            }
            res[input as usize] = self.el_gamal.gen_pk(self.sk.clone());
            res
        }

    pub fn retrieve(&mut self, m2: [Ciphertext; 8]) -> u8 {
        let input_index = translate_input_back(self.input_a, self.input_b, self.input_r);
        let ciphertext = m2[input_index as usize].clone();
        let decryption = self.el_gamal.dec(self.sk.clone(), ciphertext);
        decryption.to_bytes_be()[0]
    }
}


/*

    Implementation of Bob

*/
pub struct Bob {
    el_gamal: ElGamal,
    input_a: bool,
    input_b: bool,
    input_r: bool,
}

fn blood_function(Alice: (bool, bool, bool), Bob: (bool, bool, bool)) -> bool {
    if Alice.0 < Bob.0 {return false}
    if Alice.1 < Bob.1 {return false}
    if Alice.2 < Bob.2 {return false}
    true
}

impl Bob {
    pub fn new(common_group: Group) -> Self {
        Self { el_gamal: ElGamal::new(common_group),  input_a: false, input_b: false, input_r: false }
    }

    pub fn transfer(&mut self, input: u8, m1_from_alice: [PublicKey; 8]) -> [Ciphertext; 8] {
        let (a, b, r) = translate_input(input);
        self.input_a = a;
        self.input_b = b;
        self.input_r = r;
        let mut res: [Ciphertext; 8] = from_fn(|_| {(BigUint::from(0u8),BigUint::from(0u8))});
        for i in 0..8 {
            res[i as usize] = self.el_gamal.enc(m1_from_alice[i as usize].clone(), BigUint::from(blood_function(translate_input(i), (a,b,r))))
        }
        res
    }
}
