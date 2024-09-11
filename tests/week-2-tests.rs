
extern crate cc;

pub use cc::{TrustedDealer, Alice, Bob};

#[test]
    fn it_works() {
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