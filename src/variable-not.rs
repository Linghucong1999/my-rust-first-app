// use std::io;
fn main() {
    // 数组类型
    // let a = [3, 1, 2, 4, 5, 7, 6];
    // println!("输入访问的元素索引：");
    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("读取失败");
    // let index: usize = index.trim().parse().expect("输入错误");
    // let element = a[index];
    // println!("访问的元素的索引是{}，元素是{}",index, element);

    // 函数开发

    anothor_function(5, 6);
}

fn anothor_function(x: i64, y: i64) {
    let z = {
        let a = x + y;
        let b = a * 2;
        let c = five();
        five_plus(b + c)
    };
    println!("z is {}", z);
}

fn five() -> i64 {
    200
}
fn five_plus(x: i64) -> i64 {
    x + 5
}
