pub fn retarget(
    prev_diff: u32,
    actual_time: i64,
    target_time: i64,
) -> u32 {
    if actual_time <= 0 {
        return prev_diff.max(8);
    }

    let actual = actual_time as f64;
    let target = target_time as f64;

    let ratio = target / actual;

    let adjustment = ratio.ln();

    let mut new_diff =
        prev_diff as f64 * (1.0 + adjustment * 0.5);

    new_diff = new_diff.clamp(8.0, 64.0);

    if (new_diff - prev_diff as f64).abs() < 0.5 {
        prev_diff
    } else {
        new_diff.round() as u32
    }
}
