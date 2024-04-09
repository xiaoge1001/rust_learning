// 结构体struct和struct方法

#[derive(Debug)] //不加编译失败，有提示信息：add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
struct Rectangle {
    width : u32,
    length : u32,
}

impl Rectangle {
    // 方法 self是第一个参数
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other : &Rectangle) -> bool
    {
        self.width >= other.width && self.length >= other.length
    }

    // 关联函数 ： impl块里面不把self作为第一个入参的函数。关联函数常用于构造器
    fn square(size : u32) -> Rectangle
    {
        Rectangle{
            width : size,
            length : size,
        }
    }
}

fn main()
{
    let rect: Rectangle = Rectangle { 
        width: 5, 
        length: 6,
    };

    let rect2 : Rectangle = Rectangle
    {
        width : 2,
        length : 3,
    };

    let rect3 : Rectangle = Rectangle
    {
        width : 10,
        length : 3,
    };

    print!("Rectangle'area is {}\n", rect.area());

    // out is : Rectangle { width: 5, length: 6 }
    print!("{:?}\n", &rect); 

    /* 
    out is :
    Rectangle {
    width: 5,
    length: 6,
    }
     */
    print!("{:#?}\n", &rect); //显示更加友好

    println!("rect 是否可以包住 rect2: {}", rect.can_hold(&rect2));
    println!("rect 是否可以包住 rect3: {}", rect.can_hold(&rect3));

    let s = Rectangle::square(3);
    println!("rect 是否可以包住 s: {}", rect.can_hold(&s));
}