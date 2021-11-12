use std::io;

fn main() {
    /*
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    */

    //存在栈上的数据，没有所有权转移，直接拷贝
    let x = 5;
    let y = x;
    println!("x={}, y={}", x, y);

    let x = x + 1;
    println!("x={}", x);

    let x  = "123";
    println!("x={}", x);

    let s = String::from("hello, world!");
    //let s = get_len_of_string(&s);

    

    let _ss1 = &s;
    println!("len={}", get_len_of_string(&_ss1));
    //s.clear();
    //println!("ss1={}", ss1);

    let rect = Rect{
        width: 21,
        height: 10,
    };

    println!("the area is {:#?}", rect);
    println!("the area is {}", rect.area());

    let mut line = String::new();
    println!("请输入你的名字:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("你好 , {}", line);
    println!("读取的字节数为：{}", b1);

    //循环
    let mut i = 3;
    loop{
        println!("...");
        i -= 1;

        if i == 0 {
            break
        }
    }

    //所有权
    let str1 = String::from("data");
    {
        let str2 = &str1;
    }
    println!("str1 is {}", str1);
}

fn get_len_of_string (str: &String) -> usize {
    str.len()
}

#[derive(Debug)]
struct Rect{
    width : u32,
    height: u32,
}

impl Rect{
    fn area(&self) ->u32 {
        self.width * self.height
    }
}