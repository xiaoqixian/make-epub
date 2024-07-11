// Date:   Wed Jul 03 12:36:10 2024
// Mail:   lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use crate::common::{ZH_NUMS, ZH_UNITS};

pub fn num_to_zh(mut num: usize) -> String {
    let mut res = String::default();
    let (mut zero_flag, mut unit_index) = (false, 0 as usize);
    let og = num;

    while num > 0 {
        let digit = (num % 10) as usize;
        if digit == 0 {
            if !res.is_empty() && !zero_flag {
                res.push('零');
            }
            zero_flag = true;
        } else {
            if unit_index > 0 {
                res.push(ZH_UNITS[unit_index]);
            }
            res.push(ZH_NUMS[digit]);
            zero_flag = false;
        }
        unit_index += 1;
        num /= 10;
    }
    if og >= 10 && og < 20 {
        let _ = res.pop();
    }
    res.chars().rev().collect::<String>()
}

