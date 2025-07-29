# Enums

Rust allows you to define types called "enums" which enumerate possible values.
Enums are a feature in many languages, but their capabilities differ in each language. Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
Useful in combination with enums is Rust's "pattern matching" facility, which makes it easy to run different code for different values of an enumeration.

## Further information

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Pattern syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)

there are two kind of ip addressses v4,v6

enum IpAddrKind {
    V4,
    V6,
}
IpAddrKind is now a custom data type that we can use elsewhere in our code.

Enum Values
We can create instances of each of the two variants of IpAddrKind like this:

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

Enums can be passed to functions:

fn route(ip_kind: IpAddrKind) {}

we can call this function with either variant:


    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
