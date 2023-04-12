# Enums

- Definition
- Encode meaning along with data
- The `Option` enum
- Pattern matching (`match`-statement)
- `if let` construct

# Definition

Enums give a way of saying a value is one of a possible set of values.

For example, `Rectangle`, `Circle` and `Triangle` might be "possible shapes", encoded in enums.

## Defining an IP address enum

The IP Address can be either of version 4 or version 6.

```rust
enum IpAddrKind {
  V4,
  V6
}
```

## Creating instances of each of the variants

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

# Enums with associated values, meaning with data

```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

## Using structs as the type of an enum

```rust
struct Ipv4Addr {
  // --snip--
}

struct Ipv6Addr {
  // --snip--
}

enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}
```

## Message type example

```rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
```

## Implementing methods on an enum

```rust
impl Message {
    fn call(&self) {
        // method body
    }
}

let m = Message::Write(String::from("Hello"));
m.call()
```

## Pattern matching an enum

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

# Matching with Option<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

## Matching any without binding to the value

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // Unit value, empty type
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

# `if let` Control flow

A tool you might use to "Pick out" an arm of a match statement, and not run any other case. It isn't exhaustive, so doesn't check every option then.

These two examples are equivalent

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
