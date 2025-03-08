use std::cmp::min;

#[allow(dead_code)]
pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let window_size = k as usize;
    let is_black: Vec<bool> = blocks.chars().map(|c| c == 'B').collect();
    let mut black_count = is_black[0..window_size].iter().filter(|&&x| x).count();
    let mut min_recolors = window_size - black_count;
    for i in window_size..is_black.len() {
        if is_black[i - window_size] {
            black_count -= 1;
        }
        if is_black[i] {
            black_count += 1;
        }
        let recolors = window_size - black_count;
        min_recolors = min(min_recolors, recolors);
    }
    min_recolors as i32
}
