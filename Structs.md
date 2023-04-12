# Struct example

## Creating a struct

```rust
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64
}
```

## Using the struct

```rust
let user = User {
  active: true,
  username: String::from("someusername123"),
  email: String::from("someone@example.com"),
  sign_in_count: 1
}
```

## Creating a new user

```rust
let user2 = User {
  email: String::from("another@example.com"),
  ..user
}
```

## Tuple structs

```rust
struct Color(i32, i32, i32);

fn main() {
  let black = Color(0, 0, 0);
}
```

## Unit-Like Structs

Might be used to implement a trait on some type.

```rust
struct AlwaysEqual;

fn main() {
  let subject = AlwaysEqual;
}
```
