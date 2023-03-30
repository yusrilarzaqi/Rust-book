# Understanding Ownership

Ownership is Rust's most unique feature and has deep implications for the rest of the language.
It enables Rust to make memory safety guarantees without needing a garbage collector, so it's important to understand how ownership works.

## What is Ownership ?

_Ownership_ is a set of rules that govern how a Rust program manages memory.
All programs have to manage the way they use a computer's memory while running.
Some languages have garbage collection that regularly looks for _no-longer-used_ memory as the program runs;
in the other languages, the programmer must explicitly allocate and free the memory.
Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.
If any of the rules are violated, the program won't compile.
None of the features of ownership will slow down your program while it's running.

Because ownership is a new concept for many programmers, it does take some time to get used to.
The good news is that the more experienced you become with Rust and the rules of the ownership system, the easier you'll find it to naturally develop code that is safe and efficient. Keep at it!

When you understand ownership, you'll have a solid foundation for understanding the features that make Rust unique.

### The Stack and the Heap

Many programming languages don't require you to think about the stack and the heap very often.
But in a systems programming language like Rust, whether a value is on stack or the heap affects how the language behaves and why you have to make certain decisions.

Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.
The stack stores values in the order it gets them and removes the values in the opposite order.
This is referred to _as last in_, _first out_.
Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take on off the top.
Adding or removing plates from the middle or bottom wouldn't works as well!
Adding data is called _pushing onto the stack_ , and removing data is called _popping off the stack_ .
All data stored on the stack must have a known, fixed size.
Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized: when you put data on the heap, you request a certain amount of space.
The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a _pointer_, which is the address of that location.
This process is called _allocating on the heap_ and is sometimes abbreviated as just _allocating_ (push values onto the stack is not considered allocating).
Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

> Think of being seated at a restourant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there.
> If someone in your group comes late, they can ask where you've been seated to find you.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.
Conteporary processors are faster if they jump around less in memory.

> Continuing the analogy, consider a serer at a restourant taking orders from many tables.
> It's most efficient to get all the orders at one table before moving on the next table.
> Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process.

By the same token, a processor can do its job better if it works on data that's close to other data (as it on the stack) rather than farther away (as it can be on the heap).

When your code calls a function, the values passed into the function (including, potentionally, pointers to data on the heap) and the function's local variables get pushed onto the stack.
When the function is over, those get popped off the stack.

Keeping track of whats parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of space are all problem that the stack and heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

## Ownership Rules

- Each value in Rust has an _owner_.
- There can only be one at a time.
- When the owner goes out of scope, the value will be dropped.

## Variable Scope

Now that we're past basic Rust syntax, we won't include all the `fn main() {}` code in examples, so if you're following along, make sure to put the following examples inside a `main` function manually.
As a result, our example will be a bit more concise, letting us focus on the actual details rather than boilerplate code.

As a first example of ownership, we'll look at the _scope_ of some variables.
A scope is the range within a program for which an item is valid. Take the following variables:

```rust
let s = "hello";
```

The variable `s` refers to a string literal, where the value of string is hardcoded into the text of our program.
The variable is valid from the point at which it's declared until the end of the current _scope_.
Listing 4-1 shows a program with comments annotating where the variable `s` would be valid.

```rust
fn main() {
  { // `s` is not valid here, it's not yet declared
    let s = "hello"; // `s` valid from this point forward
  // do stuff with `s`
  }
  // this scope is now over, and `s` is no longer valid
}
```

In the other words, there are two important points in time here:

- When `s` comes _into_ scope, it is valid.
- It remains valid until it goes _out of scope_.

At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages.
Now we'll build on top of this undersanding by introduction the `String` type.

## The String Type

