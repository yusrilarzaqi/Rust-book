# Common Collections

Rust's standard library includes a number of very useful data structures called _collections_.
Most other data types represent one specific value, but collections can contain multiple values.
Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you'll develop over time.
In this chapter, we'll discuss three collections that are used very often in Rust programs:

- A _vector_ allows you to store a variable number of values next to each other.
- A _string_ is a collection of characters.
  - We've mentioned the `String` type previously, but in this chapter we'll talk about it in depth.
- A _hash map_ allows you to associate a value with a particular key.
  - It's a particular implementation of the more general data structure called a _map_.

To learn about the other kinds of collections provided by the standard library, see [the documentation](https://doc.rust-lang.org/std/collections/index.html).

We'll discuss how to create and update vectors, strings, and hash maps, as well as what makes each special.

## Storing Lists of value with Vectors

The first collection type we'll look at is `Vec<T>`, also known as a _vector_.
Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
Vectors can only store values of the some type.
They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

### Creating a New Vector

To create a new vector, we call the `Vec::new` function, as shown in Listing 8-1.

```rust
    let v: Vec<i32> = Vec::new();
```

Listing 8-1: Creating a new, empty vector

Note that we added a type annotation here.
Because we aren't inserting any values into this vector, Rust doesn't know what kind of elements we intend to store.
This is an important point.
Vectors are implemented using generics; we'll cover how to use generics with your own types in Chapter 10.
For now, know that the `Vec<T>` type provided by the standard library can hold any type.
When we create a vector to hold a specific type, we can specify the type within angle brackets.
In Listing 8-1, we've told Rust that the `Vec<T>` in `v` will hold elements of the `i32` type.

More often, you'll create a `Vec<T>` will initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation.
Rust conveniently provides the `vec!` macro, which will create a new vector that holds the values you give it.
Listing 8-2 creates a new `Vec<i32>` that holds the values `1`, `2`, and `3`.
The integer type is `i32` because that's the default integer type, as we discussed in the ["Data Types"](https://doc.rust-lang.org/book/ch03-02-data-types.html#data-types) section of Chapter 3.

```rust
    let v = vec![1, 2, 3];
```

Listing 8-2: Creating a new vector containing values.

Because we've given initial `i32` values, Rust can infer that the type of `v` is `Vec<i32>`, and the type annotation isn't necessary.
Next, we'll look at how to modify a vector.

### Updating a Vector

To create a vector and then add elements to it, we can use the `push` method, as shown in Listing 8-3.

```rust
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

Listing 8.3: Using the `push` method to add values to a vector.

As with any variable, if we want to be able to change its value, we need to make it mutable using the `mut` keyword, as discussed in Chapter 3.
The numbers we place inside are all of type `i32`, and Rust infers this the data, so we don't need the `Vec<i32>` annotation.

### Reading Elements of Vectors

There are two ways to reference a value stored in a vector: via indexing or sing the `get` method.
In the following examples, we've annotated the types of the values that are returned from these functions for extra clarity.

Listing 8-4 shows both methods of accessing a value in a vector, with indexing syntax and the `get` method.

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(thrid) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
```

Listing 8-4: Using indexing syntax or the `get` method to access an item in a vector.

Note a few details here.
We use the index value of `2` to get the third element because vectors are indexed by number, starting at zero.
Using `&` and `[]` gives us a reference to the element at the index value.
When we use the `get` method with the index passed as an argument, we get an `Option<&T>` that we can use with `match`.

The reason Rust provides these two ways to reference an element is so you can choose how to program behaves when you try to use an index value outside the range of existing elements.
As an example, let's see what happens when we have a vector of five elements and hen we try to access an element at index 100 with each technique, as shown in Listing 8-5.

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
```

Listing 8-5: Attempting to access the element at index 100 in a vector containing five elements.

When we run this code, the first `[]` method will cause the program to panic because it references a nonexistent element.
This method is best used when you want your program to crash if there's an attempt to access an element past the end of the vector.

When the `get` method is passed an index that is outside the vector, is returns `None` without panicking.
You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.
Your code will then the have logic to handle having either `Some(&element)` or `None`, as discussed in Chapter 6.
For example, the index could be coming from a person gets a `None` value, you could tell the user how many items are in the current vector and give them another change to enter a valid value.
That would be more user-friendly than crashing the program due a typo!

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure this reference and any other references to the contents of the vector remain valid.
Recall the rule states you can't have mutable and immutable references in the same scope.
The rule applies in Listing 8-6, where we hold an immutable reference to the first element in a vector and try of add an element to the end.
This program won't work if we also try to refer to that element later in the function:

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
}
```

Listing 8-6: Attempting to add an element to a vector while holding a reference to an item.

Compiling this code will result in this error:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {first}");
  |                                      ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` due to previous error
```

The code in Listing 8-6 might look like it should work: why should a reference to the first element care about changes at the end of the vector?
This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn't enough room to put all the elements next to each other where the vector is currently stored.
In that case, the reference to the first element would be pointing to deallocated memory.
The borrowing rules prevent programs from ending up in that situation.

> Note: For more on the implementation detail of the `Vec<T>` type, see ["The Rustonomicon"](https://doc.rust-lang.org/nomicon/vec/vec.html).

##