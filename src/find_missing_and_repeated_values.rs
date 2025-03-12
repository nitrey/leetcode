// https://leetcode.com/problems/find-missing-and-repeated-values
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = vec![vec![1, 3], vec![2, 2]];
        let result = find_missing_and_repeated_values(input);
        assert_eq!(result[0], 2);
        assert_eq!(result[1], 4);
    }

    #[test]
    fn test_b() {
        let input = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
        let result = find_missing_and_repeated_values(input);
        assert_eq!(result[0], 9);
        assert_eq!(result[1], 5);
    }
}
