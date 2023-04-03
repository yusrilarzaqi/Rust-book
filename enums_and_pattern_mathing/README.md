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

##
