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

##
