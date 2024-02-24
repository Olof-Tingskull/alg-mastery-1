#[allow(dead_code)]
pub fn run (r: f64, f: &[f64]) -> bool {
    let total_r = r * (f.len() as f64);
    let mut sorted_f = f.to_vec();
    sorted_f.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let can_be_done = sorted_f
        .iter()
        .enumerate()
        .all(|(i, f)| {
            f + ((i + 1) as f64) * r > total_r
        });

    can_be_done
}