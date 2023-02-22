/*
https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/description/

A conveyor belt has packages that must be shipped from one port to another within days days.

The ith package on the conveyor belt has a weight of weights[i]. Each day, we load the ship with packages on the conveyor belt (in the order given by weights). We may not load more weight than the maximum weight capacity of the ship.

Return the least weight capacity of the ship that will result in all the packages on the conveyor belt being shipped within days days.
*/

use std::cmp;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let (total_weight, max_weight) = weights
            .iter()
            .fold((0, 0), |(total, max), weight| (total + *weight, cmp::max(max, *weight)));
        let avg_weight_per_day = {
            let div = total_weight / days;
            if total_weight % days == 0 { div } else { div + 1 }
        };
        let lower = cmp::max(max_weight, avg_weight_per_day);
        let upper = total_weight;
        
        find_lowest_capacity(weights, days, lower, upper)
    }
}

fn find_lowest_capacity(weights: Vec<i32>, days: i32, lower: i32, upper: i32) -> i32 {
    if lower >= upper {
        return upper;
    }
    let capacity = lower + (upper - lower) / 2;
    let is_overweight = ships_required_for(&weights, capacity) > days;
    if is_overweight {
        find_lowest_capacity(weights, days, capacity + 1, upper)
    } else {
        find_lowest_capacity(weights, days, lower, capacity)
    }
}

fn ships_required_for(weights: &Vec<i32>, capacity: i32) -> i32 {
    let mut ships_count = 0;
    let mut curr_ship = 0;
    for weight in weights.iter() {
        if curr_ship + weight <= capacity {
            curr_ship += weight;
        } else {
            ships_count += 1;
            curr_ship = *weight;
        }
    }
    ships_count + 1
}
