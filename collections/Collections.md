# Collections

# Vectors

Storing multiple values in a single data structure that puts the values next to each other in heap memory.

## Creating a vector

```rust
let v: Vec<i32> = Vec::new();

// or use the macro

let v = vec![1, 2, 3];
```

## Adding/reading elements

Note: Vector must be mutable!

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);

v.pop(); // Removes and returns the last element

let does_not_exist = &v[100]; // May cause a panic
let does_not_exist = v.get(100); // returns Option<&T>, handle Some(&element) and None

// iterating a vector
for i in &v {
  println!("{i}")
}

// iteration with mutable references
for i in &mut v {
  *i =+ 50; // The * dereference operator gets the value in the reference i
}
```

## Storing multiple types in a Vector: Use an enum!

(Another alternative might be to use a trait object, coming back to those..!)

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

# Strings

## Creating anew

```rust
let mut s1 = String::new();

let data = "initial contents";
let s2 = data.to_string();
let s2 = "initial contents".to_string();

let s3 = String::from("initial contents");
```

## Indexing

Strings CANNOT be indexed with numbers such as `&"Здравствуйте"[0]`

Instead use a string slice:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4]; // Зд
```

But this code will not return "what you think"... Utf-8 encoding is weird.

Getting a specific character might be done with `nth` of `chars()`

```rust
let s = "hello";
let third_character = s.chars().nth(2);
```

## Counting

Use `s.chars().count()`. Note that this might behave different than expected with 'precomposed characters':

```rust
println!("{}", "é".chars().count()); // 2
println!("{}", "é".chars().count()); // 1
println!("{}", "é".graphemes(true).count()); // 1
println!("{}", "é".graphemes(true).count()); // 1
```

## Updating

```rust
let mut s = String::from("foo");

// append a string slice
s.push_str("bar");

// append a single character
s.push("l");
```

## Concatenation

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("World!");

let s3 = s1 + &s2;

// // // //
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}"); // Uses references, creates new string
let s = s1 + "-" + &s2 + "-" + &s3; // Takes ownership of s1, while s2 and s3 stays valid (deref coercion)
```

## Iterating

```rust
for c in "Зд".chars() {
    println!("{c}");
}

// З
// д

for b in "Зд".bytes() {
    println!("{b}");
}

// 208
// 151
// 208
// 180
```

## Other useful string methods

`contains` searches a string.

`replace` substitutes part of a string with another.
