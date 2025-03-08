pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let k = k as usize;
    let is_black: Vec<bool> = blocks
      .chars()
      .map(|c| c == 'B')
      .collect();
    let mut current = k - is_black[0..k].iter().filter(|&&x| x).count();
    let mut best = current;
    for i in k..is_black.len() {
        if is_black[i - k] {
            current += 1;
        }
        if is_black[i] {
            current -= 1;
        }
        if current < best {
            best = current;
        }
    }
    best as i32
}
