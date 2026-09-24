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

**Iterators** in Rust are the same as those in other languages, allowing for a sequence of items to be iterated over while performing a particular action.

One distinction in Rust is that they are _lazy by default_, meaning they won't have an effect until a method is called to consume the iterator.

The `Iterator` trait is defined in the standard library as the following (from the Rust book):

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

To create the iterator over a collection that implements the `Iterator` trait, we can use `.iter()` to get an iterator that provides _immutable references_, `.iter_mut()` to get one that iterates over _mutable references_, and `.iter_into()` to get one that provides _owned values_.

Methods that call the `next` method, as shown above and that we need to implement, are called _consuming adapters_ because calling them consumes / uses up the iterator.

Methods that don't consume the iterator but produce iterators based on aspects of the original iterator are called _iterator adapters_, such as the `map` method.

To consume the iterator after calling an _iterator adapter_, we need to call `.consume()` due to the fact that iterators are lazy.

**Performance of loops versus iterators** are negligibly different. Iterators are considered a _zero-cost abstraction_, meaning that they will boil down to the same code once compiled as would be generated for a `for` loop.

