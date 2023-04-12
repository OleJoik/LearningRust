# Collections

## Vectors

Storing multiple values in a single data structure that puts the values next to each other in heap memory.

### Creating a vector

```rust
let v: Vec<i32> = Vec::new();

// or use the macro

let v = vec![1, 2, 3];
```

### Adding/reading elements

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

### Storing multiple types in a Vector: Use an enum!

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
