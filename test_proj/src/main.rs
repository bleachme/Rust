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

    
    let x = 5;
    println!("x={}", x);

    let x = x + 1;
    println!("x={}", x);

    let x  = "123";
    println!("x={}", x);

    let s = String::from("hello, world!");
    //let s = get_len_of_string(&s);

    println!("len={}", s);

    let ss1 = &s;
    //s.clear();
    //println!("ss1={}", ss1);

    let rect = Rect{
        width: 21,
        height: 10,
    };

    println!("the area is {:#?}", rect);
    println!("the area is {}", rect.area());
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