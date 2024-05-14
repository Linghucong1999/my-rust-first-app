use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜猜数字!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("请输入你的猜测");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
        let guess: u32 = guess.trim().parse().expect("输入值转换无效");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("恭喜你，猜对了！");
                break;
            }
        }
    }
}
