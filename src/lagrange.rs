// Low degree extension using Lagrange interpolation

const PRIME: u32 = 16411;

fn mod_pow(base: u32, exponent: u32) -> u32 {
  let mut result = 1;
  let mut base = base;
  let mut exp = exponent;

  while exp > 0 {
      if exp & 1 == 1 {
          result = ((result * base) % PRIME) as u32;
      }
      base = (base * base) % PRIME;
      exp >>= 1;
  }
  result
}

// modular inverse using Fermat's Little Theorem
fn mod_inverse(a: u32) -> u32 {
  mod_pow(a, PRIME - 2)
}

fn lagrange_interpolate(y_values: &[u32], x: u32) -> u32 {
  let mut result = 0;

  for i in 0..y_values.len() {
      let mut numerator = 1;
      let mut denominator = 1;

      for j in 0..y_values.len() {
          if i != j {
              numerator = (numerator * ((x + PRIME - j as u32) % PRIME)) % PRIME;
              denominator = (denominator * ((i as u32 + PRIME - j as u32) % PRIME)) % PRIME;
          }
      }

      let term = (y_values[i] * numerator * mod_inverse(denominator)) % PRIME;
      result = (result + term) % PRIME;
  }

  result
}

pub fn extend(points: &[u32], k: usize) -> Vec<u32> {
    if points.len() > k {
        panic!("Cannot interpolate more points than requested");
    }
    let mut encoded = Vec::with_capacity(k);
    encoded.extend_from_slice(points);
    for i in points.len()..k {
        encoded.push(lagrange_interpolate(&points, i as u32));
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_degree_extension() {
        let points = vec![3, 1, 11, 7];
        let k = 10;

        let extended = extend(&points, k);

        println!("extended: {:?}", extended);

        assert_eq!(extended, vec![3, 1, 11, 7, 16374, 16264, 16062, 15742, 15278, 14644]);
    }
}
