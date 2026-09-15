## Chapter 9

**Errors** in Rust are categorized into two types:
- recoverable
  - errors we want to handle and continue running the program 
  - `Result<T, E>`
- unrecoverable
  - errors that result in the immediate termination of the program
  - `panic!` 

**Unrecoverable errors** are typically the result of `panic!` macro uses. When this occurs, the stack is _unwound_, meaning that Rust will walk up the function call stack and free up any memory that was used. This is the default behavior, but this can be simplified to a basic abort if need be within the `Cargo.toml` by setting:

```toml
[profile.release]
panic = 'abort'
```

To get the full backtrace of the error thrown by a `panic!`, we can set the environment variable `RUST_BACKTRACE`.

**Recoverable errors** will most often be contained with a `Result<T, E>` type that is returned from a function call that could fail, with `T` being the type of the successfully returned value and `E` being the type of the error returned upon failure.

**Ways to handle errors**
- `.unwrap()` - this method will pull out `T` from `Result<T, E>` if the result is successful and will return `E` if not
- `.expect(<reason>)` - this method will do the same as `.unwrap()` but will replace the default error message from `panic!` with the reason provided, allowing for easier debugging and tracing
- `?` - this operator is similar to `.unwrap()` expect it will call `impl From<EFromResult> for ErrorTypeOnCaller` to convert the error to the type defined on the caller's function signature's return type

**When to call `panic!` or return `Result`**

Call `panic!` when:
- Writing examples, prototypes, or tests
- A program runs into a bad, invalid, or unexpected state
  - This is especially important when a particular state that is reached is harmful or insecure
-

Return `Result` when:
- A error is possible, but recoverable or expected, such as a parsing error or an network connection error

Call `.expect()` when:
- The compiler isn't able to determine that a `Result` won't give an `Err` but we're able to ourselves

Example:

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
  .parse()
  .expect("Hardcoded IP address should be valid");
```


