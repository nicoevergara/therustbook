## Chapter 6

**Enums** are types that have multiple variants associated to the `enum` type.

Example:

```rust
enum IpAddr {
	V4(u8, u8, u8, u8),
	V8(String),
}
```

Rust also has no `null`, rather using a built in `Option<T>` type with the variants `Some(T)` and `None`, better encapsulating the absense of a value.

Definition:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Rust also have the feature `match`, which is used to implement pattern matching as a manner of control flow construction.

Example:

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
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

A great benefit about `match` versus other control flow constructs is that is checks that all arms within the `match` cover the associated type being match against. So in the above example, you can't omit any of the `Coin` arms in the `match`, otherwise it will throw an error. Put simply, `match` statements are _exhaustive_.

Two additional features of `enum` statements are using a final arm with a variable as a catch-all for all non-specified cases and the `_` placeholder, which will not bind the value to a variable to be used within the arm, which can be helpful in the case that we don't actually need the value that would have been bound because it won't be used in the arm's definition.

**if let** is another statement that allows for pattern matching in select cases where the entire list of variants don't need to be covered and we only care about matching and acting on a single variant.

Example:

```rust
let config_max = Some(3u8);
  if let Some(max) = config_max {
      println!("The maximum is configured to be {max}");
  }
``` 

In the example, we only care about the `Some(u8)` case and don't need to act on the `None` variant. Though less verbose, it will not provide the same exhaustive type checking that `match` does, some it's something to consider when using it.

There is also the `let`-`else` pattern that allows for a nice form of short-circuiting when we find a value we don't want to continue with.

Example:

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```