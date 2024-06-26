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
    // let mut s1 = String::from("hello");
    // len_string(&mut s1); // &就是引用
    // println!(" s1: {}", s1);

    // slice的运用
    let mut s = String::from("hello world");
    let word = slice_world(&s);
    println!("word: {}", word);
    let a = [1, 2, 3, 4, 5];
    let person = Person { name: s, age: a };
    println!("{:?}", person);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: [u32; 5],
}
fn len_string(s: &mut String) -> &String {
    s.push_str(", world");
    s
}

fn slice_world(s: &str) -> &str {
    // 将&String改为&str 不在局限于传String，还可以传切片
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 在Rust中，通过在字节前加上b前缀，可以创建字节字面值
            return &s[..i];
        }
    }

    &s[..]
}
