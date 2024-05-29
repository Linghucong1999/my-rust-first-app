use std::io;
fn main() {
    let a = [3, 1, 2, 4, 5, 7, 6];
    println!("输入访问的元素索引：");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("读取失败");
    let index: usize = index.trim().parse().expect("输入错误");
    let element = a[index];
    println!("访问的元素的索引是{}，元素是{}",index, element);
}
