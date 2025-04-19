## Chapter 3 Notes

**Functions** use snakecasing, just like variables.

_Example:_

```rust
fn main() {
    println!("Hello, world!");

    print_labeled_measurement(10, 'm');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

**Parameters** are also defined similar to other languages. The types of parameters _must_ be defined.

**Statements** and **Expressions** are distinct in that **statements** are instructions that _do not_ return a value, while **expressions** evaluate to a resultant value which can optionally be bound to a variable.

> Assignment does not return the assigned value.

**Expressions** don't include a semi-colon, which will denote whether or not to return a value, as shown below:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // <- the lack of semi-colon makes this line an expression and will return x + 1 to y
    };

    println!("The value of y is: {y}");
}
```

**Function return values** are denoted like the following function `five`:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

Function definitions whose final line contains a statement will return _unit_, represented as `()` in Rust, similar to other languages.

`if`s are a form of **Control flow** in Rust. Portions of code associated with an `if` are sometimes referred to as _arms_, similar to `match` expressions.

Something to note, in `if`s, falsy and truthy values are not automatically converted to `bool`s like they are in other languages, an explicit conversion is need.

The exercises from the end of the chapter are in the project folder.