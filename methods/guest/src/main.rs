#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use concrete_ntt::prime32::Plan;

use risc0_zkvm::guest::env;
use serde::{Serialize, Deserialize};

risc0_zkvm::guest::entry!(main);

#[derive(Serialize, Deserialize)]
struct NTT {
    n: usize,
    p: u32,
    data: Vec<u32>,
}

fn perform_ntt(ntt_input: NTT) {

    let plan = Plan::try_new(ntt_input.n, ntt_input.p).unwrap();

    let mut signal: [u32; 512] = [0; 512];
    for i in 1..ntt_input.n {
        signal[i] = ntt_input.data[i];
    }
    let mut transformed_fwd = signal;
    plan.fwd(&mut transformed_fwd);

    let mut transformed_inv = transformed_fwd;
    plan.inv(&mut transformed_inv);

    for (&actual, expected) in transformed_inv.iter().zip(signal.iter().map(|x| x * ntt_input.n as u32)) {
        assert_eq!(expected, actual);
    }
}

fn main() {
    // TODO: Implement your guest code here

    // read the input
    // let input: u32 = env::read();
    let ntt_input: NTT = env::read();
    perform_ntt(ntt_input);

    // write public output to the journal
    let o = 1u32;
    env::commit(&o);
}
