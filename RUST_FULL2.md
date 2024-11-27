# Struct To Structure Relative Data:

Structs are similar to tuples, in that both hold multiple related values.
Unlike with tuples, you have to name each piece of data so it's clear what values mean.
It also makes it more flexible than tuples.

- Defining:
```rs
struct User{
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}
```

- Using the Struct:
```rs
let mut user1 = User{
    active: true,
    username: String::from("someuser123"),
    email: String::from("some@example.co"),
    sign_in_count: 1,
};
```
> Note that entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable.

- Another use way (As expression in function):

Because the field name and value name is same, we only need to write the value name.
> Field name and value name must be same
```rs
fn build_user(email: String, username: String) -> User{
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

- Instance from other instance

This is convenient but remember data is moved. Struct update syntax uses **=** like an assignment; this is because it moved data.
So, we cannot use user1 after moving data to user2 and user2 after transfering to user3 and so on...

If active and sign_in_count was only used from user1, it would be exist beyond user1 as stack data are copied due to **copy** trait.
```rs
fn main() {
    // --snip--
    let user1 = build_user(String::from("mail@..."), String::from("Name..")); // User1 exits

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    }; // User1 moved to user2

    let user3 = User{
        email: String::from("mail2@..."),
        ..user2
    }; // User2 data moved to user3

    // user3 only exits...

} //user3 doesn't exist no more
```


# Tuple Structs without named fields to create different types
```rs
struct color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main(){
    let black = color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

# Unit-Like Structs without fields

This is useful to implement a trait on some tyep but don't have any data that you want to store.
```rs
struct AlwaysEqual;

fn main(){
    let subject = AlwaysEqual
}
```

# Ownership of Struct Data

The following code won't work. Why? Because Rust requires that each instance of struct own all of its data and that data be valid for as long as
struct exists.

It is possible for structs to store references to data owned by something else but to do so requires use of lifetimes.
```rs
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```


We use stucts for its convenience
