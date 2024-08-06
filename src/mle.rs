// Multilinear extension

const PRIME: u64 = 16411;

pub fn mle_eval(f: &[u64], e: &[u64]) -> u64 {
    // check f is 2^v
    debug_assert_eq!(f.len(), 1 << e.len());
    let mut result = 0;
    for w in 0..f.len() {
        let w_vector = (0..e.len()).map(|j| (w >> j) & 1).collect::<Vec<_>>();
        let mut prod = 1;
        for i in 0..e.len() {
            let term1 = e[i] * w_vector[i] as u64;
            let term2 = ((PRIME - e[i] + 1) * (1 - w_vector[i] as u64)) % PRIME;
            prod = prod * (term1 + term2) % PRIME;
        }
        result = (result + (f[w] * prod) % PRIME) % PRIME;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mle_1() {
        // evaluations for (x, y) = (0, 0), (0, 1), (1, 0), (1, 1)
        let points = vec![3, 1, 11, 7];

        // evaluation point
        let e = vec![2, 2];

        let result = mle_eval(&points, &e);
        println!("result: {:?}", result);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_mle_2() {
        let points = vec![0, 0, 0, 1, 0, 1, 0, 2];

        let e = vec![1, 1, 1];

        assert_eq!(mle_eval(&points, &e), 2);
    }

    #[test]
    fn test_mle_3() {
        let points = vec![8, 8, 8, 8];

        let e = vec![4, 3];

        assert_eq!(mle_eval(&points, &e), 8);
    }
}
