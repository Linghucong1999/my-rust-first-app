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

    // anothor_function(5, 6);

    // 控制流,不要过多的使用if else,可以使用match
    let mut x = anothor_function(5, 6);
    let array = [1, 2, 3, 4, 5, 6, 7, 89, 0];
    for element in array {
        let result = anothor_function(element, 1);
        println!("{}", result);
    }

    let a = loop {
        if x > 100000 {
            break x;
        } else {
            x = anothor_function(x, 1);
        }
    };
    println!("{}", a);
}

fn anothor_function(x: i64, y: i64) -> i64 {
    let z = {
        let a = x + y;
        let b = a + 1;
        let c = five();
        five_plus(b + c)
    };
    z
}

fn five() -> i64 {
    1
}
fn five_plus(x: i64) -> i64 {
    x + 2
}