To illustrate the rules of ownership, we need a data type that is more complex than those we covered in the [**Data Types**](https://doc.rust-lang.org/book/ch03-02-data-types.html#data-types).
The types covered previously are of a known size, can be scored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same valud in a different scope.
but we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the `String` type is a great example.

We'll concentrate on the parts of `String` that relate to ownership.
These aspect also apply to other complex data types, wether they are provided by the standard library or created by you.

We've already seen string literals, where a string value is hardcoded into our program.
String literals are convenient, but they aren't suitable for every situation in which we may want to use text.
One reason in that they're immutable.
Another is that not every string value can be known when we wite our code: for example, what if we wat to take user input and store it?
For these situations, Rust has a second string type, `String`.
This type manages data allocated on the heap and as such is able to store an amount of tetx is unknown to us at compile time.
You can create a `String` from a string literal using `from` function, like so:

```rust
let s = String::from("Hello");
```

The double colon `::` operator allows us to namespace this particular `from` function under the `String` type rather than using some sort of name like `string_from`.
We'll discuss this syntax more in ["Method Syntax"](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax) section of Chapter 5 and when we talk about namespacing with modules in ["Path for Referring to an item in the Module Tree"](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html) in Chapter 7.

This kind string _can_ be mutated:

```rust
let mut s = String::from("hi");
s.push_str(" mom"); // push_str() appends a literal to a string
println!("{}", s); // this will print hi mom
```

So, what's the differece here? Why can `String` be mutated but literals cannot?
The difference is how these two types deal with memory.

## Memory and Allocation

In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
This is why string literals are fast and efficien.
But these properties only come from the string literal's immutablilly.
Unfortunately, we can't put a blob of memory into the binary for each piece of thext whose size is unknown at compile time and whose size might change while running the program.

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we're done with our `String`.

That first part is done by us: when we call `String::from`, its implementation requests the memory it needs.
This is preety much universal in programming languages.

However, the second part is different.
In languages with a _garbage collector (**GC**)_, the GC keeps track of and cleans up memory that isn't being used anymore, and we don't need to think about it.
In most languages without a GC, it's our responsibility to identify when memory is no longer being used and call code to explicity free it, just as we did to request it.
Doing this correctly has historically been a difficult programing problem, if we forget, we'll waste memory.
If we do it too early, we'll have an invalid variable.
If we do it twice, that's a bug too.
We need to pair exacly one `allocate` with exacly one `free`.

Rust takes a different path: the memory is automatically returned onece the variable that owns it goes out of scope.
Here's a version of our scope example from Listing 4-1 using a `Strin` instead of string literal:

```rust
{
    let s = String::from("Hello"); // s is valid from this point forward
    // do stuff with s
}   // this scope is now over, and s is no longer valid
```

There is a natural point at which we can return the memory our `String` needs to the allocator: when `s` goes out of scope.
When a variable goes out of scope, Rust calls a special function for us.
The function is called [`drop`](https://doc.rust-lang.org/book/), and it's where the author of `String` can put the code to return the memory.
Rust calls `drop` automatically at the closing curly bracket.

> Note: in C++, this pattern of deallocating resources at the end of an item's lifetime is sometimes called _Resource Acquisition Is Initialization (RAII)_.
> The `drop` function in Rust will be familliar to you if you've used RAII patterns.

This pattern has a profound impact on the way Rust code is written.
It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the data we've callocated on the heap.
Let's explore some of those situations now.

## Ways Variables and Data Interact: Move

Multiple variables can interact with the same data in different ways in Rust.
Let's look at an example using an integer in Listing 4-2.

```rust
let x = 5;
let y = x;
```

Listing 4-2: Assigning the integer value of variable `x` to `y`.

We can probably guess what this is doing: "bind the value `x` to bably guess what this is doing: "bind the value `x` to bably guess what this is doing: "bind the value `5` to `x`; then make a copy of the valud in `x` and bind it to `y`"
We now have two variables, `x` and `y`, and both equal `5`.
This is indeed what is happening, because integers are simple value with a known, fixed size, and these two `5` values are pushed onto the stack.

Now let's look at the `String` version:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

This looks very similar, so we might assume that the way it works would be the same: that is, the second line would make a copy of the value in `s1` and bind it to `s2`.
But this isn't quite what happens.

Take a look at Figure 4-1 to see what is happening to `String` under the covers.
A `String` is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity.
This group of data is stored on the stack.
On the right is the memory on the heap that holds the contents.

![4-1](./img/trpl04-01.svg)

Figure 4-1: Representation in memory of a `String` holding the value `"hello"` bound to `s1`

The length is how much memory, in bytes, the contents of the `String` is currently using.
The capacity is the total amount of memory, in bytes, that the `String` has received from the allocator.
The differece between length and capacity matters, but not in this context, so for now, it's fine to ignore the capacity.

When we assign `s1` to `s2`, the `String` data is coppied, meaning we copy the pointer, the length, and the capacity that are on the stack.
We do not copy the data on the heap that the pointer refers to.
In other words, the data representation in memory looks like Figure 4-2.

![4-2](./img/trpl04-02.svg)

Figure 4-2:Representation in memory of the variable `s2` that has a copy of the pointer, length, and capacity of `s1`.

The representation does _not_ look like Figure 4-3, which is what memory would look like if Rust instead copied the heap data as well.
If Rust did this, the operation `s2 = s1` could be very expresive in terms of runtime performance if the data on the heap were large.

![4-3](./img/trpl04-03.svg)

Figure 4-3:Another possibility for what `s2 = s1` might do if Rust copied the heap data as well.

Earlier, we said that when a variable goes out of scope, Rust automatically calls the `drop` function and cleans up the heap memory for that variable.
But Figure 4-2 shows both data pointers pointing to the same location.
This is a problem: when `s2` and `s1` go out of scope, they will both try to free the same memory.
This is known as a _double free_ error and is one of the memory safety bugs we mentioned previously.
Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, after the line `let s2 = s1`, Rust consider `s1` as no longer valid.
Therefore, Rust doens't need to free anything when `s1` goes out of scope.
Check out what happends when you try to use `s1` after `s2` is created, it won't work:

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}. wold!", s1);
```

You'll get an error like this because Rust prevents you from using the invalidated reference:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

If you've heard the terms _shallow copy_ and _deep copy_ while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sound like making a shallow copy.
But because Rust also invalidateds the first variable, instead of calling it a shallow copy, it's known as a _move_.
In this example, we would say that `s1` was _moved_ into `s2`.
So what actually happens in shown in Figure 4-4.

![4-4](./img/trpl04-04.svg)

Figure 4-4:Representation in memory after `s1` has been invalidated

That solves our problem~
With `s2` valid, when it goes out of scope, it alone will free the memoy and we're done.

In addition, there's design choise that's implied by this: Rust will never automatically create "deep" copies of you data.
Therefore, any _automatic_ copying can be assumed to be inexpensive in terms of runtime performance.

## Ways Variable and Data Interact: Clone

If we _do_ want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`.
We'll discuss method syntax in Chapter 5, but because methods are a common feature in meny programming languages, you've probably seen them before.

Here's an example of the `clone`

```rust
{
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);
}
```

This works just fine and explicity produces the behavior shown in Figure 4-3, where the heap data _does_ get copied.

When you see a call to `clone`, you know that some arbitrary code is being executed and that code may be expensive.
It's a visual indicator that something different is going on.

## Stack-Only Data: Copy

There's another wrinkle we haven't talked about yet.
This code using integers - part of which was shown in Listing 4-2 - works and it is valid:

```rust
{
  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);
}

```

But this code seems to contradict what we just learned:we don't have a call to `clone`, but `x` is still valid and wasn't moved into `y`.

This reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copied of the actial values are quick to make.
That means there's no reason we would want to prevent `x` from being valid after create the variable `y`.
In other words, there's no differece between deep and shallow copying here, so calling `clone` wouldn't do anything different from the usual shallow copying and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on types that are sotred on the stack, as integers are (we'll talk more about traits in [Chapter 10]()), if a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Rust won't let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait.
If the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we'll get a compile-time error.
To learn about how to add the `Copy` annotation to your type to implements the trait, see ["Drivable Trait"](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) in Appendix C.

So, what types implement the `Copy` trait?
You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement `Copy`, and nothing that requires allocation or is some form of resource can implement `Copy`.
Here are some of the types that implement `Copy`:

- All the integer types, suck as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating-point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`.
  - For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

## Ownership and function

The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
Passing a variable to a function will move or copy, just as assignment does.
Listing 4-3 has an example with some annotations showing where variable go into and out of scope.

Filename: `src/main.rs`

```rust
fn main() {
    let s = String::from("Hello"); // s comes into scope

    take_ownership(s);// s's value moves into the function
                       // and so is no longer valid here.

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function
                   // but i32 is Copy
                   // use x afterward;
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_func);
} // Here, some_string goes out of scope and `drop` is called.
  // The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
   println!("{}", some_integer) ;
} // Here, some_integer goes out of scope. Nothing special happens.
```

Listing 4-3:Function with ownership and scope annotated

If we tried to use `s` after the call to `take_ownership`, Rust would throw a compile-time error.
These static check protect us from mistakes.
Try adding code to `main` that uses `s` and `x` to see where you can use them and where the ownership rules prevent you from doing so.

## Return Values and Scope.

Returning values can also transfer ownersip. Listing 4-4 shows an example of a function that returns some value, with similar annotations as those in Listing 4-3.

Filename: `src/main.rs`

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

Listing 4-4:Transferring ownership of return values.

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every function is a bit tendious.
What if we wat to let a function use a value but not take ownership?
It's quite annoying that anything we pass in also need to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might to return as well.

Rust does let us return multiple values using a tuple, as shown in Listing 4-5.

Filename `src/main.rs`

```rust
fn main() {
    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len)
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

