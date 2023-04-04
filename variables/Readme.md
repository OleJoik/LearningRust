# Variables and Mutability

## Mutability

Variables are by default immutable, and can be made mutable with the `mut` keyword when declared.

- immutable = can not change
- mutable = can be changed later

```rust
let x = 5
x = 6
// compiler error
```

```rust
let mut x = 5
x = 6
// Ok
```

## Constants

- declared with `const` keyword
- type must be annotated
- can not be mutable
- can be declared in any scope, including the 'global' scope
- can not be evaluated at runtime, but the compiler can evaluate a set of [limited operations](https://doc.rust-lang.org/reference/const_eval.html).

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## Shadowing

- declaring a new variable with the same name as a previous variable
- often used to rename variables
- 'overshadows' the value of the previous variable until the second variable goes out of scope.

```rust
let spaces = "   ";
let spaces = spaces.len();
```

## Data types: Scalar vs Compound

Scalar types are single values of a primary scalar type:

- integers
- floats
- booleans
- characters

Compound types groups multiple values into one type. Rust primitive compounds are:

- tuples
- arrays

## Scalar data types

### Integers

Can ble split into two groups:

- 'Signed' `i` can be negative.
- 'Unsigned' `u` can only be positive.

| Length  | Signed (+/-) | Unsigned (+) |
| :-----: | :----------: | :----------: |
|  8-bit  |      i8      |      u8      |
| 16-bit  |     i16      |     u16      |
| 32-bit  |     i32      |     u32      |
| 64-bit  |     i64      |     u64      |
| 128-bit |     i128     |     u128     |
|  arch   |    isize     |    usize     |

Where `isize` and `usize` depends on the architecture of the computer the program is running on. They are efficient when indexing some sort of collection.

### Floats

Rust has two primitive types for float, `f32` and `f64`. f64 is the default and has double precision.

### Basic numeric operations

```rust
// addition
let sum = 5 + 10;

// subtraction
let difference = 95.5 - 4.3;

// multiplication
let product = 4 * 30;

// division
let quotient = 56.7 / 32.2;
let truncated = -5 / 3; // Results in -1

// remainder
let remainder = 43 % 5;
```

All other operators of rust can be found in [Appendix B: Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)

### Boolean

```rust
let t = true;

let f: bool = false; // with explicit type annotation
```

### Character

The `char` type can hold any unicode scalar value

Important note: Chars are defined inside single quotes, such as `'C'`, strings have double quotes `"string"`

- accented letters etc.
- emojis

```rust
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
```

## Compound types

### Tuples

- instantiated as (comma, seperated, list)
- has fixed length, once declared, cannot grow or shrink.
- types can optionally be annotated:
  ```rust
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  ```
- individual values can be extracted by pattern matching

  ```rust
  let (x, y, z) = tup
  ```

- tuple elements can be accessed directly with dot syntax:
  ```rust
  let five_hundred = tup.0
  let one = tup.2
  ```

### Arrays

- Allocates data to the stack instead of the heap.
- Has a fixed number of elements. (Vectors are more flexible..!)
- All elements are of the same type.

Annotating the type of an array:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Quicker initialization to a standard value:

```rust
let b = [3; 5]; // a = [3, 3, 3, 3, 3]
```

Accessing array elements

```rust
let first = a[0]
```
