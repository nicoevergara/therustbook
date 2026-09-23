## Chapter 13

**Closures** are one of the many functional programming concepts that are within Rust. When capturing the local environment, such as an immutable reference to `Self`, we can do so with closures while not being able to with a regular function.

Example:

```rust
some_option.unwrap_or_else(|| self.default_value_of_same_type());

let identity_closure = | value: usize | { value };

// without curly braces for a single line
let identity_closure_clean = | value: usize | value;
```

Note: type annotations within closures are often optional but can be added for additional explicitness. They're also required if there is no evaluation of the closures since the types won't be able to be inferred without it and closures can only have one inferred type so multiple invocations of a closure by different types will result in a compiler error.

Closures can borrow immutably, mutably, and take ownership of values within their environment. To take ownership, the `move` keyword is used, which is often done when passing a closure to a new thread, such as the example below:

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```

**Closure traits** are implemented on the closures automatically depending on the way they are set up.

The following are the 3 traits that can be implemented on a closure (from the book directly):

- **FnOnce** applies to closures that can be called once. All closures implement at least this trait because all closures can be called. A closure that `moves` captured values out of its body will only implement `FnOnce` and none of the other `Fn` traits because it can only be called once.
- **FnMut** applies to closures that don’t move captured values out of their body but might mutate the captured values. These closures can be called more than once.
- **Fn** applies to closures that don’t move captured values out of their body and don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

**Iterators**