Listing 4-5:Returning ownership of parameters

But this is too much ceremony and a lot of work for a concept that should be common.
Luckily for us, Rust has a feature for using a value without transferring ownership, called _references_.

## References and Borrowing

The issue with the tuple code in Listing 4-5 is that we have to return the `String` to the calling function so we can still use the `String` after the call to `calculate_length`, because the `String` was moved into `calculate_length`.
Instead, we can provide a reference to the `String` value.
A _reference_ is like a pointer in that it's an address we can follow to access to the data stored at that address; that adata is owned by some other variable.
Unlike a pointer, a reference is guaranteed to point a valid value of a particular type for the life of that reference.

Here is how you would define and use a `calculate_length` function that has a reference to an objet a parameter instead of taking ownership of the value:

```rust
fn main() {
   let s1 = String::from("hello") ;

   let len = calculate_length(&s1);

   println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize  {
    return s.len()
}
```

First, notice that all the tuple code in the variable declaration and the function return value is gone.
Second, note that we pass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than `String`.
These ampersands represent _references_, and they allow you to refer to some value without taking ownership of i.
Figure 4-5 depicts this concept.

![4-5](./img/trpl04-05.svg)

Figure 4-5:A diagram of `&String` `s` pointing `String` `s1`.

> Note: The opposite of referencing by using `&` is _dereferencing_, which is accomplished with the deference operator `*`.
> We'll see some uses of the deference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.

