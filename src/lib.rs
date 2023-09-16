pub fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    if max == 0 {return vec![false]};
    if max == 1 {return vec![false, false]};

    let mut values = vec![true; max + 1];
    values[0] = false;
    values[1] = false;
    let upper: usize = ((max as f64).sqrt() as usize) + 1;
    for i in 2..upper {
        if values[i] {
            for j in ((i*i)..=max).step_by(i) {
                values[j] = false;
            }
        }
    }
    values
}

pub fn prime_count(max: usize) -> usize {
    let prime_mask = sieve_of_eratosthenes(max);
    prime_mask.into_iter().map(|x| x as usize).sum()
}
