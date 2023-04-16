# Error Handling

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

##
