#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct AlwaysEqual; // 类单元结构体
fn main() {
    // 定义实列化结构体

    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: (String::from("123@qq.com")),
        ..user
    };

    println!("{:?}", user.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: (true),
        username: (username),
        email: (email),
        sign_in_count: (1),
    }
}
