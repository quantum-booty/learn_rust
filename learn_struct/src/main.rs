#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// use tuple struct when field names does not have to be named
struct Color(i32, i32, i32);

// unit-like structs without any fields
struct UnitLikeStruct;

fn build_user(email: String, username: String) -> User {
    // variable email has same name as struct field
    // so can use the field init shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn struct_update_syntax(user: User) -> User {
    // create new instance from other instance

    // because username is moved
    // the original user cannot be used anymore
    
    // had we only created the new User from active and sign_in_count
    // we could then still use the original user
    // because active and sign_in_count exists on the stack
    // and implements the Copy trait
    User{
        email: String::from("another@email.com"),
        ..user
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{:#?}", user1);

    let user2 = build_user(String::from("yaya@email.com"), String::from("henry"));
    println!("{:#?}", user2);

    let user3 = struct_update_syntax(user2);
    println!("{:#?}", user3);

    let black = Color(0, 0, 0);

    let unit_like_struct = UnitLikeStruct;
}
