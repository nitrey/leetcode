// https://leetcode.com/problems/ipo/

use std::collections::BinaryHeap;

struct Project {
    capital: i32,
    profit: i32,
}

#[allow(dead_code)]
pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut projects: Vec<Project> = capital
        .into_iter()
        .zip(profits.into_iter())
        .map(|(capital, profit)| Project { capital, profit })
        .collect();
    projects.sort_unstable_by_key(|elem| elem.capital);

    let mut total_profit = w;
    let mut next_index = 0;
    let mut opportunities = BinaryHeap::with_capacity(projects.len());

    for _ in 0..k {
        while let Some(proj) = projects
            .get(next_index)
            .filter(|proj| proj.capital <= total_profit)
        {
            opportunities.push(proj.profit);
            next_index += 1;
        }
        if let Some(max_profit) = opportunities.pop() {
            total_profit += max_profit;
        }
    }

    total_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let k = 2;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 1];
        let result = find_maximized_capital(k, w, profits, capital);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_b() {
        let k = 3;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 2];
        let result = find_maximized_capital(k, w, profits, capital);
        assert_eq!(result, 6);
    }
}
