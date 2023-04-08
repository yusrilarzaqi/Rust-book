# Managing Growing Projects with Packages, Creates, and Modules

As you write large programs, organizing your code will become increasingly important.
By grouping related functionality and separating code with distinct feature, you'll clarify where to find code that implements a particular feature and where to find code that implements a particular feature and where to go to change how a feature works.

The programs we've written so far have been in one module in one file.
As a project grows, you should organize code by splitting it into multiple modules and then multiple files.
A package can contain multiple binary crates and optionally one library crate.
As a package grows, you can extract parts into separate crates that become external dependencies.
This chapter covers all these techniques.
For very large projects comprising a set of interrelated packages that evolve together , Cargo provides _workspaces_, which we'll cover in the ["Cargo Workspaces"](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) section in Chapter 14.

We'll also discuss encapsulating implementation detail, which lets you reuse code at higher level: once you've implemented an operation, other code can call your code via its public interface without having to know how the implementation works.
The way you write code defines which parts are public for other code to use and which parts are private implementations details that you reserve the right to change.
This is another way to limit the amount of detail you have to keep in your head

A related concept in scope: the nested context in which code is written has a set of names that are defined as "in scope".
When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant, or other item and what that item means.
You can create scopes and change which names are in or out of scope.
You can't have two items with the same name in the same scope; tools are available to resolve name conflicts.

Rust has a number of features that allow you to manage code's organization, including which details are exposed, which details are private, and what names are what names are in each scope in your programs.
These features, somtimes collectively referred to as the _module system_, include:

- **Packages**: A cargo feature that lets you build, test, and share crates.
- **Crates**: A tree of modules that produces a library or executable.
- **Modules** and **use**: Let you control the organization, scope, and privacy of paths.
- **Paths**: A way of naming an item, such as a struct, function, or module.

In this chapter, we'll cover all these features, discuss how they interact, and explain how to use them to manage scope.
By the end, you should have a solid understanding of the module system and be able to work with scopes like a pro!

## Packages And Crates

The first parts of the module system we'll cover are packages and crates.

A _crate_ is the smallest amount of code that the Rust compiler considers at a time.
Even if you run `rustc` rather than `cargo` add pass a single source code file (as we did all they way back in the "Writing and Running a Rust Program" section of Chapter 1), the compiler considers that file to be a crate.
Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we'll see in the coming sections.

A crate can come in one of two forms: a binary crate or a library crate.
_Binary crate_ are programs you can compile to an executable that you can run, such as a command-line program or a server.
Each must have a function called `main` that defines what happens when the executable runs.
All the crates we've crated so far have been binary crates.

_Library crates_ don't have a `main` function, and they don't compile to an executable.
Instead, they define functionality intended to be shared with multiple projects.
For example, the `rand` crate we used in [Chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number) provides functionality that generates random numbers.
Most of the time when Rustaceans say "crate", they mean library crate, and they use "crate" interchangeably with the general programming concept of a "library".

The _crate root_ is a source file that the Rust compiler starts from and makes up the root module of your crate (we'll explain modules in depth in the ["Defining Modules to Control Scope and Privacy"](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) section).

A _package_ is a bundle of one or more crates that provides a set of functionality.
A package contains a _Cargo.toml_ file that describes how to build those crates.
Cargo is actually a package that contains the binary crate for the command-line tool you've been using to build your code.
The Cargo package also contains a library crate that the binary crate depends on.
Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses.

A package can contain as many binary crates as you like, but at most only one library crate.
A package must contain at least one crate, whether that's library or binary crate.

Let's walk through what happens when we create a package.
First, we enter the command `cargo new`:

```sh
cargo new my-project
```

```sh
ls my-project
```

`Cargo.toml`

```sh
ls my-project/src
```

`main.rs`

After we run `cargo new`, we use `ls` to see what Cargo creates.
In the project directory, there's a _Cargo.toml_ file, giving us a package.
There's also a _src_ directory that contains _main.rs_.
Open _Cargo.toml_ in your text editor, and note there's no mention of _src/main.rs_.
Cargo follows a convention that _src/main.rs_ is the crate root of a binary crate with the same name as the package.
A package can have multiple binary crates by placing files in the _src/bin_ directory: each file will be separate binary crate.

##
