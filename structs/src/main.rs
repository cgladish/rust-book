struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color (u32, u32, u32);
struct Point (u32, u32, u32);

struct UnitStruct;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("cgladish"),
        email: String::from("chase@example.com"),
        sign_in_count: 111
    };

    user1.username = String::from("cgladish2");

    let mut user2 = create_new_user(String::from("chase2@example.com"), String::from("cgladish2"));

    let mut user3 = User {
        sign_in_count: 2,
        ..user2
    };

    user2.email = String::from("cgladish3");

    let user2_email = user2.email;
    println!("User2 email {user2_email}");

    let color = Color(1, 2, 3);
    let point = Point(4, 5, 6);

    let Point(x, y, z) = point;

    let unit = UnitStruct;
}

fn create_new_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}