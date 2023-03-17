struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    //Simple Struct Concept
    let mut user1: User = User {
        email: String::from("asim@xord.com"),
        username: String::from("Asim"),
        active: true,
        sign_in_count: 1,
    };

    let name: String = user1.username;
    user1.username = String::from("AsimRaza");

    let user2: User = build_user(String::from("asimraza243@gmail.com"), String::from("Asim"));

    let user3: User = User {
        email: String::from("asimraza2016@gmail.com"),
        username: String::from("SyedAsim"),
        ..user2
    };

    println!("user3 username: {}", user3.username);

    //Tuple Struct Concept
    // struct Color(
    //     i32, i32, i32
    // );

    // struct Point(
    //     i32,i32,i32
    // );

    // let width1 = 30;
    // let height1 = 50;
    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     area(width1, height1)
    // )

    // replace with best practices
    // let rect = (30, 50); // more approach exist
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels", area(rect))
}

//dimensions: (i32, i32)
fn area(rectangle: Rectangle) -> i32 {
    // dimensions.0 * dimensions.1
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
