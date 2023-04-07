# Enums and Pattern Matching

In this chapter, we'll look at _enumerations_, also reference as _enums_.
Enums allow you to define a type by enumerating its possible _variants_.
First we'll define and use an enum to show how an enum can encode meaning along with data.
Next, we'll explore a particularly useful enum, called `Option`, which expresses that a value can be either something or nothing.
Then we'll look at how pattern matching in the `match` expression makes it easy to run different code for different values of an enum.
Finally, we'll cover how the `if let` construct is another convenient and concise idiom available to handle enums in your code.

## Defining an Enum

Where structs give you a way of grouping together related fields and data, like `Rectangle` with its `width` and `height`, enums give you a way of saying a value is one of a possible set of values.
For example, we may want to say that `Rectangle` is one of a set of possible shapes that also include `Circle` and `Triangle`.
To do this, Rust allows us to encode these possibilities as an enum.

Let's look at a situation we might want to express in ode and see why enums are useful and more appropriate than strucct in this case.
Say we need to work with IP addresses.
Currently, two major standards are used for IP addresses: version four and version six.
Because these are the only possibilities for an IP address that our program will come across, we can _enumerate_ all possible variant, which is where enumeration gets its name.

Any IP address can be either a version four of a version six address, but not both at the same time.
That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants.
Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

We can express this concept in code by defining an `IpAddrKind` enumeration and listing the possible kinds an IP address can be, `V4` and `V6`.
These are the variants of the enum:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

`IpAddrKind` is now a custom data type that we an use elsewhere in our code.

## Enum Values

We can create instances of each of the two variants of `IpAddrKind` like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.
This is useful because now both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type: `IpAddrKind`.
We can then, for instance, define a function that takes any `IpAddrKind`:

```rust
fn route(ip_kind: IpAddrKind) {}
```

And we can call this function with either variant:

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Using enums has even more advantages.
Thinking more about our IP address type, at the moment we don't have a way to store the actual IP address _data_; we only know what _kind_ it is.
Given that you just learned about structs in Chapter 5, you might be tempten to tackle this problem with struct as shown in Listing 6-1.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
}
```

Listing 6-1: Storing the data and `IpAddrKind` variant of an IP address using a `struct`.

Here, we've defined a struct `IpAddr` that has two fields: a `kind` field that is of type `IpAddrKind` (the enum we defined previously) and an `address` field of type `String`.
We have two instances of this struct.
The first is `home`, and has the value `IpAddrKind::V4` as its `kind` with associated address data of `127.0.0.1`.
The second instance is `loopback`.
It has the other variant of `IpAddrKind` as its `kind` value, `V6`, and has address `::1` associated with it.
We've used a struct to bundle the `kind` and `address` values together, so now the variant is associated with the value.

However, representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant.
This is new definition of the `IpAddr` enum says that both `V4` and `V6` variant will be associated `String` values:

```rust
enum IpAddr{
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
```

We attach data to each variant of the enum directly, so there is no need for an extra struct.
Here, it's also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
That is, `IpAddr::V4()` is a function call that takes a `String` argument and returns an instance of the `IpAddr` type.
We automatically get this constructor function defined as a result of defining the enum.

There's another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
Version four Ip addresses will always have four numeric components that will have value between 0 and 255.
If we wanted to store `V4` addresses as four `u8` values but still express `V6` address as one `String` value, we wouldn't be able to with a struct.
Enums handle this case with ease:

```rust
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

We've shown several different ways to define data structures to store version four and version six IP addresses.
However, as it turns out, wanting to store IP addresses and encode which kind they are is so common that [the standard library has a definition we can use!](https://doc.rust-lang.org/std/net/enum.IpAddr.html)
Let's look at how the standard library defines `IpAddr`: it has the exact enum and variants that we've defined and used, but it embeds the address data inside the variants in the form of two different structs, which are defined differently for each variant:

```rust
struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {}
```

This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
You can even include another enum!
Also, standard library types are often not much more complicated than what you might come up with.

Note that even though the standard library contains a definition for `IpAddr`, we can still create and use our own definition without conflict because we haven't brought the standard library's definition into our scope.
We'll talk more about bringing types into scope in Chapter 7.

Let's look at another example of an enum in Listing 6-2: this one has a wide variety of types embedded in its variants.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Listing 6-2: A `Message` enum whose variants each store different amounts and types of values.

This enum has four variants with different types:

- `Quit` has no data assoociated with it at all.
- `Move` has named fields, like a struct does.
- `Write` includes a single `String`.
- `ChangeColor` includes three `i32` values.

Defining an enum with variants such as the ones in Listing 6-2 is similar to defining different kinds of struct definitions, except the enum doesn't use the `struct` keyword and all the variants are grouped together under the `Message` type.
The following structs could hold the same data that the preceding enum variants hold:

```rust
struct QuitMessage;  // unit
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColor(i32, i32, i32); // tuple struct
```

But if we used the different structs, each of which its own type, we couldn't as easily define a function to take any of these kinds of messages as we could with the `Message` enum defined in Listing 6-2, which is a single type.

There is one more similarity between enums and structs: just as we're able to define methods on structs using `impl`, we're also able to define methods on enums.
Here's a method named `call` that we could define on our `Message` enum:

```rust
impl Message {
    fn call(&self) {
        // method method would be defined here
    }
}


fn main() {
    let m = Message::Write(String::from("Hello"));

    m.call();
}
```

The body of the method would use `self` to get the value that we called the method on.
In this example, we've created a variable `m` that has the value `Message::Write(String::from("hello"))`, and that is what `self` will be in the body of the `call` method when `m.call()` runs.

Let's look another enum in the standard library that is very common and useful: `Option`.

## Matches Are Exhaustive

There's one other aspect of `match` we need to discuss: the arms' patterns must cover all possiblilities.
Consider this version of our `plus_one` function, which has a bug and won't compile

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
```

We didn't handle the `None` case, so this code will cause a bug.
Luckily, it's a bug Rust knows how to catch.
If we try to compile this code, we'll get this error:

```
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
 --> src/main.rs:3:15
  |
3 |         match x {
  |               ^ pattern `None` not covered
  |
note: `Option<i32>` defined here
  = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
4 ~             Some(i) => Some(i + 1),
5 ~             None => todo!(),
  |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` due to previous error
```

Rust know that we didn't cover every possible case, and even knows which we forget!
Matches in Rust _exhaustive_: we must exhaust every last possbile in order for the code to be valid.
Especially in the case of `Option<T>`, when Rust prevents us from forgetting to explicitly handle the `None` case, it protects us from assuming that we have a value when we might have null, thus making the billion-dolar discussed earlier impossible.


##
