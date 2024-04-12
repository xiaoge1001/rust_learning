// 题目链接：https://leetcode.cn/problems/two-sum/description/


// 暂时还未学习Vec，使用已学的内容解决解题
// 不能提交leetcode CI,因为没有按照给定的形式去实现
use std::io;

struct InputInfo {
    number_value: usize,
    number_input_index: usize,
}

fn main() {
    // 获取输入
    let mut nums_str = String::new();
    let mut target: String = String::new();
    // 第一行输入整数数组nums，每个元素之间用空白分割
    io::stdin().read_line(&mut nums_str).expect("读取输入失败");
    io::stdin().read_line(&mut target).expect("读取输入失败");

    let target: usize = target.trim().parse().expect("解析输入的target失败");

    let mut input_data1: InputInfo = InputInfo {
        number_value:0, 
        number_input_index:0
    };
    let mut input_data2: InputInfo = InputInfo {
        number_value:0, 
        number_input_index:0
    };

    for item_i in nums_str.split_whitespace().enumerate() 
    {
        input_data1.number_value = item_i.1.parse().expect("解析输入的数组失败");
        input_data1.number_input_index = item_i.0;
        for item_j in nums_str.split_whitespace().enumerate() 
        {
            input_data2.number_value = item_j.1.parse().expect("解析输入的数组失败");
            input_data2.number_input_index = item_j.0;
            if item_j.0 > item_i.0 && input_data1.number_value + input_data2.number_value == target
            {
                println!("[{}, {}]", input_data1.number_input_index, input_data2.number_input_index);
                break;
            }
        }
    }
}