Let's take a closer look at the function call here.

```rust
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

The `&s1` syntax lets us create a reference that _refers_ to the value `s1` but does not own it.
Because it does not own it, the value it points to will not be dropped when the reference stops being used.

Likewise, the signature of the function uses `&` to indicate that the type of the parameter `s` is a reference.
Let's add some explanatory annotation:

```rust
fn calculate_length(s &String) -> usize {
    s.len()
}
```

The scope in which the variable `s` is valid is the same as any function parameter's scope, but the value pointed to by the reference is not dropped when `s` stops being used, because `s` doesn't have ownership.
When functions have references as parameters instead of the actual values, we won't need to return the values in order to give back ownership, because we never had ownership.

We call the action of creating a reference _borrowing_.
As in real life, if a person owns something, you can borrow it from them.
When you're done, you have to give it back.
You don't own it.

So, what happens if we try to modify something we're borrowing?
Try the code in Listing 4-6.
Spoiler alert;it doesn't work!

```rust
fn main() {
   let mut s1 = String::from("hello") ;

   change(&s1);
}

fn change(some_string: &String)  {
    some_string.push_str(", world");
}
```

Listing 4-6:Attempting to modify a borrowed value

Here's the error:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error
```

Just as variables are immutable by default, so are references.
We're not allowed to modify something we have a reference to.

## Mutable References

We can fix the code from Listing 4-6 to allow us to modify a borrowed value with just a few small tweaks that use, instead, a _mutable reference_:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First we change `s` to be `mut`.
Then we create a mutable reference with `&mut` `s` where we call the `change` function, and update the function signature to accept a mutable reference with `some_string: &mut String`.
This makes it very clear that the `change` function will mutate the value it borrows.

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other reference to that value.
This code that attempts to create two mutable references to `s` will fail:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

