use std::io;
use rand::Rng;
use std::cmp::Ordering;

// fn main() {
//     println!("猜数字游戏开始了！！！");
//     println!("您可以输入你猜测的数字:");
//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("无法读取输入信息");
//     println!("你猜测的数字是 {}", guess);
// }

// fn main() {
//     println!("猜数字游戏开始了！！！");
//     // 设置被猜测的神秘数字
//     let secert_number = rand::thread_rng().gen_range(1..101);
//     println!("被猜测的神秘数字是：{}", secert_number);
//     println!("您可以输入你猜测的数字:");
//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("无法读取输入信息");
    
//     let guess: u32 = guess.trim().parse().expect("输入数据非法，请输入数字!");
//     match guess.cmp(&secert_number) {
//         Ordering::Less => println!("too less"),
//         Ordering::Greater => println!("too big"),
//         Ordering::Equal => println!("win !!!"),
//     }
// }


fn main() {
    println!("猜数字游戏开始了！！！");
    // 设置被猜测的神秘数字
    let secert_number = rand::thread_rng().gen_range(1..101);
    println!("被猜测的神秘数字是：{}", secert_number);
    
    loop {
        println!("您可以输入你猜测的数字:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取输入信息");
        
        //输入非法数据就会报错推出，不好
        // let guess: u32 = guess.trim().parse().expect("输入数据非法，请输入数字!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("输入不合法，请输入整数数据！！！\n");
                continue;
            }
        };
        match guess.cmp(&secert_number) {
            Ordering::Less => println!("too less"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("win !!!");
                break;
            }
        }
    }
}
