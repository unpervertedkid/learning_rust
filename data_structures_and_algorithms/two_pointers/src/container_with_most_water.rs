use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = height.len() - 1;

    let mut max_area: i32 = -1;

    while start < end {
        let start_height = height[start];
        let end_height = height[end];

        let water_height = min(start_height, end_height);
        let water_width = (end - start) as i32;

        let current_area = water_height * water_width;
        max_area = max(max_area, current_area);

        if start_height < end_height {
            start += 1;
        } else if end_height < start_height {
            end -= 1;
        } else {
            start += 1;
            end -= 1;
        }
    }

    return max_area;
}
