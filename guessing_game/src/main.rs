use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("請猜測一個數字！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret 數字: {secret_number}");

    loop {

        println!("請輸入你的猜測數字。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("讀取該行失敗");

        //let guess: u32 = guess.trim().parse().expect("Input number!");
        let guess: u32  = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你的猜測數字：{guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too large!"),
            std::cmp::Ordering::Equal => {
                println!("Win!");
                break;
            }
    
        }
    }
}
