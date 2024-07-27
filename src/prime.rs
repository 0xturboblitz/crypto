pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64;
    // check for divisibility by 6k +/- 1
    for i in (5..=limit).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }
    true
}

pub fn find_prime_above(n: u64) -> u64 {
    (n..).find(|&i| is_prime(i)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_find_prime_above() {
        let minimum = 16384;
        let start = Instant::now();
        let result = find_prime_above(minimum);
        let duration = start.elapsed();

        println!("First prime number above {}: {}", minimum, result);
        println!("Time taken: {:?}", duration);

        assert!(result > minimum);
        assert!(is_prime(result));
        assert!(!is_prime(result - 1));
    }
}