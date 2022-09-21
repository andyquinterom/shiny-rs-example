
fn sample_dist(n: u64, mean: f64, sd: f64) -> Vec<f64> {
    get_dist(n as usize, mean, sd).unwrap_or_default()
}
