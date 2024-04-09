// 函数学习
// fn main() {
//     println!("add_number_func{} 的返回结果是 {}", 1, add_number_func1(5, 6));
//     println!("add_number_func{} 的返回结果是 {}", 2, add_number_func2(5, 6));
//     println!("add_number_func{} 的返回结果是 {}", 3, add_number_func3());
// }

// fn add_number_func1(a : i32, b : i32) -> i32 {
//     a + b
// }

// fn add_number_func2(a : i32, b : i32) -> i32 {
//     return a + b
// }

// fn add_number_func3() -> i32{
//     let a = 5;
//     let b = 6;
//     a + b
// }

// 函数与所有权学习

// fn main()
// {
//     let s = String::from("hello");
//     take_ownership(s);
//     // s 失效
//     // println!("{}", s); //value borrowed here after move

//     let x = 5;
//     make_copy(x);
//     println!("{}", x);
// }

// fn take_ownership(somestring: String) //this parameter takes ownership of the value
// {
//     println!("{}", somestring);
//     //函数退出，自动调用drop函数，somestring内存被回收
// }

// fn make_copy(somenumber:i32)
// {
//     println!("{}", somenumber);
// }


// 悬空引用

// fn main()
// {
//     let r = dangle();
    
// }

// fn dangle()
// {
//     let s = String::from("hello, hello");
//     &s // 函数退出后s的内存被释放，但是这里又返回了s的引用，使用错误，&s将指向一个已经释放的内存地址。rust禁止这样使用，编译报错
// }


// 切片

fn main()
{
    let s = String::from("hello, world");
    
    let sub_s = first_world(&s[..]);
    println!("'{}' 的第一个单词是 {}", s, sub_s);

    let s = "hello, world";
    let sub_s = first_world(s);
    println!("'{}' 的第一个单词是 {}", s, sub_s);

}

// 返回输入字符串的第一个单词
// 注意这里的入参是&str类型而不是String类型
fn first_world(s:&str) -> &str
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}