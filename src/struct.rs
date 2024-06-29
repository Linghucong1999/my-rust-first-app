#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct AlwaysEqual; // 类单元结构体
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    // 定义实列化结构体

    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        sign_in_count: 1,
        ..user
    };
    let user3 = build_user(user2.email, user2.username);
    println!("{:?}", user.sign_in_count);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rectangle_area = area(&rect);
    println!("rectangle area: {}", rect.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        active: (true),
        username: (username),
        email: (email),
        sign_in_count: (1),
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
