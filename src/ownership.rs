// 认识所有权

fn main() {
    // let x = 5;
    // let mut y = x;
    // y = 3;
    // println!("x: {}, y: {}", x, y);

    // String
    // let s1 = String::from("hello");
    // let s2 = s1; // s1 作废
    // println!("s2:{}", s2);

    // let s1 = String::from("hello");
    // let s2 = s1.clone(); // 给s2 开辟一个新的空间
    // println!("s1: {}, s2: {}", s1, s2);

    // 引用和借用
    let mut s1 = String::from("hello");
    len_string(&mut s1); // &就是引用
    println!(" s1: {}", s1);
}

fn len_string(s: &mut String) -> &String {
    s.push_str(", world");
    s
}
