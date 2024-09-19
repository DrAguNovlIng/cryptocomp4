
extern crate cc;

pub use cc::{TrustedDealer, Alice, Bob};

#[test]
    fn it_works() {
    // This main method runs the tests from the week-2-tests.rs file, it can also be run using the command `cargo test`
    // For the implementation of the structs and methods, check the lib.rs file
    // First bit indicates A, second bit B, last bit +/-
    // Example: 001 = O+, 110 = AB-
    // Truth table goes: 000  001  010  011  100  101  110  111 (left to right, and up to down)
    let true_truth_table: [[u8; 8]; 8] = [
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 0, 0, 0, 0, 0, 0],
        [1, 0, 1, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0],
        [1, 1, 0, 0, 1, 1, 0, 0],
        [1, 0, 1, 0, 1, 0, 1, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut calculated_truth_table: [[u8; 8]; 8] = [
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
    ];

    let mut dealer = TrustedDealer::new();
    let mut alice = Alice::new();
    let mut bob = Bob::new();

    // Tests for every combination of inputs, using the implementation example presented in the assignment exercise
    for i in 0..8 {
        for j in 0..8 {
            dealer.init();
            alice.init(i, dealer.rand_a());
            bob.init(j, dealer.rand_b());
            bob.receive_input_share(alice.send_input_share());
            alice.receive_input_share(bob.send_input_share());
            while !alice.has_output() {
                bob.receive(alice.send());
                alice.receive(bob.send());
            }
            let z = alice.output();
            calculated_truth_table[i as usize][j as usize] = z;
        }
    }

    for i in 0..8 {
        for j in 0..8 {
            assert_eq!(calculated_truth_table[i as usize][j as usize], true_truth_table[i as usize][j as usize]);
        }
    }

    println!("All tests passed successfully!")
    }