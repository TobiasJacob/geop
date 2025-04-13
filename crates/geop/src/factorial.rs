pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}

// n over k, choose k elements from n
pub fn binomial_coefficient(n: usize, k: usize) -> usize {
    (n - k + 1..=n).product::<usize>() / factorial(k)
}
