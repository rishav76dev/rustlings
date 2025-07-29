struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u32,
    green: u32,
    blue: u32
}

struct ColorTupleStruct(/* TODO: Add the fields that the test `tuple_structs` expects */
u32, u32, u32);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}
//To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
//  user struct instance
// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
// // }

// ; Rust doesn’t allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.


// ✅ What is Field Init Shorthand?
// Field Init Shorthand in Rust is a syntax shortcut you can use when:

// You're creating a struct

// And the field names in the struct match the names of your variables (or parameters)

// Instead of writing this:

// rust
// Copy
// Edit
// User {
//     username: username,
//     email: email,
// }
// You can just write:

// rust
// Copy
// Edit
// User {
//     username,
//     email,
// }
// Rust automatically understands that the field username should take the value from the variable username, and so on.

// 🎯 Why is it used?
// Purpose: To reduce repetition and make your code cleaner and easier to read.

// 🔁 Without shorthand (repetitive):
// rust
// Copy
// Edit
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }
// Here, username: username is repetitive. You're repeating the variable name twice.

// ✅ With shorthand (cleaner):
// rust
// Copy
// Edit
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,  // understood as username: username
//         email,     // understood as email: email
//         sign_in_count: 1,
//     }
// }
// Now the intent is the same, but the code is shorter and easier to maintain.

// 🧠 When can you use it?
// You can use field init shorthand only when the struct field name matches the variable name exactly.

// Example:

// rust
// Copy
// Edit
// let name = String::from("Alice");
// let user = User {
//     username: name, // ❌ name != username → can't use shorthand
//     ...
// };
// This is not allowed because name and username are different names

//  2. Struct Update Syntax
// Sometimes, you want to create a new struct that’s mostly the same as another one, but with a few changed fields.

// 🔸 Without update syntax:

// rust
// Copy
// Edit
// let user2 = User {
//     active: user1.active,
//     username: user1.username,
//     email: String::from("another@example.com"),
//     sign_in_count: user1.sign_in_count,
// };
// This works but is verbose. Instead, Rust gives you a shortcut:

// 🔸 With struct update syntax (..user1):

// rust
// Copy
// Edit
// let user2 = User {
//     email: String::from("another@example.com"),
//     ..user1
// };
// This means:

// email is set to a new value.

// All other fields (active, username, sign_in_count) are copied (or moved) from user1.

// 🔸 ..user1 must go last because it fills in remaining fields.

// ⚠️ Ownership and Move Behavior
// Rust tracks ownership of data, so when you use ..user1, you need to be careful.

// In this example:

// rust
// Copy
// Edit
// let user2 = User {
//     email: String::from("another@example.com"),
//     ..user1
// };
// The username field (a String) is moved from user1 to user2. After this line:

// ❌ You can’t use user1.username anymore (it's moved).

// ✅ But you can still use user1.email (because it wasn’t moved).

// ✅ You can also use user1.active and user1.sign_in_count again only if they implement the Copy trait, which primitive types like bool and u64 do.

// So, if your struct contains both Copy and non-Copy fields (like String), only the non-Copy fields will be moved when using ..user1.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        // let green =

        let green = ColorRegularStruct { red: 0, green: 255, blue: 0};
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        // let green =
        let green = ColorTupleStruct(0, 255, 0);
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        // let unit_struct =
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