Here's the error:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` due to previous error
```

This error says that this code is invalid because we cannot borrow `s` as mutable more than once at a time.
The first mutable borrow is in `r1` and must last until it's used in the `println!`, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in `r2` that borrows the same data as `r1`.

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.
It's something that new Rustaceans struggle with because most languages let you mutate whenever you'd like.
The benefit of having restriction is that Rust can prevent data races at compile time.
A _data race_ is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At lease one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you're trying to track them down at runtime;
Rust prevents this problem by refusing to compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not _simultaneous_ once:

```rust
let mut s: String = String::from("hello");

{
    let r1: &mut String = &mut s;
}   // r1 goes out of scope here, so we can make a new reference with no problems.

let r2: &mut String = &mut s;
```

Rust enforces a similar rule for combining mutable and immutable references.
This code results in an error:

```rust
fn main() {
    let mut s = String::from("Hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, {}", r1, r2, r3);
}
```

Here's the error:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

Whew! we _also_ cannot have a mutable reference while we have an immutable one to the same value.

Users of an immutable reference don't expect the value to suddenly change out from under them!
However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else's reading of the data.

Not that a reference's scope starts from where it is introduced and continues through the last time that reference is used.
For instance, this code will compile because the last usage of the immutable references, the `println!`, occurs before the mutable reference is introduced:

```rust
fn main() {
    let mut s = String::from("Hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{}, {}", r1, r2);
    // variable r1 and r2

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last used, which is before the mutable reference `r3` is created.
These scopes don't overlap, so this code is allowed:the compiler can tell that the reference is no longer being used at a point before the end of the scope.

Even though borrowing errors may be frustrating at times, remember that it's the Rust compiler pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly where the problem is.
Then you don't have to track down why your data isn't what you thought it was.

## Dangling References

In languages with pointers, it's easy to erroneously create a _dangling pointer_ - a pointer that references a location in memory that may have been given to someone else - by freeing some memory while preserving a pointer to that memory.
In Rust, by contrast, the compiler guarantees that references will never be dangling references:if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

Let's try to create a dangling reference to see how Rust prevents them with a compile-time error:

```rust
fn main() {
    let reference_to_noting = dangel();
}

fn dangel() -> &String {
    let s = String::from("Hello");

    &s
}
```

Here's the error:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                 +++++++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```

This error message refers to a feature we haven't covered yet: **lifetimes**.
We'll discuss lifetime in detail in Chapter 10.
But, if you disregard the parts about lifetimes, the message does contain the key to why this code is a problem:

```
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

Let's take a closer look at exactly what's happening at each stage of our `dagle` code:

```rust
fn dangel() -> &String {
    // dangel returns a reference to a String
    let s = String::from("Hello"); // s is a new String

    &s // we return reference to the String, s
} // Here, s soes out of scope, and dropped, Its memory goes away.
  // Danger !
```

Because `s` is created inside `dangel`, when the code of `dangel` is finished, `s` will be deallocated.
But we tried to return a reference to it.
That means this reference would be pointing to an invalid `String`.
That's no good! Rust won't let us do this.

The solution here is to return the `String` directly:

```rust
fn dangel() -> String {
    let s = String::from("Hello"); // s is a new String

    s
}
```

This works without any problems.
Ownership is moved out, and nothing is deallocated.

## The Rules of References

Let's recap what we've discussed about references:

- At any given time, you can have _either_ one mutable reference _or_ any number of immutable references.
- References must always be valid.

Next, we'll look at a different kind of reference: slices.

## The Slice Type

_Slices_ let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.

Here's a small programming problem:write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
If the function doesn't find a space in the string, the whole string must be one word, so the entire string should be returned.

Let's work through how we'd write the signature of this function without using slices, to understand the problem that slices will solve:

```rust
fn fist_word(s: &String) -> ?
```

The `fist_word` function has a `&String` as a parameter.
We don't want ownership, so this is find.
But what should we return?
We don't really have a way to talk about _part_ of string.
However, we could return the index of the wold, indicated by a space.
List's try that, as shown in LIsting 4-7.

```rust
fn fist_workds(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

Listing 4-7:The `first_word` function that returns a byte index value into the `String` parameter.

Because we need to go through the `String` element by element by element and check whether a value is a space, we'll convert our `String` to an array of bytes using the `as_bytes` method.

```rust
let bytes = s.as_bytes();
```

Next, we create an iterator over the array of bytes using the `iter` method:

```rust
for (i, &item) in bytes.iter().enumerate() {}
```

We'll discuss iterators in more detail in [Chapter 13](https://doc.rust-lang.org/book/ch13-02-iterators.html).
For now, know that `iter` is a method that returns each element in a collection and that `enumerate` wraps the result of `iter` and returns each element as part of a tuple instead.
The first element of the tuple returned from `enumerate` is the index, and the second element is a reference to the element.
This is a bit more convenient than calculating the index ourselves.

Because the `enumerate` method returns a tuple, we can use patterns to desctructure that tuple.
We'll be discussing pattern [Chapter 6]().
In the `for` loop, we specify a pattern that has `i` for the index in the tuple and `&item` for the single byte in the tuple.
Because we get a reference to the element for `.iter().enumerate()`, we use `&` in the pattern.

Inside the `for` loop, we search for the byte that represents the space by using the byte literal syntax.
If we find a space, we return the position.
Otherwise, we return the length of the string by using `s.len()`.

```rust
    if item == b' ' {
      return i;
    }
  }

  s.len()
```

We now have a way to find out the index of the end of the first word in the string, but there's a problem.
We're returning a `usize` on its own, but it's only a meaningful number in the context of the `&String`.
In other words, because it's a seprate value from the `String`, there's no guarantee that is will still be valid in future.
Considier the program in Listing 4-8 that uses the `fist_word` function from Listing 4-7.

```rust
fn main() {
  let mut s = String::from("hello");

  let word = fist_workds(&s); // word will get the value 5

  s.clear(); // this empies the String, making it equal to ""

  // word still has the value 5 here, but there's no more string that
  // we could meaningfully use the value 5 with.
  // word is now totally invalid!
}
```

Listing 4-8:Sorting the resulting from calling the `first_word` function and then changing the `String` contents

This program conpiles without any errors and would also do so if we used `word` after calling `s.clear()`.
Because `word` isn't connected to the state of `s` at all, `word` still contains the value `s`.
We could use that value `5` with the variable `s` to try to extract the first word out, but this would be a bug because the contents of `s` have changed since we saved `5` in `word`.

Having to worry about the index `word` getting out of sync with the data in `s` is tedious and error prone!
Managing rhese indices is even more brittle if we write a `second_word` function.
Its signature would have to looke like this:

```rust
fn second_word() -> (usize, usize) {}
```

Now we're tracking a starting _and_ an ending index, and we have even more values that were calculate from data in a particular state but aren't tied to that state at all.
We have three unrelated variables floating around that need to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

## String Slices

A _string slice_ is a reference to part of a `String`, and it looks like this:

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

Rather than a reference to the entire `String`, `hello` is a reference to a portion of the `String`, specified in the extra `[0..5]` bit.
We create slices using a range within brackets by specifying `[starting_index..ending_index]`, where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in slice.
Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to `ending_index` minus `starting_index`.
So, in the case of `let world = &s[6..11];`, `world` would be a slice that contains a pointer to the byte at index 6 of `s` with a length value of `5`;

Figure 4-6 shows this diagram.

![4-6](./img/trpl04-06.svg)

Figure 4-6:String slice referring to part of a `String`

With Rust's `..` range syntax, if you want to start at index 0, you can drop the value before the two periods.
In other words, these are equal:

```rust
let s = String::from("hello world");

let slice = &s[0..2];
println!("{}", slice); // he

let slice = &s[..2];
println!("{}", slice); // he
```

By the same token, if your slice includes the last byte of the `String`, you can drop the trailing number.
That means these are equal:

```rust
let s = String::from("hello world");

let len = s.len();
let slice = &s[3..len];
println!("{}", slice); // lo world

let slice = &s[3..];
println!("{}", slice); // lo world
```

You can also drop both values to take a slice of the entire string.
So these are equal:

```rust
let s = String::from("hello world");

let len = s.len();
let slice = &s[0..len]; // hello world
println!("{}", slice);

let slice = &s[..]; // hello world
println!("{}", slice);
```

> Note: String slice range indices must occur at valid UTF-8 character boundaries.
> If you attempt to create a string slice in the middel of a multibyte character, your program will exit with an error.
> For the purposes of introduction string slices, we are assuming ASCII only in this section; a more through discussion of UTF-8 handling is in the ["Storing UTF-8 Encoded Text widh Strings"](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) section of Chapter 8.

With all this information in mind, let's rewrite `first_word` to return a sclice.
The type the signifies "string slice" is written as `&str` :

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

We get the index for the end of the word the same way we did in Listing 4-7, by looking for the first occurrence of a space.
When we find a space, we return a string slice using the start of the string and the index of the space as the starting and ending indices.

Now when we call `first_word`, we get back single value that is tied to the underlying data.
The value is made up of reference to the starting point of the slice and the number of elements in the slice.

Returning a slice would also work for a `second_word` function:

```rust
fn second_word(s: &String) -> &str {}
```

We now have a straightforward API that's much harder to mess up because the compiler will ensure the references into the `String` remain valid.
Remember the bug in the program in Listing 4-8, when we got the index to the end of the first word but then cleared the string so our index was invalid ?
That code was logically incorrent but didn't show any immediate errors.
The problems would show up later if we kept trying to use the first would index with an emptied string.
Slices make this bug impossible and let us know we have a problem with our code sooner.
Using the slice version of `first_word` will throw a compile-timer error:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error !

    println!("the first word is: {}", word);
}
```

Here's the compiler error:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference.
Because `clear` needs to truncate the `String`, it needs to get a mutable reference.
The `println!` after the call to `clear` uses the reference in `word`, so the immutable reference must still be active at that point.
Rust disallows the mutable reference in `clear` and the immutable reference in `word` from existing at the same time, and compilations fails.
Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!

## String Literals as Slice

Recall that we talked about string literals being stored insde the binary.
Now that we know about slices, we can properly understand string literals :

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`:it's like slice pointing to that specific point of the binary.
This also why string literals are immutable;`&str` in an immutable reference.

## String Slices as Parameters

Knowing that you can take slices of literals and `String` values leads us to one more improvement on `first_word`, and that's is signature:

```rust
fn first_word(s: &String) -> &str {}
```

A more experienced Rustaceans would write the signature shown in Listing 4-9 instead because it alows us to use the same function on both `&String` values and `&str` values.

```rust
fn first_word(s: &str) -> &str {}
```

Listing 4-9:Improving the `first_word` function by using a string slice for the type of the `s` parameter.

If we have a string slice, we can pass that directly.
If we have a `String`, we can pass a slice of the `String`.
This flexibility takes advantage of _deref coercions_, a feature we will cover in ["Implicit Deref Coercions with Functions and Methods"](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods) section of Chapter 15..

Defining a function to take a string slice instead of a reference to a `String` makes our API more general and useful without losing any functionality:

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `STring`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("word: {}", word);

    let word = first_word(&my_string[..]);
    println!("word (1) : {}", word);
    // `first_word` also works on references to `String`s, which are quivalent
    // to whole slices of `String's`
    let word = first_word(&my_string);
    println!("word (2) : {}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, wheter partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("word (3) : {}", word);

    let word = first_word(&my_string_literal[..]);
    println!("word (4) : {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word (5) : {}", word);
}
```

## Other Slices

String slices, as you might imagine, are specific to strings.
But There's a more general sclice type too.
Consider this array :

```rust
let a = [1, 2, 3, 4, 5];
```

Just as we might want to refer to part of string, we might want to refer to part of an array.
We'd do so like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
```

This slice has the type `&[i32]`.
It works the same way as tring slice do, by storing a reference to the first element and a length.
You'll use this kind of slice for all sorts of other collections.
We'll discuss these collections in detail when we talk about vectors in Chapter 8.

## Summary

The concept of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don't have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we'll talk about these concept further thoughout the rest of the book.
Let's move on to Chapter 5 and look at grouping pieces of data together in a `struct`.

###
