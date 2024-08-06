// Sumcheck

use rand::Rng;

const PRIME: u64 = 16411;

pub fn fiat_shamir(_coefs: Vec<u64>) -> u64 {
    rand::thread_rng().gen_range(0..PRIME)
}

// f is the evaluations of the function at all 2^v points
pub fn prove_sumcheck(
    mut f: &mut [u64],
    size: u64,
    mut sum: u64,
) -> Vec<Vec<u64>> {
    let mut proof = Vec::new();
    let mut challenge = 0;
    for _ in 0..size {
        // evaluate the sum for current i = 0
        let e0: u64 = f.iter().take(f.len() / 2).sum::<u64>() % PRIME;

        // evaluate the sum for current i = 1
        let e1: u64 = f.iter().skip(f.len() / 2).sum::<u64>() % PRIME;
        
        assert_eq!((e0 + e1) % PRIME, sum);

        // this gives us univariate polynomial: f(x) = e0 + (e1 - e0)*x
        let coefs = vec![e0, (PRIME + e1 - e0) % PRIME];
        
        proof.push(vec![coefs[0], coefs[1], sum, challenge]);

        challenge = fiat_shamir(coefs.clone());

        // update f to be the new coefficients with x0 = challenge
        // from https://github.com/recmo/delegated-spartan/blob/main/src/sum_check.rs
        let (a, b) = f.split_at_mut(f.len() / 2);
        a.iter_mut().zip(b).for_each(|(a, b)| *a = (*a + challenge * ((*b + PRIME) - *a) % PRIME) % PRIME);
        f = a;

        // new sum
        sum = (coefs[0] + challenge * coefs[1]) % PRIME;
    }

    assert_eq!(f[0], sum);
    proof
}

pub fn verify_sumcheck(
    proof: Vec<Vec<u64>>,
    size: u64,
    sum: u64,
) -> bool {
    assert_eq!(proof.len() as u64, size);
    assert_eq!(proof[0][2], sum);
    for i in 0..size {
        let p = &proof[i as usize];
        assert_eq!((p[0] + p[0] + p[1]) % PRIME, p[2]);
        if i == 0 {
            assert_eq!(p[3], 0);
        } else {
            let prev_proof = &proof[i as usize - 1];
            assert_eq!((prev_proof[0] + prev_proof[1] * p[3]) % PRIME, p[2]);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumcheck_1() {
        // evaluations for (x, y) = (0, 0), (0, 1), (1, 0), (1, 1)
        let mut points = vec![3, 1, 11, 7];
        let size = 2;

        let sum = 22;

        let proof = prove_sumcheck(&mut points, size, sum);
        println!("proof: {:?}", proof);

        let result = verify_sumcheck(proof, size, sum);
        assert!(result);
    }
    
    #[test]
    fn test_sumcheck_2() {
        let mut points = vec![3, 1, 11, 7, 15, 5718, 1121, 7123];
        let size = 3;

        let sum = 13999;

        let proof = prove_sumcheck(&mut points, size, sum);
        println!("proof: {:?}", proof);

        let result = verify_sumcheck(proof, size, sum);
        assert!(result);
    }

}
