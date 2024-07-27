use rand::Rng;

const PRIME: u32 = 16411;

pub fn random_int() -> u32 {
    rand::thread_rng().gen_range(0..PRIME)
}

pub fn random_vector(n: usize) -> Vec<u32> {
    (0..n).map(|_| random_int()).collect()
}

pub fn random_matrix(n: usize, m: usize) -> Vec<Vec<u32>> {
    (0..n).map(|_| random_vector(m)).collect()
}

pub fn is_valid_matrix(a: &Vec<Vec<u32>>) -> bool {
    let m = a[0].len();
    a.iter().all(|vec| vec.len() == m)
}

pub fn empty_matrix(n: usize, m: usize) -> Vec<Vec<u32>> {
    vec![vec![0; m]; n]
}

pub fn sum_of_prods(a: &Vec<Vec<u32>>, b: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    (0..a.len()).map(|k| a[k][j] * b[i][k]).sum::<u32>() % PRIME
}

pub fn matmul(a: &Vec<Vec<u32>>, b: &Vec<Vec<u32>>) -> Result<Vec<Vec<u32>>, &'static str> {
    if !is_valid_matrix(a) || !is_valid_matrix(b) {
        return Err("Invalid matrix");
    }
    if a.len() != b[0].len() {
        return Err("Invalid dimensions for multiplication");
    }

    let n = b.len();
    let m = a[0].len();
    let mut product = empty_matrix(n, m);

    for i in 0..n {
        for j in 0..m {
            product[i][j] = sum_of_prods(a, b, i, j);
        }
    }

    Ok(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freivald() -> Result<(), &'static str> {
        let n: usize = 10;
        let m: usize = 10;

        let a = random_matrix(n, m);
        let b = random_matrix(n, m);

        let c = matmul(&a, &b)?;

        let x = random_vector(m);

        let y = matmul(&c, &vec![x.clone()])?;

        println!("Y: {:?}", y);

        let z = matmul(&a, &matmul(&b, &vec![x])?)?;

        println!("Z: {:?}", z);

        assert_eq!(y, z, "Y and Z should be equal");

        Ok(())
    }
}