// 题目链接：https://leetcode.cn/problems/length-of-last-word/description/

// 本地实现，不采用leetcode形式

/*
use std::io;

fn main() {
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("读取输入字符串失败");
    println!("{}", input_str.split_whitespace().last().expect("解析输入字符串").len());
}
*/

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().expect("解析输入字符串").len() as i32
    }
}
