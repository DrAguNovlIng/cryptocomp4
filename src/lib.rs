use std::{fs::File, io::Write};
use rand::prelude::Distribution;
use num_bigint::{BigUint, RandomBits};
use miller_rabin::is_prime;
use serde::{Deserialize, Serialize};

/*

    Implementation of everything Group related
    This includes saving and reading groups to and from files
    since generating a group is time consuming

*/


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    g: BigUint, //generator
    q: BigUint, //order of the group
    p: BigUint, //prime number
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
            let g = BigUint::from(2u8); //TODO I dont know if this a good generator for the group (whether it generates only a subgroup of size q, and not p)
            return Group { g, q, p };
        }
    }
    panic!("Could not generate a safe prime group");
}

/*
    Implementation of ElGamal
*/

pub type Ciphertext = (BigUint, BigUint);
pub type Plaintext = BigUint;
pub type SecretKey = BigUint;
pub type PublicKey = BigUint;

pub struct ElGamal {
    group: Group,
}

impl ElGamal {
    //The constructure creates a group of prime order q and a generator g
    pub fn new(group: Group) -> Self {
        Self { group: group }
    }

    //Takes a secret key and outputs a corresponding public key
    pub fn gen(&self, sk: SecretKey) -> PublicKey {
        let h = self.group.g.clone().modpow(&sk, &self.group.p);
        h
    }

    //Takes some randomness and outputs a random looking public key
    pub fn o_gen(&self, r: BigUint) -> PublicKey {
        todo!()
    }

    //Encrypts a message using a public key
    pub fn enc(&self, pk: PublicKey, m: Plaintext) -> Ciphertext {
        todo!()
    }

    //Decrypts a message using a secret key
    pub fn dec(&self, sk: SecretKey, c: Ciphertext) -> Plaintext {
        todo!()
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
}

fn translate_input(input: u8) -> (bool, bool, bool) {
    let a = (input & 4) > 0;
    let r = (input & 1) > 0;
    let b = (input & 2) > 0;
    (a, b, r)
}

impl Alice {
    pub fn new(common_group: Group) -> Self {
        Self { el_gamal: ElGamal::new(common_group), input_a: false, input_b: false, input_r: false }
    }

    pub fn choose(&mut self, input: u8) -> (PublicKey, PublicKey, PublicKey, PublicKey, PublicKey, PublicKey, PublicKey, PublicKey) {
            let (a, b, r) = translate_input(input);
            self.input_a = a;
            self.input_b = b;
            self.input_r = r;
            todo!()
        }

    pub fn retrieve(&mut self, m2: (Ciphertext, Ciphertext, Ciphertext, Ciphertext, Ciphertext, Ciphertext, Ciphertext, Ciphertext)) -> u8 {
        todo!()
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

impl Bob {
    pub fn new(common_group: Group) -> Self {
        Self { el_gamal: ElGamal::new(common_group),  input_a: false, input_b: false, input_r: false }
    }

    pub fn transfer(&mut self, input: u8, m1_from_alice: (PublicKey, PublicKey, PublicKey, PublicKey, PublicKey, PublicKey, PublicKey, PublicKey)) -> (Ciphertext, Ciphertext, Ciphertext, Ciphertext, Ciphertext, Ciphertext, Ciphertext, Ciphertext) {
        let (a, b, r) = translate_input(input);
        self.input_a = a;
        self.input_b = b;
        self.input_r = r;
        todo!()
    }
}
