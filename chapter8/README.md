## Chapter 8

**Vectors (Vec<T>)** are collections that store items on the heap, are of variable length, and only store one type per instance. There is also a built in macro for creating vectors, ex. `vec![1, 2, 3]`, which will infer the type of the values `i32` and create a `Vec<i32>`.

**Memory storage location**: Heap

To access values within a vector, they can be retrieved from a vector `v` either by the indexing syntax, `&v[2]`, or by the `get` method, `v.get(2)`. When trying to access a value in a vector with an index beyond the last item in the vector, the indexing syntax will cause the program to panic.

On the point of `Vec<T>` only being able to store items of the same type `T`, a way to store multiple variants of the same type is to use enums.

**Strings** are also collections of bytes within Rust, similar to other collection types but with some subtle differences, such as indexing. Rust only contains one string type in the core of its language, the `str` string slice primitive, which is stored in the resultant binary, though `String` and `str` are often referred to as strings in Rust.

**Memory storage location**: Heap

All strings, both `String` and `str`, are UTF-8 encoded in Rust as well, allowing for the embedding of UTF-8 encoded values in the code directly. This also means that within a `String`, we can't index to access a character because some UTF-8 encoded characters are denoted used Unicode scalar values, which can take multiple bytes instead of just 1 byte, affecting the ability to index on them. 

Slicing strings to get a subset of the bytes within a string slice is possible, such as the following:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

This would return `Зд` but a slice such as `&hello[0..1]` would fail because the letters stored in this string are represented using 2 bytes so fetching only 1 would cause the program to panic at runtime. If we wanted to iterate over the characters within a `String`, we could grab the Unicode scalar values using the `.chars()` method that is iterable. If we wanted to grab the bytes, we could use the `.bytes()` method.

**Hash Maps** are another type of common collections within Rust, written as `HashMap<K, V>`, with `K` being the type of the key and `V` being the type of the stored value. It needs to be brought into scope with `use std::collections::HashMap;` because it's not as commonly used, therefore it's not included in the prelude.

**Memory storage location**: Heap

Similar to `Vec<T>`, `HashMap<K,V>` as homogenous and cannot have mixed types in the collection.

Also similar to vectors, hash maps can be iterated over:

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

When inserting values into a hash map using `.insert(K, V)`, the hash map takes ownership of the values or creates a copy if they've implemented the `Copy` trait. References can also be inserted but the lifetime of the references will need to be valid through the entire lifetime of the hash map.

`.insert(K, V)` will always insert, regardless if the key-value pair exists already or not, but there is also another API for checking if the value exists before inserting, `.entry(key).or_insert(value)`, which will only insert the key-value pair if the key does _not_ already exist within the hash map.

The underlying hasher can be changed too, with the default hashing function being `SipHash`, which is DoS attack resistent.
