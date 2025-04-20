## Chapter 4

**Ownership** is the most unique feature of Rust, denoting the set of rules that govern Rust program memory management.

Some of the patterns of memory management with a programming language are:

1. A Garbage Collector, automatically allocating and deallocating memory on behalf of the program.

2. Manually or explicit memory management done by the programmer on behalf of the program.

3. Rust's method, the **ownership model**, which is done through a system of rules that govern the allocation and deallocation of memory that are then checked by the compiler at compile time.

Checking at compile time allows us to catch the possible errors _before_ running a program so that there is no overhead at runtime, meaning there are no effects on the runtime performance due to these checks.

Rust's **Ownership Rules** are as follows:

1. Each value in Rust has an _owner_.
2. There can be **only one** _owner_ at a time.
3. When the _owner_ goes out of scope, the value will be dropped.

When a variable for a non-literal typed value is defined within a particular scope, memory is allocated to it for the duration of that scope. Once that scope ends, Rust will automatically call `drop`, which will deallocate the memory used by that variable.

> Note: **Resource Acquisition Is Initialization (RAII)** is a pattern that is used within C++, which is the pattern of deallocating resources at the end of an item’s lifetime.

The following is an example of a `String` represented in memory on the heap:

```rust
let s1 = String::from("hello");
```

![simple string memory allocation](./assets/simple-string-memory-allocation.svg)

This next diagram is an example of "copying" data from one item stored on on the heap to another.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

![simple string memory copying](./assets/simple-string-memory-copying.svg)

To prevent errors of double-freeing memory, such as could hypothetically happen in the above example, when `s2` is assigned to `s1`, instead of performing a copy, Rust will invalidate `s1` in favor of `s2`. This is known as a **move**.

Rust will also never automatically create a deep-copy of data.

As for **scoping** and **assignment**, Rust will deallocate any data (that is stored on the heap) from a variable immediately after that variable is assigned to a new value.

Example:
```rust
let mut s = String::from("hello");
s = String::from("ahoy");

println!("{s}, world!");
```

> Note: In the case of values stored on the stack, such as values whose size is known as run time, this will not happen.

**Functions** also will _move_ variables used as arguments under the ownership of the function. **References** are used to point to the _value_ of the variables passed into the function instead of transfering complete ownership with as with a pointer. They also guarantee to point to a valid value of a particular type for the lifetime of the reference.

Below is an example and diagram of references being used for a `String`:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

![reference example](./assets/reference-example.svg)

The act of creating a reference is known as **borrowing**. Though references are _immutable_ by default, using a **mutable reference** allows for the same ownership guarantees as before, but allows us to mutable the value we borrow. The only difference here is that a variable can only have _one mutable reference at a time_. So the following code would cause a compiling error:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

These limitations prevent the existence of _data races_, which occur in the following scenarios:

1. Two or more pointers access the same data at the same time.
2. At least one of the pointers is being used to write to the data.
3. There’s no mechanism being used to synchronize access to the data.

Both immutable and mutable references cannot exist at the same time either for the same variable, though they can be used if the immutable references scope ends before the creation of the mutable reference.