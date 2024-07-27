// Reed - Solomon encoding

const PRIME: u32 = 16411;

// Evaluates polynomial with coefficients `coefs` at point `x`
pub fn eval(coefs: &[u32], x: u32) -> u32 {
    let mut sum = 0;
    let mut power = 1;
    for &value in coefs.iter() {
        sum = (sum + value * power) % PRIME;
        // stores running powers of r
        power = (power * x) % PRIME;
    }
    sum
}

pub fn encode(coefs: &[u32], k: u32) -> Vec<u32> {
    let mut encoded = Vec::with_capacity(k as usize);
    for i in 0..k {
        encoded.push(eval(coefs, i));
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprinting() {
        let message_text = "hi bob, how u doin'?";
        let message_alice: Vec<u32> = message_text.chars().map(|c| c as u32).collect();

        let challenge = 1838;

        assert_eq!(eval(&message_alice, challenge), 8133);
    }

    #[test]
    fn test_encoding() {
        let coefs = vec![1, 2, 3, 4, 5];
        let k = 10;

        assert_eq!(encode(&coefs, k), vec![1, 15, 129, 547, 1593, 3711, 7465, 13539, 6326, 3161]);
    }
}
