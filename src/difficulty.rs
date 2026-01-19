pub fn retarget(
    prev_diff: u32,
    actual_time: i64,
    target_time: i64,
) -> u32 {
    if actual_time <= 0 {
        return prev_diff.max(256);
    }

    if actual_time < target_time / 20 {
        return (prev_diff.saturating_mul(4)).clamp(256, 4096);
    }

    let actual = actual_time as f64;
    let target = target_time as f64;

    let ratio = target / actual;
    let adjustment = ratio.ln();

    let mut new_diff =
        prev_diff as f64 * (1.0 + adjustment * 0.75);

    new_diff = new_diff.clamp(256.0, 4096.0);

    if (new_diff - prev_diff as f64).abs() < 2.0 {
        prev_diff
    } else {
        new_diff.round() as u32
    }
}
