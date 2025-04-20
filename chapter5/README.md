## Chapter 5

**Structs** are custom data types that allow the bundling of fields into a single type.

```rust
struct Book {
	title: String,
	author_name: String,
	pages: i32,
}
```

When marking a `struct` as mutable, the entire `struct` must be made mutable. In Rust, specific fields on a `struct` cannot be made mutable while leaving other immutable.

A `struct` can be created from a previously defined one by using the update syntax:

```rust
let book1 = Book {
	title: String::from("The Book 2"),
	author_name: String::from("Person Guy"),
	pages: 100,
};

let book2 = Book {
	title: String::from("The Book 2"),
	pages: 100,
	..book1
};
```

> Note: Any value whose type _does not_ implement the `Copy` trait will have its value _moved_ when using the update syntax. So in the above example, `author_name` from `book1` will have moved ownership to `book2` and can no longer be accessed, though `title` still can be.

**Tuple structs** are also similar to _structs_ in that they allow for the naming of a particular type to be associated with a _tuple_, but in cases where explicit field names are not necessary.

```rust
struct RGB(u8, u8, u8);

fn main() {
	let black = RGB(0, 0, 0);
}
```

_Structs_ without fields can be implement as well, known as _unit-like structs_.

```rust
struct UnitLikeStruct;
```

**Methods** for _structs_ are defined using _impl_ blocks. **Associated functions** can also be defined for a _struct_ in the case that a reference to _self_ is not needed.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

		fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

`Self` is an implicit type associated with the _struct_ that the _impl_ block is for.