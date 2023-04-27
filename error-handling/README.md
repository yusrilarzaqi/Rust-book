# Error Handling

<!--toc:start-->

- [Error Handling](#error-handling)
  - [Unrecoverable Errors with `panic!`](#unrecoverable-errors-with-panic)
    - [Unwinding the Stack or Aborting in Response to a Panic](#unwinding-the-stack-or-aborting-in-response-to-a-panic)
    - [Using `panic!` Backtrace](#using-panic-backtrace)
  - [](#)
  <!--toc:end-->

Error are a fact of life in software, so Rust has a number of feature for handling situations in which something goes wrong.
In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.
This requirement makes your program more robust by ensuring that you'll discover errors and handle them appropriately before you've deployed your code to production!

Rust groups errors into two major categories: _recoverable_ and _unrecoverable_ errors.
For a recoverable error, such as a _file not found_ error, we most likely just want to report the problem to the user and retry the operation.
Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.

Most languages don't distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions.
Rust doesn't have exceptions.
Instead, it has the type `Rust<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.
This chapter covers calling `panic!` first and then talks about returning `Result<T, E>` values.
Additionally, we'll explore considerations when deciding whether to try to recover from an error or to stop execution.

## Unrecoverable Errors with `panic!`

Sometimes, bad things happen in your code, and there's nothing you can do about it.
In these cases, Rust has the `panic!` macro.
There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the `panic!` macro.
In both cases, we cause a panic in our program.
By default, these panics will print a failure message, unwind, clean up the stack, and quit.
Via an environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.

---

### Unwinding the Stack or Aborting in Response to a Panic

By default, when a panic occurs, the program starts _unwinding_, which means Rust walks back up the stack and cleans up the data from each function it encounters.
However, this walking back and cleanup is a lot of work.
Rust, therefore, allows you to choose the alternative of immediately _aborting_, which ends the program without cleaning up.

Memory that program was using will then need to be cleaned up by the operating system.
If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropriate `[provile]` section in your _Cargo.toml_ file.
For example, if you want to abort on panic in release mode, add this

```toml
[profile.release]
panic = 'abort'
```

---

Let's try calling `panic!` in a simple program:

```rust
fn main() {
    panic!("crash and burns");
}
```

When you run the program, you'll see something like this:

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The call to `panic!` causes the error message contained in the last two lines.
The first line shows our panic message and the place in our source code where the panic occurred: _src/main.rs:2:5_ indicated that it's the second line, fifth character of our _src/main.rs_ file.

In this case, the line indicated is part of our code, and if we go to that line, we see the `panic!` macro call.
In other cases, the `panic!` call might be in code that our code calls, and the filename and line number reported by the error message will be someone else's code where the `panic!` macro is called, not the line of our code that eventually led to the `panic!` call.
We can use the backtrace of the functions the `panic!` call came from the figure out the part of our code that is causing the problem.
We'll discuss backtrace in more detail next.

### Using `panic!` Backtrace

Let's look at another example to see what it's like when a `panic!` all comes from a library because of a bug in our code Instead of from our code calling the macro directly.
Listing 9-1 has some code that attempts to access an index in a vector beyond the range of valid indexes.

Filename: _src/main.rs_

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

Listing 9-1: Attempting to access an element beyond the end of a vector, which will cause a call to `panic!`.

Here, we're attempting to access access the 100th element of our vector (which is at index 99 because indexing starts at zero), but the vector has only 3 elements.
In this situation, Rust will panic.
Using `[]` is supposed to return an element, but if you pass an invalid index, there's no element, but if you pass an invalid index, there's no element that Rust could return here that would be correct.

In C, attempting to read beyond the end of a data structure is undefined behavior.
You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn't belong to that structure.
This is called a _buffer overread_ and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn't be allowed to that is stored after the data structure.

To protect your program from this soft of vulnerability, if you try too read an element at an index that doesn't exist, Rust will stop execution and refuse to continue.
Let's try it and see:

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This error points at line 4 of our `main.rs` where we attempt to access index 99.
The next note line tells us that we can set the `RUST_BACKTRACE` environment variable to get a backtrace of exactly what happened to cause the error.
A _backtrace_ is a list of all the functions that have been called to get to this point.
Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote.
That's the spot where the problem originated.
The lines above that spot are code that your code has called; the lines below are code that called your code.
These before-and-after lines might include code Rust code, standard library code, or crates that you're using.
Let's try getting a backtrace by setting the `RUST_BACKTRACE` environment variable to any value except 0.
Listing 9-2 shows output similar to what you'll see.

```language
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
   2: core::panicking::panic_bounds_check
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:84:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:242:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/alloc/src/vec/mod.rs:2591:9
   6: panic::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Listing 9-2: The backtrace generated by a call to `panic!` displayed when the environment variable `RUST_BACKTRACE` is set.

That's a lot of output!
The exact output you see might be different depending on your operating system and Rust version.
In other to get backtraces with this information, debug symbols must be enabled.
Debug symbols are enabled by default when using `cargo build` or `cargo run` without `--release` flag, as we have here.

In the output in Listing 9-2, line 6 of the backtrace points to line in our project that's causing the problem: line 4 of _src/main.rs_.
If we don't want our program to panic, we should start our investigation at the location pointed to by the first line mentioning a file we wrote.
In Listing 9-1, where we deliberately wrote code that would panic, the way to fix the panic is not request an element beyond the range of the vector indexes.
When your code is taking with what values to cause the panic and what the code should do instead.

We'll come back to `panic!` and when we should not use `panic!` to handle error conditions in the ["To `panic!` or Not to `panic!`"](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic) section latter in this chapter.
Next, we'll look at how to recover from an error using `Result`.

## Recoverable Errors with `Result`

Most errors aren't serious enough to require the program to stop entirely.
Sometimes, when a function fails, it's for a reason that you an easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn't exist, you might want to create the file instead of terminating the process.

Recall from ["Handling Potential Failure with `Result`"](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result) in Chapter 2 that the `Result` enum is defined as having two variants, `Ok`, and `Err`, as follows:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `T` and `E` are generic type parameters: we'll discuss generic in more detail in Chapter 10.
What you need to know right now is that `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant.
Because `Result` has these generic type parameters, we can use the `Result` type and the functions defined on it in many different situations where the successful value and error value we want to return may differ.

Let's call a function that returns a `Return` value because the function could fail.
In Listing 9-3 we try to open a file.

Filename: _src/main.rs_

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

Listing 9-3: Opening a file

The return type of `File::open` is a `Result<T, E>`.
The generic parameter `T` has been filled in by the implementation of `File::open` with the type of the success value, `std::fs::File`, which is a file handle.
The type of `E` used in the error value is `std::io::Error`.
This return type means the call to `File::open` might succeed and return a file handle that we can read from or write to.
The function call also might fail: for example, the file might not exist, or we might not have permission to access the file.
The `File::open` function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information.
This information is exactly what the `Result` enum conveys.

In the case where `File::open` succeeds, the value in the variable `greeting_file_result` will be an instance of `Ok` that contains a file handle.
In the case where it fails, the value in `greeting_file_result` will be an instance of `Err` that contains more information about the kind of error that happened.

We need to add to the code in Listing 9-3 to take different actions depending on the value `File::open` returns.
Listing 9-4 shows one way to handle the `Result` using a basic tool, the `match` expression that we discussed in chapter 6.

Filename: _src/main.rs_

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("src/hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:#?}", error),
    };

    println!("{:#?}", greeting_file);
}
```

Listing 9-4: Using a `match` expression to handle the `Result` variants that might be returned

Note that, like the `Option` enum, the `Result` enum and its variants have been brought into scope by the prelude, so we don't need to specify `Result::` before the `Ok` and `Err` variants in the `match` arms.

When the result is `Ok`, this code will return the inner `file` value out of the `Ok` variant, and we then assign that file handle value to the variable `greeting_file`.
After the `match`, we can use the file handle for reading or writing.

The other arm of `match` handles the case where we get an `Err` value from `File::open`.
In this example, we've chosen to call the `panic!` macro.
If there's file named _hello.txt_ in our current directory and we run this code, we'll see the following output from the `panic!` macro:

```
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

As usual, this output tells us exactly what has gone wrong.

### Match on Different Errors

The code in Listing 9-4 will `panic!` no matter why `File::open` failed.
However, we want to take different actions for different failure reasons: if `File::open` failed because the file doesn't exist, we want to create the file and return the handle to the new file.
If `File::open` failed for any other reason--for example, because we didn't have permission to open the file--we still want the code to `panic!` in the same way as it did in Listing 9-4.
For this we add an inner `match` expression, shown in Listing 9-5.

Filename: _src/main.rs_

```rust
use std::{fs::File, io::ErrorKind, panic};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}",e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    println!("{:#?}", greeting_file);
}
```

Listing 9-5: Handling different kinds of errors in different ways.

The type of the value that `File::open` returns inside the `Err` variant is `io::Error`, which is a struct provided by the standard library.
This struct has a method `kind` that we can to get an `io::ErrorKind` value.
The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of errors that might result from an `io` operation.
The variant we want to use is `ErrorKind::NotFound`, which indicates the file we're trying to open doesn't exist yet.
So we math on `greeting_file_result`, but we also have an inner match on `error.kind()`.

The conditions we want to check in the inner match is whether the value returned by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum.
If it is, we try to create the file with `File::create`.
However, because `File::create` could also fail, we need a second arm in the inner `match` expression.
When the file can't be created, a different error message is printed.
The second arm of the outer `match` stays the same, so the program panics on any error besides the missing file error.

### Alternatives to Using `match` with `Result<T, E>`

That's a lot of `match`!
The `match` expression is very useful but also very much a primitive.
In Chapter 133, you'll learn about closures, which are used with many of the methods defined on `Result<T, E>`.
These methods an be more concise than using `match` when handling `Result<T, E>` values in your code.

For example, here's another way to write the same logic as shown in Listing 9-5, this time using closures and the `unwrap_or_else` method:

```rust
use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        }
        else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("{:#?}", greeting_file);
}
```

Although this code has the same behavior as Listing 9-5, it doesn't contain any `match` expressions and is cleaner to read.
Come back to this example after you've read Chapter 13, and look up the `unwrap_or_else` method in the standard library documentation.
Many more of these methods can clean up huge nested `match` expressions when you're dealing with errors.

##
