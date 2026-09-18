## Chapter 10

**Generics** are supported throughout Rust on features such as `struct`s, `enum`s, methods, functions, etc. It is very similar to generics in other programming languages. The standard style of generic types is `UpperCamelCase`.

Performance impact: there is no performance impact with generics in Rust when compared to using concrete types, including no impact at runtime. This is done by Rust's use of monomorphisation, which is the process of filling in the generic code with the applicable concrete types at compile time.

**Traits** define the functionality that a particular type can have. In the case of generics, we can use **trait bounds** to specify the set of behaviour a particular types for a generic must have. 

To implement a particular trait for a type, we must use the `impl ... for ...` keywords, such as `impl Print for Document`. One limitation on the implementation of traits is that within a particular library, we cannot implement an external trait on an external type, such as `impl Display for Vec<t>` because of a property called _coherence_ (also known as the _orphan rule_), which prevents two conflicting definitions from two libraries from clashing.

For traits where _we do want_ a default definition, we can provide a default method implementation on a trait that allows for the optional overriding of the default definition.

Traits can also be used as a parameter's type, enforcing that not a particular type but _a particular trait is implemented on the parameter's type_.

Example:

```rust
fn some_function(param: &impl SomeTrait) -> ...
```

**Trait bounds** allow for the restricting of types for a particular generic item. The above is an example of trait bounds with a shorthand syntax, with the following being the more verbose version:

```rust
fn some_function<T: SomeTrait>(param: &T) -> ...
```

The verbose syntax allow for the constraining of a generics types across all params if used for multiple in the same function signature, why the shorthand version enforces that the types implement the trait, but _not that the parameters are all the same type_.

Example:

```rust
// Shorthand syntax
//
// Enforces that param1 and param2 have the same trait SomeTrait implemented, but not
// that they're the same type.
fn some_longer_function(param1: &impl SomeTrait, param2: &impl SomeTrait) -> ...

// Verbose syntax
//
// Enforces that param1 and param2 are the same type that implements SomeTrait
fn some_longer_function<T: SomeTrait>(param1: &T, param2: &T) -> ...
```

Multiple traits can be enforced as well with the use of multiple trait bounds, such as the following:

```rust
// Shorthand
fn some_function(param: &(impl SomeTrait + AnotherTrait)) -> ...

// Verbose
fn some_function<T: SomeTrait + AnotherTrait>(param: &T)-> ...
```

Another way to define these trait bounds is by using the `where` keyword to define the trait bounds at the end of a function. The following is an example:

```rust
// Verbose syntax without where
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// Trait bounds with where
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

Trait bounds can also be used in the return type of a function too, such as the following example:

```rust
fn some_function() -> impl SomeTrait {
```

_Note: only one type can be returned when the return type is trait bound_

Traits can also be conditionally implemented for any type that implements another type. These are called **blanket implementations**, such as this example from the standard library:

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

**Lifetimes** are similar to generic types in the sense that they define bounds, but lifetimes specify the duration during which a particular reference is valid, not the type of the reference.

The goal of lifetimes is to _prevent dangling references_ where a reference to a particular set of data is no longer in scope and, therefore, no longer available and whose memory has been deallocated.

The Rust **borrow checker** is what checks and enforces the validity of references within a given program. Below is an simple example from the book demostrating the lifetimes and the given scopes:

```rust
// This program will not compile
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```

We can see how the lifetime of `r`, denoted with the lifetime `'a`, is in the outer scope and the lifetime of `x`, denoted with the lifetime `'b`, is the inner scope. Once the inner scope is left, the reference `&'b x` is no longer valid since it doesn't live past the inner scope.

A version of the above program that _will_ compile is as follows:

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+
```

**Generic lifetimes in functions** are used to specify the lifetime of the parameters and the return value when it's not able to be determined by the compiler. The following is an example of a function that needs lifetimes for it to compile successfully:

```rust
// The program will not compile since the lifetimes are ambiguous
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

Since the return value's lifetime depends on which of the two params are being returned, which belong to different scopes within the function, the lifetimes need to be defined on the function so the compiler knows how we intended the references relationship to be:

```rust
// Now the program will compile with the lifetimes defined
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The lifetime annotations don't defined how long the references live, rather they define the relationship expected between multiple lifetimes. The above lifetime annotations for `longest` are denoting that the references of `x`, `y`, and the returned value are all expected to live as long as lifetime `'a`. This means that if the lifetime of `x` is shorter / smaller than that of `y`, then `'a` can live for only as long as the shorter of the two lifetimes, being the lifetime of `x` in this case.

When defining the lifetime parameter for a return type, it _must_ also include a matching lifetime parameter on one of the function's parameters. If this is not done and it were allowed by the compiler, a dangling reference would be created because the returned reference would go out of scope at the end of the function. In those cases, it's best to return an owned data type so that the calling function can take ownership and be responsible for its cleanup.

**Generic lifetimes in structs** can also be used to define the relationship between the references within a `struct`, where a lifetime annotation would be necessary to denote the lifetime of the reference(s) within a `struct`.

Example:

```rust
struct Message<'a> {
    content: &'a str,
}
```

The above denotes that an instance of `Message` must not live longer than the reference it holds in `content`.

**Lifetime elision rules** are rules baked into the the Rust compiler's analysis that can determine the lifetime of functions in a particular subset of situations, allowing for us to omit the lifetime annotations that would have otherwise been required.

**Lifetime inputs** are the lifetime annotations on functions and methods and **lifetime outputs** are the lifetime annotations on return values.

The static lifetime `'static` denotes that a lifetime of a reference can live for the _entire duration of the program_.
