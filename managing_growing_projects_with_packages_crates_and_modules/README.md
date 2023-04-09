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

## Defining Modules to Control Scope and Privacy

In this section, we'll talk about modules and other parts of the module system, namely _paths_ that allow you to name items; the `use` keyword that brings a path into scope; and the `pub` keyword to make items public.
We'll also discuss the `as` keyword, external packages, and the glob operator.

First, we're going to start with a list of rules for easy reference when you're organizing your code in the feature.
Then we'll explain each of the rules in detail.

### Modules Cheat Sheet

Here we provide a quick reference on how modules, path, the `use` keyword, and the `pub` keyword work in the compiler, and how most developers organize their code.
We'll be going through examples of each of these rules throughout this chapter, but this is a great place to refer to as a remainder of how modules work.

- **Start from the crate root**: When compiling a crate, the compiler first look in the crate root file (usually _src/lib.rs_ for a library crate or _src/main.rs_ for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules; say, you declare a "garden" module with `mod garden;`. The compiler will look for the module's code in these place:
  - Inline, within curly brackets that replace the semicolon following `mod garden`.
  - In the file _src/garden.rs_.
  - In the file _src/garden/mod.rs_.
- **Declaring submodules**: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegatables;` in _src/garden.rs_. The compiler will look for the submodule's code within the directory named for the parent module in these places:
  - Inline, directly following `mod vegatables`, within curly brackets instead of the semicolon.
  - In the file _src/garden/vegetables.rs_.
  - In the file _src/garden/vegetables/mod.rs_.
- **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code.
  - For example, an `Aspargus` type in the garden vegetables module would be found at `crate::garden::vegetables::Aspargus`.
- **Private vs Public**: Code within a module is private from its parent modules by default.
  - To make a module public, declare it with `pub mod` instead of `mod`.
  - To make items within a public module public as well, use `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.
  - In any scope that can refer to `crate::garden::vegetables::Aspargus`, you can create a shortcut with `use crate::garden::vegetables::Aspargus;` and from then on you only need to write `Aspargus` to make use of that type in the scope.

Here we create a binary named `backyard` that ilustrates these rules.
The crate's directory, also named `backyard`, contains thse files and directory:

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

The crate root file in this case is _src/main.rs_, and it contains:

Filename: _src/main.rs_

```rust
use crate::garden::vegatables::Aspargus;

pub mod garden;

fn main() {
    let plant = Aspargus {};
    println!("I'm growing {:?}!", plant);
}
```

The `pub mod garden;` line tells the compiler to include the code it finds in _src/garden.rs_, which is:

Filename: _src/garden.rs_

```rust
pub mod vegetables;
```

Here, `pub mod vegetables;` means the code in _src/garden/vegetables.rs_ is included too.
That code is:

```rust
#[derive(Debug)]
pub struct Aspargus{}
```

Now let's get into the details of these rules and demonstrate them in action!

### Grouping Related Code in Modules

_Modules_ let use organize code within a crate for readability and easy reuse.
Modules also allow us to control the _privacy_ of items, because code within a module is private by default.
Private items are internal implementation details not available for outside use.
We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.

As an example, let's write a library crate than provides the functionality of a restaurant.
We'll define the signatures of functions but leave their bodies empty to concentrate on the organization of the code, rather than the implementation of a restaurant.

In the restaurant industry, some parts of a restaurant are referred to as _front of house_ and others as _back of house_.
Front of house is where customers are; this encompasses where the hosts seat customers, servers to take order and payment, and bartenders make drinks.
Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

To structure our crate in this way, we can organize its functions into nested modules.
Create a new library named `restaurant` by running `cargo new restaurant --lib`; then enter the code in Listing 7-1 into _src/lib.rs_ to define some modules and function signatures.
Here's the front of house section:

Filename: _src/lib.rs_

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    } /* hosting */

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    } /* serving */
} /* front_of_house */
```

Listing 7-1: A `front_of_house` module containing other modules that then contain functions.

We define a module with the `mod` keyword followed by the name of the module (in this case, `front_of_house`).
The body of the module then goes inside curly brackets.
Inside modules, we can place other modules, as in this case with the modules `hosting` and `serving`.
Modules can also hold definitions for other items, such as structs, enums, constants, traits, and as in Listing 7-1 functions.

By using modules, we can group related definitions together and name why they're related.
Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions, making it easier to find the definitions relevant to them.
Programmers adding new functionality to this code would know where to place the code to keep the program organized.

Earlier, we mentioned that _src/main.rs_ and _src/lib.rs_ are called roots.
The reason for their name is that contents of either of those files from a module named `crate` at the root of the crate's module structure, know as the _module tree_.

Listing 7-2 shows the module tree for the structure in Listing 7-1.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Listing 7-2: The module tree for the code in Listing 7-1.

This tree shows how some of the modules nest inside one another; for example, `hosting` nests inside `front_of_house`.
The tree also shows that some modules are _sibling_ to each other, meaning they're defined in the same module; `hosting` and `serving` are sibling defined within `front_of_house`.
If module A is contained inside module B, we say that module A is the _child_ of module B and that module B is the _parent_ of module A.
Notice that the entire module tree is rooted under the implicit module named `crate`.

The module tree might remind you of the filesystem's directory tree on your computer; this is a very apt comparison!
Just like directories in a filesystem, you use modules to organize your code.
And just like files in a directory, we need a way to find our modules.

## Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem.
To call a function, we need to know its path.

A path can take two forms:

- An _absolute path_ is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal `crate`.
- A _relative path_ starts from the current module and uses `self`, `super`, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifier separated by double colons (`::`).

Returning to Listing 7-1, say we want to call the `add_to_waitlist` function.
This is the same as asking: what's the path of the `add_to_waitlist` function?
Listing 7-3 contains Listing 7-1 with some of the modules and functions removed.

We'll show two ways to call the `add_to_waitlist` function from a new function `eat_at_restaurant` defined in the crate root.
These paths are correct, but there's another problem remaining that will prevent this example from compiling as-is.
We'll explain why in a bit.

The `eat_at_restaurant` function is part of our library crate's public API, so we mark it with the `pub` keyword.
In the ["Exposing Paths with the `pub` Keyword"](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword) section, we'll go into more detail about `pub`.

Filename: _src/lib.rs_

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    } /* hosting */
} /* front_of_house */

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Listing 7-3: Calling the `add_to_waitlist` function using absolute and relative paths.

The first we call the `add_to_waitlist` function in `eat_at_restaurant`, we use an absolute path.
The `add_to_waitlist` function is defined in the same crate as `eat_at_restaurant`, which means we can use the `crate` keyword to start an absolute path.
We then include each of the successive modules until we make our way to `add_to_waitlist`.
You can imagine a filesystem with the same structure: we'd specify the path `/front_of_house/hosting/add_to_waitlist` to run the `add_to_waitlist` program; using the `crate` name to start from the crate root is like using `/` to start from the filesystem root your shell.

The second time we call `add_to_waitlist` in `eat_at_restaurant`, we use a relative path.
The path starts with `front_of_house`, the name of the module defined at the same level of the module tree at `eat_at_restaurant`.
Here the filesystem equivalent would be using the path `front_of_house/hosting/add_to_waitlist`.
Starting with a module name means that the path is relative.

Choosing whether to use a relative or absolute path is a decision you'll make based on your project and depends on whether you're more likely to move item definition code separately from or together with the code that uses the item.
For example, if we move the `front_of_house` module and the `eat_at_restaurant` function into a module named `customer_experience`, we'd need to update the absolute path to `add_to_waitlist`, but the relative path would still be valid.
However, if we moved the `eat_at_restaurant` function separately into a module named `dining`, the absolute path to the `add_to_waitlist` call would stay the same, but relative path would need to be updated.
Our preference in general is to specify absolute paths because it's more likely we'll want to move code definitions and item calls independently of each other.

Let's try to compile Listing 7-3 and find out why it won;t compile yet!
The error we get is shown in Listing 7-4.

```
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

Listing 7-4: Compiler errors from building the code in Listing 7-3.

The error messages say that module `hosting` is private.
In other words, we have the correct paths for the `hosting` module and the `add_to_waitlist` function, but Rust won't let us use them because it doesn't have access to the private sections.
In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
If you want to make an item like a function or struct private, you put it in a module.

Items in a parent module can't use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they're defined.
To continue with our metaphor, think of the privacy rules as being like the back office of a restaurant: what goes on in there is private to restaurant customers, but office managers can see and do everything in the restaurant they operate.

Rust chose to have the module system function this way so that hiding inner implementation details is the default.
That way, you know which parts of the inner code you can change without breaking outer code.
However, Rust does give you the option to expose inner parts of child modules' code to outer ancestor modules by using the `pub` keyword to make an item public.

##
