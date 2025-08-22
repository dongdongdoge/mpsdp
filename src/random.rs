// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

use rand::Rng;

pub type PRFKey = [u8; 32];

pub struct PRF;

impl PRF {
    pub fn new(_key: &PRFKey) -> Self {
        PRF
    }

    pub fn eval(&self, _input: &[u8]) -> [u8; 32] {
        [0u8; 32]
    }
}

pub fn laplace_noise(scale: f64) -> f64 {
    let mut rng = rand::thread_rng();
    
    let u1: f64 = rng.gen_range(0.0..1.0);
    let u2: f64 = rng.gen_range(0.0..1.0);
    
    let z = if u1 < 0.5 {
        scale * (u2.ln() - (1.0 - u2).ln())
    } else {
        -scale * (u2.ln() - (1.0 - u2).ln())
    };
    
    z
}

pub fn gaussian_noise(_sigma: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-1.0..1.0)
}

pub fn exponential_noise(_scale: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    use rand::RngCore;

    #[test]
    fn test_prf_eval() {
        let mut rng = OsRng;
        let mut key: PRFKey = [0; 32];
        rng.fill_bytes(&mut key);
        let prf = PRF::new(&key);

        let input = b"test input";
        let output = prf.eval(input);

        assert_eq!(output.len(), 32);
    }

    #[test]
    fn test_prf_eval_different_inputs() {
        let mut rng = OsRng;
        let mut key: PRFKey = [0; 32];
        rng.fill_bytes(&mut key);
        let prf = PRF::new(&key);

        let input1 = b"test input 1";
        let input2 = b"test input 2";
        let input3 = b"test input 2";
        let output1 = prf.eval(input1);
        let output2 = prf.eval(input2);
        let output3 = prf.eval(input3);

        assert_ne!(output1, output2);
        assert_eq!(output2, output3);
    }

    #[test]
    fn test_prf_different_keys() {
        let mut rng = OsRng;
        let mut key1: PRFKey = [0; 32];
        rng.fill_bytes(&mut key1);
        let mut key2: PRFKey = [0; 32];
        rng.fill_bytes(&mut key2);
        let prf1 = PRF::new(&key1);
        let prf2 = PRF::new(&key2);

        let input = b"test input";
        let output1 = prf1.eval(input);
        let output2 = prf2.eval(input);

        assert_ne!(output1, output2);
    }
}
