fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("GD15"),
        email: String::from("gerardohuerta1502@gmail.com"),
        sign_in_count: 1,
    };
    user1.username = String::from("0218201@up.edu.mx");
    let user2 = build_user(String::from("1234@fjasf.com"), String::from("dmi"));
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("ukfñja"),
        sign_in_count: user1.sign_in_count,
    };
    let user4 = User {
        email: String::from("djfakjf"),
        ..user2
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
