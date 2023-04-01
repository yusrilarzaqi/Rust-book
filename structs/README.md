# Using Structs to Structure Related Data

A _struct_, or _structure_, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.
If you're familiar with an object-oriented language, a _struct_ is like a object's data attributes.
In this chapter, we'll compare and contrast tuples with structs to build on what you already know and demonstrate when structs are a bettery way to group data.

We'll demonstrate how to define and instantiate structs.
We'll discuss how to define associated functions, especially the kind of associated functions called _methods_, to specify behavior associated with a struct type.
Struct and enums (discuss in Chapter 6) are the building bloks for creating new types in your program's domain to take full advantage of Rust's compile-time checking.

## Defining and Instantiating Structs

Struct are similar to tuples, discussed in ["The Tuple Type"](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type) section, in that both hold multiple related values.
Like tuples, the pieces of a struct can be different types.
Unlike with tuples, in a struct you'll name each piece of data so it's clear what the values mean.
Adding these names means that structs are more flexible than tuples:you don't have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct.
A stuct's name should describe the significance of the pieces of data being grouped together.
Then, inside curly brackets, we define the names and types of the pieces of data, which we call _fields_.
For example, Listing 5-1 shows a struct that stores information about a user account.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Listing 5-1:`User` struct definition

To use a struct after we've define it, we create an _instance_ of that struct by specifying concrete value for each of the fileds.
We create an instance by stating the name of the struct and then add add curly brackets containing _key_: _value pair_, when the keys are the names of the fields and the values are the data we want to store in those fields.
We don't have to specify the fields in the same order in which we declared them in the struct.
In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.
For example, we can declare a particular user as shown in Listing 5-2.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
    };
}
```

Listing 5-2:Creating an instance of the `User` struct.

To get a specific value from a struct, we dot notation.
For example, to access this user's email address, we use `user1.email`.
If the instance is mutable, we can change a value by using the dot notation and assigning into particular field.
Listing 5-3 shows how to change the value in the `email` field mutable `User` instance.

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

Listing 5-3:Changing the value in the `email` field a `User` instance.

Note that the entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable.
As with any expression, we can construct a new instance of the struct as the expression in the function body to implicitly return that new instance.

Listing 5-4 shows a `build_user` function that returns a `User` instance with the given email and username.
The `active` field gets the value of `true`, and the `sign_in_count` gets a value of `1`.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

Listing 5-4:A `build_user` function that takes an email and username and returns a `User` instance.

It makes sense to name the function parameters with the same name as the struct fields, but having to repeat the `email` and `username` field names and variables is a bit tendious.
If the struct had more fields, repeating each name would get even annoying.
Luckily, there's convenient shorthand!

## Using the Field Init Shorthand

Because the parameter names and the struct field names are exactly the same Listing 5-4, we can use the _field init shorthand_ syntax to rewrite `build_user` so it behave exactly the same but doesn't have the repetition of `username` and `email`, as shown in Listing 5-5.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

Listing 5-5:A `build_user` function that uses field init shorthand because the `username` and `email` parameters have the same name as struct fields.

Here, we're create a new instance of the `User` struct, which has a field named `email`.
We want to set the `email` field's value to the value in the `email` parameter of the `build_user` function.
Because the `email` field and the `email` parameter have the same name, we only need to write `email` rather than `email: email`.

## Creating instances from Other Instances with Struct Update Syntaix

It's often useful to create a new isntance of a struct that includes most of the values from another instance, but changes some.
You can do this using _struct update syntax_.

First, in Listing 5-6 we show how to create a new `USer` instance in `user2` regularly, without the update syntax.
We set a new value for `email` but otherwise use the same values from `user1` that we create in Listing 5-2.

```rust
fn main() {
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Listing 5-6:Creating a new `User` instance using one of the values from `user1`

Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7.
The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the field in the given instance.

```rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

Listing 5-7:Using struct update syntax to set a new `email` value for a `User` instance but to use the rest of the values from `user1`

The code in Listing 5-7 also creates an instance in `user2` that has a different value for `email` but has the same values for the `username`, `active`, and `sign_in_count` fields from `user1`.
The `..user1` must come last to specify that any remaining fields should get their values from the corresponding field in `user1`, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct's definition.

Note that the struct update syntax uses `=` like an assignment; this is because it moves the data, just as we saw in ["Variables and Data interacting with Move"](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move) section.
In this example, we can no longer use `user1` as a whole after creating `user2` because the `String` in the `username` field of `user1` was moved into `user2`.
If we had given `user2` new `String` values for both `email` and `username`, and thus only used the `active` and `sign_in_count` values from `user1`, then `user1` would still be valid after creating `user2`.
Both `active` and `sign_in_count` are types that implement the `Copy` trait, so the behavior we discussed in the ["Stack-Only Data:Copy"](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy) section would apply.

##
