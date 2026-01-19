pub fn retarget(
    prev_diff: u32,
    actual_time: i64,
    target_time: i64
) -> u32 {
    if actual_time < target_time / 4 {
        prev_diff + 1
    } else if actual_time > target_time * 4 && prev_diff > 1 {
        prev_diff - 1
    } else {
        prev_diff
    }
}