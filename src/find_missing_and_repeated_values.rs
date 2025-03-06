#[allow(dead_code)]
pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let grid_size = grid.len() * grid.len();
    let mut is_found = vec![false; grid_size];
    let mut repeated_val = -1;
    for row in grid {
        for val in row {
            let x = val - 1;
            if is_found[x as usize] {
                repeated_val = val;
            } else {
                is_found[x as usize] = true;
            }
        }
    }
    let mut missing_val = -1;
    for i in 0..grid_size {
        if !is_found[i] {
            missing_val = (i + 1) as i32;
        }
    }
    vec![repeated_val, missing_val]
}
