# RUST 100 exercises

## 1. Welcome

Function declaration example

```rust
fn <function_name>(<var_identifier: var_type, ...>) -> <return_type>{<body>};
fn greeting() -> &'static str{};
```

## 2. Basic Calculator

Types e.g. u32Integers

- Signed/unsigned
- 8/16/32/64/128 bit
- Literal: default i32, suffix type eg 62u64 (u64)

Operators: +, -, *, /, %(mod)

No automatic type coercion even lossless:

```rust
let b: u8 = 100;
let a: u32 = b; 
// Compilation error: mismatched types
```

Variable declaration: `let a = 42;`
Explicit type annotation: `let a: u8 = 42;`

Control Flow: Branching

```rust
if <condition1> {
    // ...
} else if <condition2>{
    // ...
} else {
    // ...
}
```

Condition *MUST* be a bool type
Boolean: true/false
Comparison operators: ==, !=, <, >, <=, >=
if/else expression:

```rust
// if/else returns a value too
let message = 
    if <condition1> {
        "condition1"
    }else{
        "condition2"
    }
```

Panic: Signal an unrecoverable error.
e.g. `100 / 0` -> `thread panicked: attempt to divide by zero`
Can be intentionally triggered with `panic!("This is a panic")`

While Loop
`while <condition> { ... }`

Variables are immutable by default, declare the variable as mutable using the `mut` keyword

For Loop: to execute for each element in an iterator
`for <element> in <iterator> { ... }`
Rust stdlib provides `range` type to iterate over a seq of numbers

```rust
let mut sum = 0;
for i in 1..=5 {
    sum += i;
}
```

Ranges:

- 1..5: 1,2,3,4,
- 1..=5: 1,2,3,4,5
- 1..: 1 to infinity (maximum value of integer type)
- ..5: minimum value of integer type to end at 4
- ..=5: minimum value of integer type to end at 5

Overflow/Underflow
NO automatic promotion when overflow happens
Either *reject the operation (panic)* or *come up with a new result fits into expected type*

Can choose the approach to use when overflow occurs by setting `overflow-checks` in profile:

- `overflow-checks=true` Panic at runtime when an integer operation overflows
- `overflow-checks=false` Wrap around when an integer operation overflows

Default: true for dev profile, false for release profile

Profile: a set of configuration options used to customize the compilation
Cargo provides 4 built-in profiles: dev, release, test and bench

- dev: `cargo build`, `cargo run`, `cargo test`, aimed at local development, sacrifices performance for faster compilation times & better debugging
- release: optimized for runtime performance but longer compilation times. Explicitly request via --release flag
- test: default profile used by `cargo test`, inherits settings from dev profile
- bench: default profile used by `cargo bench`, inherits from release profile

Case-by-case wrapping/panicking
*wrapping_* & *saturating_* methods

```rust
let i:u8 = 255u8.wrapping_add(1u8);
assert_eq!(i, 0); // i=0, wrapped
let u:u8 = 255u8.saturating_add(1u8);
assert_eq!(u, 255); // i=255(u8::MAX), saturated
```

Explicit type conversion:
`as` casting (infallible) to convert between integer types
Recommended only convert from smaller type to larger type, or a truncation will occur

```rust
let i = 255u8 as u16; // i=255
let u = 257u16 as u8; // u=1
```

## 3. Ticket v1

Modelling a JIRA-like ticket, with concepts of `struct`, ownership, memory management, modules & visibility, and strings

### Structs

Assume need to keep track of the title, description and status of each ticket
Start by using a `String` to represent each of them
`String` - a type defined in stdlib to represent *UTF-8* encoded text
Combine three pieces of information into a single entity using a `struct`

```rust
// Define a struct's fields
struct Configuration {
    version: u32,
    active: bool
}
// Define methods
impl Configuration{
    // If the first param is self, it's an instance method
    // Can be called using method call syntax or function call method
    // <instance>.<method>(<params>);
    // <StructName>::<method>(<instance>, <params>);
    fn is_version(self, ver: u32) -> bool {
        self.version == ver
    }

    // If the param doesn't have a self param, it's a static method
    // Can be called only using function call method:
    // <StructName>::<method>(<params>);
    fn new_instance() -> Configuration {
        // Example of instantiation
        Configuration {version: 0, active: false}
    }
}
// Calling a static method
let c = Configuration::new_instance();
// Equivalent methods to call an instance method
print!("{}", c.is_version(0));
print!("{}", Configuration::is_version(c, 0));
```

### Modules

To get proper encapsulation, visibility and modules are needed
**module** is a way to group related code under a common namespace.
Modules can be nested, forming a tree structure:
Root is the crate itself, containing all the other modules

Inline modules: module declaration & contents are next to each other.

External modules: split the code into multiple files:

1. Declare the submodule using the `mod` keyword in the parent module.
2. `cargo` is then in charge of finding the file with the module.

- Expected file for modules declared in the root of the crate:
  - src/<module_name>.rs
  - src/<module_name>/mod.rs
- For submodule:
  - src/<parent_module>/<module_name>.rs
  - src/<parent_module>/<module_name>/mod.rs

Item paths and `use` statementsIn the same module, can access items directly with their nameTo access an entity from a different module, use a `path` to the entity:

- starting from the root of the current crate,
  `crate::module_name::MyStruct`
- starting from the parent module,
  `super::my_function`
- starting from the current module,
  `sub_module_name::MyStruct`

`crate` and `super` are Rust keywords
use `use` statement to import an entity
or use a star import to import all the items from a module
`use crate::module_1::module_2::*;`
**(generally discouraged because can pollute the current namespace)**

Can use `cargo-modules` to visualize the module tree structure

### Visibility

By default, everything in Rust is **private**A private entity can only be accessed **within the same module or its submodule****Visibility modifiers**

- pub: makes the entity public
- pub(crate): makes the entity public within the same crate
- pub(super): makes the entity public within the parent module
- pub(in path::to::module): makes the entity public within the specific module

```rust
// Confifguration is public, version can be accessed from the same crate, bool is private
pub struct Configuration {
    pub(crate) version: u32,
    active: bool
}
```

If at least one field of a struct is private, the struct cannot be instantiated directly, need to provide public constructors
Rust does not have a built-in accessor (getter/setter) generator

### Ownership

Ownership is designed to ensure:

- Data is never mutated when it's being read
- Data is never read while it's being mutated
- Data is never accessed after it has been destroyed

Constraints are enforced by the borrow checker (compiler). Ensures memory safety without affecting performance.

Each value has an owner.
Ownership can be transferred.

```rust
// a is the owner of the String
let a = "hello, world".to_string();
let b = a; // ownership transferred to b
fn foo(x: String){ ... };
foo(b); // move semantics, foo takes ownership of the String
```

To build useful accessor methods we need to start with reference (or all the getters/setters can only run once).

**Borrowing**: to read the value of a variable without taking ownership of it.Done via **borrowing (&)**, tagged with privileges:

- &: immutable references, allow to read the value
- &mut: mutable references, allow to read and mutate the value

To ensure the goals (no read when mutate, no mutate when read), restrictions apply:

- Can't have a mutable reference and an immutable reference at the same time
- Can't have more than one mutable reference at the same time

Mutable reference is like a read-only lock, immutable reference is like a read-write lock.

Borrow a value by adding `&` or `&mut` in front of a variable. Type of the reference to type \<T>: &T

```rust
impl Ticket {
    // Example getter to access the fields of a `Ticket` instance without consuming it
    pub fn title(&self) -> &String {
        &self.title
    }

    // Setter #1 with self
    pub fn changeTitle(self, title: String) -> Self {
        self.title = title;
        self
    }
    // To use:
    // ticket = ticket.changeTitle("New Title");
    // Can be chained
    // ticket = ticket.changeTitle("New Title").changeTitle("Latest Title");
  
    // Setter #2 with mutable referenced self
    pub fn changeTitle(&mut self) {
        self.title = changeTitle;
    }
    // To use:
    // ticket.changeTitle("New Title");
    // Cannot be chained
}
```

### Stack

Last-In-First-Out/FIFO
Used when the data size is known at compile time, e.g. a function is called to push function's arguments, store local variables, etc.
If there are too many nested function calls, stack memories may be exhausted, causing stack overflow

`std::mem::size_of::<data_type>()` verifies how many bytes would the data type take on the stack

### Heap

Used when the data size is not known at compile time, e.g. dynamically-sized collections, strings, etc.
Heap: big chunk of memory. When need to store data, use the `allocator` to reserve a subset of the heap (`heap allocation`), returning a `pointer` to the start of the block.
Need to call the allocator to `free` the memory if no longer need.
More flexible than stack, but allocation is slower.

`String`'s memory layout:
The text itself is stored on the heap;
The pointer to the heap, the length of the String & the capacity of the string (how much bytes reserved) are stored on the stack.\

How much space we need to store pointer, length & capacity? Address are represented with an integer, size = `usize` (u32 on 32-bit machines, u64 on 64-bit machines). Pointers, capacity, length are all represented as `usize`

Most references are represented as a pointer to a memory location (with size `usize`), e.g. &String points to the memory location of the String's metadata (String's pointer in Stack).

### Destructors

**Scope**: the region of code where a variable is valid/alive.Starts with the variable declaration, ends when one of the following happens:

- The block {} where the variable declared ends
- Ownership of the variable is transferred to someone else

**Destructor**: Invoked when the owner of a value goes out of scope, used to clean up the resources used by the value (in particular memory allocated to it). Can also manually invoke by passing the variable to `std::mem::drop`, aka "value has been dropped"
When trasferring the ownership of a value to a function, the responsibility of cleaning it up is also transferred.
Destructor is only to be called ONCE. It is NOT guaranteed, e.g. explicitly choose to leak memory.

Dropping references:

```rust
let x = 123;
let y = &x;
drop(y); // Does NOTHING while dropping a reference
```

## 4. Traits

### Trait

Key traits defined in stdlib:

- Operator traits (`Add`, `Sub`, `PartialEq`, etc.)
- `From` and `Into` for infallible conversions
- `Clone` and `Copy` for copying values
- `Deref` and deref coercion
- `Sized` to mark types with a known size
- `Drop` for custom cleanup logic

Example for an operator trait `PartialEq`:

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String
}
let a = Ticket{title: "123", description: "456", status: "Done"};
let b = Ticket{title: "123", description: "456", status: "Done"};
let c = a == b;
// operation `==` cannot be applied to type `Ticket`
// note: an implementation of `PartialEq` might be missing for `Ticket`
```

Traits are Rust's way of defining **interfaces**
A trait defines a set of methods that a type must implement to satisfy the trait's contract.

Syntax for a trait definition:

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

Syntax for trait implementation:

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // Method body
    }
}
```

To invoke a trait method, use `.` operator as with regular methods. The type must implement the trait and the trait must be in scope using `use`. Rust will automatically include the trait defined in stdlib's **prelude** (as if `use std::prelude::*` is added to every Rust module).

### Orphan Rule

When a type is defined in another crate, can't directly define new methods for it. Example:

```rust
impl u32 {
    fn some_method(){
        // Method body...
    }
}
// error: cannot define inherent `impl` for primitive types, consider using an extension trait instead
```

Extension trait: a trait whose primary purpose is to attach new methods to foreign types (e.g. `u32`).

Orahan Rule: The trait is defined in the current state, or the implementor type is defined in the current crate, or both. Example of violation:

```rust
impl PartialEq for u32 {
    fn eq(&self, _other: &Self) -> bool {
        // Method body...
    }
}
// Implementing a foreign trait (`PartialEq` from `std`) on a foreign type (`u32` from `std`), violating the Orphan Rule
```

### Operator overloading

When write `x==y`, compiler will look for implementation of the `PartialEq` trait for the types of `x` and `y` and replace with `x.eq(y)`.
Corresponding main operators & traits: + (Add), - (Sub), * (Mul), / (Div), % (Rem), == and != (PartialEq), < > <= and >= (PartialOrd).

Default implementation example:

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    // Default implementation for not equal is provided, can skip implementing ne (still can override)
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

### Derive Macros

Implementing `PartialEq` is a bit tedious as we have to manually compare each field of tohe struct.
The implementation is brittle as if the struct definition changes we always have to remember to update the `PartialEq` implementation
To mitigate the risk, use destructuring

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool{
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        if title==other_title ...
    }
}
```

**Macros** are code generators.
**Derive macro** is a particular macro specified as an attribute on top of a struct.
Derive macros will automate the implementation of common traits for custom types, e.g. `PartialEq` trait.

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

### Trait bounds

Generics allow code that works with a type parameter instead of a concrete type:

```rust
fn print_if_even<T>(n:T) where T:IsEven + Debug {
// or: fn print_if_even<T: IsEven + Debug>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

`print_if_even` is a generic function works with any type `T` that implements the `IsEven` trait and the `Debug` trait.

### String slices

We can define a string literal by enclosing the raw text in double quotes:
`let s = "Hello World!";` <- type of `s`: `&str`, a ref to a string slice

`&str` and `String` are different types and not interchangeable.

`String`:
Stack: Metadata (Pointer to heap, length, capacity)
Heap: Text

**String slices**: `&str` is a view into a String, a reference to a sequence of UTF-8 bytes stored elsewhere.

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from s:
let slice: &str = &s[1..];
```

`&str` In memory:
Stack: Pointer ponts to the first byte of the slice on the Heap, length.
Heap: String slice reference doesn't own data on Heap but just points to a `String`'s allocated data.

Use `&str` instead of `&String` whenever need a reference to textual data. `&str` is more flexible and more idiomatic in Rust.
If a method returns `&String`, it's promised that there is heap-allocated UTF-8 text that matches exactly the same.
If a method returns `&str`, just saying somewhere there's a bunch of text data and a subset matches it.

### Deref trait

The `Deref` trait is the mechanism behind the language feature known as **deref coercion**, defined in the stdlib.

```rust
pub trait Deref {
    type Target;

    fn deref(&self) -> &Self::Target;
}
```

`type Target` is an associated type, as a placeholder for a concrete type that must be specified when the trait is implemented.

By implementing `Deref<Target = U>` for a type `T`, we imply that `&T` and `&U` are somewhat interchangeable, in particular:

- References to `T` are implicitly converted into references to `U`. (i.e. `&T` becomes `&U`)
- Can call on `&T` all the methods defined on `U` that take `&self` as input.

`String` implements `Deref` with `Target = str`

```rust
impl Deref for String {
    type Target = str;

    fn deref(&self) -> &str {
        // deref method body...
    }
}
```

`&String` is automatically converted into a `&str` when needed.

### Sized trait

`&str` stores the pointer to the first byte of the string slice and the length of the slice. `str` is a dynamically sized type (DST), whose size is not known at compile time. Whenever we have a reference to a DST, e.g. `&str`, it has to include additional information (e.g. length) about the data it points to, a.k.a. fat pointer.

Rust's stdlib defines a trait called `Sized` (a **marker trait**, empty trait with no methods to implement, serves to mark a type as having certain properties. The mark is then leveraged by the compiler to enable certain behaviors or optimizations.). A type is `Sized` if its size is known at compile time, i.e. not a DST.

`Sized` is also a **auto trait**, the compiler will implement it automatically based on type's definition.

### From trait

```rust
struct Ticket {
    title: String
}
let ticket = Ticket{title: "title".into()};
```

`"title"` is a String slice (`&str`), but a `String` is expected: a conversion is needed.

**From and Into**: two traits for infallible conversions, in the `std::convert` module0000.

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

**Supertrait/Subtrait** `From: Sized` implies that `From` is a subtrait of `Sized`: any type that implements `From` must also implement `Sized`.
**Implicit trait bounds** Every time having a generic type parameter, the compiler implicitly assumes it's `Sized`.

```rust
// Equivalent:
pub struct Foo<T> {
    inner: T,
}
pub struct Foo<T: Sized> {
    inner : T,
}
```

**Negative trait bounds** Can opt out of the implicit `Sized` bound with a negative trait bound:

```rust
pub struct Foo<T: ?Sized> {
    inner: T,
}
```

Implies that "`T` may or may not be `Sized`", allows us to bind `T` to a DST (e.g. `Foo<str>`).
Special case: negative trait bounds are exclusive to `Sized`, can't use them with other traits.

**&str to String**
`String` types implement the `From` trait. Thus, can write:
`let title = String::from("title");`
or:
`let title = "title".into();`
However, `Into<String>` is not implemented for `&str`, but we can still use `into()` on `&str`:
`From` and `Into` are **dual traits**, `into` is implemented for any type that implements `From` using a **blanket implementation**:

```rust
impl <T, U> Into<U> for T where U: From<T> {
    fn into(self) -> U {
        U::from(self)
    }
}
```

`into()` will work as long as the compiler can infer the target type from the context (e.g. function signature/variable declaration type annotation).

### Generics and associated types

```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}
pub trait Deref {
    type Target;
    fn deref(&self) -> &Self::Target;
}
```

For `From`, `T` is a generic parameter
For `Deref`, `Target` is an associated type

`Target`(Associated type): at most one implementation. E.g. `String` can only deref to `str`. Uniquely determined by the trait implementation only once.

`T` (Generic traits): can implement `From` multiple times for a type as long as the input type `T` is different. E.g. `impl From<u32> for Wrapping` and `impl From<u16> for Wrapping` (considered as different traits and no ambiguity).

Use an `associated type` when the type must be uniquely determined for a given trait implementation;
Use a `generic parameter` when want to allow multiple implementations of the trait for the same type with different input types.

### Clone trait

The restrictions on ownership can be somewhat limiting: we might have to call a function that takes ownership of a value, but still need to use that value afterward.

`Clone` is a trait defined in stdlib, takes a reference to `self` and returns a new owned instance of the same type.

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
// Or simply use:
#[derive(Clone)]
```

### Copy trait

```rust
fn consumer(s: u32) { /* ... */}
fn example(){
    let s = 5;
    consumer(s);
let t = s + 1; // s is still available even after consumer() gets the ownership `/
}
```

`Copy` is another marker trait defined in the stdlib
If a type implements `Copy`, there's no need to call `clone()` to create a new instance of the type: implicitly bitwise clone (`memcpy`) is performed for every call.

Requirements to implement `Copy`:

- Must implement `Clone` (`Copy` is a subtrait of `Clone`)
- The type doesn't manage any additional resources (e.g. heap memory, file handles, etc.) beyond the std::mem::size_of bytes occupies in memory
- The type is not a mutable reference (&mut): there should be only one mutable borrow of a value at any given time

Derive `Copy`: `#[Derive(Clone, Copy)]

### Drop trait

`drop` function:

- Reclaims the memory occupied by the type (`std::mem::size_of` bytes)
- Cleans up any additional resources that the value might be managing (e.g. the heap buffer of a `String`)

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

`Drop` trait is a mechanism to define additional cleanup logic beyond what the compiler does automatically.
If type has an explicit `Drop` implementation, the compiler will assume that the type has additional resources attached to it and won't allow `Copy` trait.

### Wrapping up guidelines

- Don't make a function generic if it's always invoked with the same type, will make the code harder to understand/maintain
- Don't create a trait if only have one implementation.
- Implement standard traits (`Debug`, `PartialEq`, etc.) for custom types whenever it makes sense. It will make the types easier to work with, unlocking functionalities provided by the stdlib and ecosystem crates.
- Implement traits from third-party crates if need the functionality they unlock within their ecosystem.
- Do not make the code generic solely to use mock data types.

## Modelling A Ticket pt.2

### enum

An enumeration is a type that can have a fixed set of values called variants, defined using the `enum` keyword:

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum` is like `struct` that defines a new Rust type.

### match

The most common operation on `enum`, to compare a Rust value against a series of patterns.

```rust
imwpl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            Status::InProgress | Status::ToDo => false
        }
    }
}
```

`match` is exhaustive, must handle all enum variants, or there will be a compile-time error ("pattern xxx not covered").
If don't care about one or more variants, can use `_` pattern as a catch-all:

```rust
match status {
    Status::Done => true,
    _ => false
}
```

### enum variants to hold data

Can attach data to each variant in Rust enums.

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

`InProgress` is a struct-like variant. `assigned_to` is variant-specific, not available on all `Status` instances. To access, need to use pattern matching:

```rust
match status:Status {
    Status::InProgress { assigned_to: abc } => {
        println!("Assigned to: {}", abc);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

### Concise branching with `if let`

`if let` construct allows to match on a single variant of an enum, without having to handle all the other variants.

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str
    {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!("Only `In-Progress` tickets can be assigned to someone");
        }
    }
}
```

If the `else` branch is to return early, can also use `let/else` construct:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            // Return early if condition does not meet
            panic!("Only `In-Progress` tickets can be assigned to someone");
        };
        assigned_to
    }
}
```

### Nullability

`Option` is a Rust type that represents **nullable values**, as an enum defined in Rust's stdlib

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option` encodes the data that a value must be present(`Some(T)`) or absent(`None`).
Forces to explicitly handle both cases or there will be a compiler error.
`Some` is a tuple-like variant that holds unnamed fields. Tuple-like variants are often used when there is a single field to store.

To define tuple-like structs: `struct Point(i32, i32);`
To access: `point.0`/`point.1`...

Tuples: another primitive Rust type, group together a fixed number of values with (potentially different) types.`let i: (i32, &str) = (3, "hello");`

### Fallibility

We use `panic!` when the checks fail in `Ticket::new` before, which is not ideal as it doesn't give the caller a chance to handle the error.
Use the `Result` type defined in the stdlib instead:

```rust
enum Result<T,E> {
    Ok(T),
    Err(E),
}
```

`Ok(T)` represents a successful operation, `Err(E)` represents a failed operation.
Rust does not have exceptions but uses `Result`, which forces programmers to encode fallibility in the function's signature.
`Result` makes fallibility explicit.

### Unwrap

`Result` forces the caller to handle errors at the call site.
The caller can panic if the operation failed:

```rust
let number = parse_int("42").unwrap(); // Panic if returns an `Err`
let number = parse_int("42").expect("Failed to parse the integer"); // Custom panic message
```

Or to destructure the `Result` to deal with the error case explicitly:

```rust
match parse_int("42") {
    Ok(n) => println!("Number parsed: {}", n),
    Err(err) => eprintln!("Error: {}", err),
}

### Error enums

When want to allow the caller to behave differently based on the specific error that occured, can use an enum to represent the different error cases to encode the different error cases in the type system:

```rust
enum U32ParseError {
    NotANumber,
    TooLarge,
    Negative,
}
fn Foo() -> Result<u32, U32ParseError> { ... }

match Foo {
    Ok<n> => n,
    Err(U32ParseError::NotANumber) => {
        panic!("Not a number!!");
    },
    Err(U32ParseError::TooLarge) => u32::MAX
    Err(U32ParseError::Negative) => 0,
}
```

### Error trait

Previously we have to destructure the Error enum variant to extract the error message and pass it to the arm logic, this is an example of error reporting: transforming an error type into a representation that can be shown to a user/service operator/developer. Rust provides the `std::error::Error` trait for the general error reporting strategy.

There are no constraints on the type of the `Err` variant in a `Result`, but it's a good practice to use a type that implements the `Error` trait `pub trait Error: Debug + Display { ... }`, which is the cornerstone of Rust error handling.
`Debug` and `Display` both have the only function `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;`, but `Debug` is for developers but `Display` is for end-users. `Debug` can be implemented with #[derive(Debug)] while `Display` must be manually implemented.

### Libraries and packages

It's not very convenient to implement the `Error` trait as we have to implement `Display` as well.
Can remove some of the boilerplate by using `thiserror` third-party crate which provides a procedural macro to simplify the creation of custom error types.

A Rust package is defined by the [package] section in `Cargo.toml` file (the manifest), can set the package's metadata (name, version, etc.). Inside a package, can have one or more crates, also known as targets. Most common crate types: binary crates and library crates.\

- Binaries: a program that can be compiled to an executable file, must include the `main` function.
- Libraries: not executable on their own, to be imported to another package that depends on it.

Convensions:

- The package's source code is usually in the `/src` directory
- If there is `/src/lib.rs`, `cargo` would infer the package contains a library crate
- If there is `/src/main.rs`, `cargo` would infer the package contains a binary crate

Can override conventions by explicitly declaring targets in `Cargo.toml` file.
A package can have multiple crate, but only one library crate.

### Dependencies

A package can depend on other packages by listing them in the `[dependencies]` section of the `Cargo.toml` file.
Most common way:

```toml
[dependencies]
thiserror = "1"
```

Will add `thiserror` as a dependency with a **minimum** version of `1.0.0`. `thiserror` will be pulled from Rust's official package registry crates.io. When`cargo buid` runs:

- Dependency resolution
- Downloading dependencies
- Compiling project

Dependency resolution is skipped if the project has a `Cargo.lock` file and manifest files are unchanged. `Cargo.lock` contains exact versions of all dependencies used in the project to ensure the same versions are consistently used across different builds. Should commit the `Cargo.lock` file to version control during multi-developers' work.\
Use `cargo update` to update the `Cargo.lock` file with latest compatible versions of all dependencies.

**Path dependencies**: specify a dependency using a path, useful when working on multiple local packages.

```toml
[dependencies]
my-lib = { path = "../my-lib" }
```

**Dev dependencies**: dependencies that are only needed for development, only get pulled in when running `cargo test`.

### `thiserror`

We have implemented the `Error` trait manually for a custom error type, which is a lot of boilerplate. Can use third-party crate `thiserror` to remove some boilerplate, which provides a procedural macro to simplify the creation of custom error types.

```rust
#[derive(thiserror::Error, Debug)]
enum TicketNewErr {
    #[error("{0}")]
    TitleError(String),
    #[error("{0}")]
    DescriptionError(String),
}
```

`thiserror::Error` is an example of a third-party `derive` macro.\
`derive` macros are a subset of procedural macros, to generate Rust code at compile time.

Each procedural macro can define its own syntax. In `thiserror`, we have:

- `#[derive(thiserror::Error)]`: the syntax to derive the `Error` trait for a custom error type.
- `#[error("{0}")]`: the syntax to define a `Display` implementation for each variant of the custom error type. `{0}` is replaced by the zero-th field of the variant when the error is displayed.

### TryFrom trait

We have used he `From` and `Into` traits for infallible type conversions, but some conversions are not guaranteed to succeed:\
Use `TryFrom` and `TryInto` (from `std::convert`) instead, which returns a `Result` type.

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

### Error::source

The `source` method of the `Error` trait:

```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

The `source` method is a way to access the **error cause** (if any)\
Errors are often chained, `source` allows to "walk" the full chain of errors, often used when capturing error context in logs.

**Implementing `source`**: The `Error` trait provides a default impl that always return `None` (no underlying cause). Can override the default implementation to provide a cause:

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

`DatabaseError` wraps an `std::io::Error` as its source.

**`&(dyn Error + 'static)`**: a reference to a trait object that implements the `Error` trait and is valid for the entire program execution.

- `dyn Error` is a trait object refer to any type that implements the `Error` trait.
- `'static` is a special lifetime specifier, implies that the reference is valid for the entire program execution.

**Implementing source using `thiserror`**: a field named `source` will automatically be used as the source of the error; or annotate a field with `#[source]` attribute; or annotate a field with `#[from]` attribute and `thiserror` will also generate a `From` implementation to convert the annotated type into the custom error type.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to connect to the database")]
    DatabaseError {
        //Method 1
        source: std::io::Error
        //Method 2
        #[source]
        inner: std::io::Error
        //Method 3
        #[from]
        inner: std::io::Error
    }
}
```

**`?` operator**: shorthand for propagating errors. When used in a function that returns a `Result`, will return early with an error if the `Result` is `Err`. `?` operator will automatically convert the error type of the fallible operation into the error type of the function if a conversion is possible.

```rust
file.read_to_string(&mut contents)?;
Ok(contents)
```

is equivalent to:

```rust
match file.read_to_string(&mut contents) {
    Ok(_) => (),
    Err(e) => {
        return Err(e);
    }
}
Ok(contents)
```

## Ticket Management

### Arrays

Fixed-size collections of elements of the same type.\
To define an array:\
`let numbers: [u32; 3] = [1;3];` gives an array of `[1,1,1]` named `numbers`.\
To access elements, `numbers[0]` (index must be of type `usize`).\
Rust will panic if the index is out of bound: `numbers[3]`, or can use the `get` method which returns an `Option<&T>`.

### Vectors

The size of an array must be known at compile-time.\
For a growable array, use `Vec` instead.

```rust
let mut numbers: Vec<u32> = Vec::new();
// Or
let mut numbers: Vec<u32> = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
numbers.push(3);
// Or
let mut numbers: Vec<u32> = vec![1,2,3];

let i: &u32 = numbers.get(1); // &2
```

Memory layout:
`Vec` keeps track of three things on the Stack: pointer to the heap, length of the vector & capacity (space reserved on heap) of the vector.\
The elements are stored on the heap.

When inserting an element into a `Vec` that's already at its max capacity, the `Vec` will resize itself.

### Iteration

Rust allows iterating over collections using `for` loops:

```rust
let v: Vec<u32> = vec![1,2,3];
// Or
let v: [u32, 3] = [1,2,3];
for n in v { // Syntax sugar
    println!("{}", n);
}
// Desugar:
let mut iter = IntoIterator::into_iter(v);
loop {
    match iter.next() {
        Some(n) => {
            println!("{}", n);
        }
        None => break,
    }
}
```

`loop` is a looping construct that runs forever unless explicitly `break`.\
The `next` method comes from the `Iterator` trait defined in stdlib for types that can produce a sequence of values:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

The iterator is not guaranteed to exhaust when returns `None`. Only guaranteed if the iterator implements more restrictive `FusedIterator` trait.

**`IntoIterator`** trait:

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

The `into_iter` method consumes the original value and returns an iterator over its elements. To ensure there's no ambiguity as what `for` should desugar to, a type can only have one implementation of `IntoIterator`.\
Every type that implements `Iterator` will automatically implement `IntoIterator` that returns itself as the iterator.

Bounds checks:\
By design, iterating over iterators can't go out of bounds, so it's possible to remove bounds checks from the machine code, making iteration faster.

```rust
let v = vec![1,2,3];
for n in v {
    println!("{}", n);
}
```

is faster than:

```rust
let v = vec![1,2,3];
for i in 0..v.len() {
    println!("{}", v[i]);
}
```

So generally prefer iteration to indexing where possible.

The iterator returns owned values, but if want to iterate over a collection without consuming it, most collections expose an `.iter()` method that returns an iterator over references to the collection's elements.

```rust
let numbers: Vec<u32> = vec![1,2];
// n: &u32
for n in numbers.iter() {
    println!("{}", n);
}
```

### Lifetimes

```rust
impl IntoIterator for &TicketStore {
    type Item = &Ticket;
    type IntoIter = ?;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}
```

What should `type IntoIter` be set to?\
`pub struct Iter<'a, T> { ... }`, of which `'a` is a lifetime parameter.

Lifetimes are labels used by the Rust compiler to keep track of how long a reference is valid. The lifetime of a reference is constrained by the scope of the value it refers to. The compiler always ensure references are not used after the value they refer to has been dropped, to avoid dangling pointers and use-after-free bugs.

Naming is important when have multiple references and need to clarify how references relate to each other. E.g.

```rust
impl <T> Vec<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // ...
    }
}
```

`Vec::iter()` is generic over a lifetime parameter named `'a` -> The `Iter` returned by `iter()` cannot outlive the `Vec` reference (`&self`) it was created from.\
Important because `Vec::iter` returns an iterator over references to the `Vec`'s elements. If the `Vec` is dropped, references will be invalid. Lifetimes are the tool to prevent such errors.

**Lifetime elision rules**: Omit explicit lifetime annotations in many cases. E.g. in `std::vec::Vec`'s source code:

```rust
impl <T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // ...
    }
}
```

Elision rules imply the lifetime of the `Iter` returned is tied to the lifetime of the `&self` reference. `'_` as the placeholder for the lifetime of the &self reference.

Rust's safety concepts:

- Signature-first: Caller's context and function's context are completely separate, caller focuses only on a function's signature, i.e. function's signature must include all information a caller need to call, e.g. Rust compiler will **not** inference the return type.
- Lifetime variance: Longer lifetimes can be safely converted to shorter lifetimes.

```rust
// Function returns local variable's reference
// Illegal operation, can be avoided with function context check
fn return_ref<'a>() -> &'a u32 {
    let num = 12;
    &num
}
// Function takes an external variable reference
// Legal operation, easy to check
fn use_ref<'a>(arg: &'a u32) {
    let v = *arg;
}
// Function takes external references and returns variable references
// More complicated to check, might be legal/illegal
fn use_and_return<'a>(arg: &'a u32) {
    arg
}
```

```rust
fn select_str<'a, 'b, 'c>(s1: &'a str, s2: &'b str) 
-> &'c str 
// Need to give information to the caller about the lifetime of the returned reference
where 'a: 'c, 'b: 'c // longer lifetimes 'implements' shorter lifetimes
{
    if s1.len()>s2.len() {
        s1
    }else{
        s2
    }
}
```

Example of how lifetime specifier works in a function call:

```rust
fn main(){
    let str1 = String::from("abc");
    let result_str;
    {
        let str2 = String::from("wowow");
        result_str = select_str(str1.as_str(), str2.as_str()); // compiler infers result_str's lifetime is the same as str2
        println!("{}", result_str); // works
    }
    println!("{}", result_str); // compile-time error
}
```

协变/covariant: 构造器保持了子类型序关系\
逆变/contravariant：构造器逆转了子类型序关系\
不变/invariant：构造器失去子类型序关系\
安全的类型操作：只读（如参数）协变，只写（如返回值）逆变，读写不变
数组

`struct` uses lifetime specifiers to construct parent/child relationship.\
For life specifier `'a`:
- If all usage of `'a` in `struct S` is covariant to all member variable of `S`, `S` is covariant to `'a`
- If all usage of `'a` in `struct S` is contravariant to all member variable of `S`, `S` is contravariant to `'a`
- Else `S` is invariant to `'a`.

### Combinators

`Iterator` trait has a vast collection of methods to transform, filter, and combine iterators, called **combinators**:

- `map` applies a function to each element of the iterator
- `filter` keeps only the elements that satisfy a predicate
- `filter_map` applies a function to each filtered element
- `cloned` converts an iterator of references into an iterator of values by cloning each element
- `enumerate` returns a new iterator that yields `(index, value)` pairs
- `skip` skips the first `n` elements of the iterator
- `take` stops the iterator after `n` elements
- `chain` combines to iterators into one

Combinators are usually chained together to create complex transformations in a concise and readable way:

```rust
let numbers = vec![1,2,3,4];
// gives the sum of the squares of all even numbers
let outcome:u32 = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .sum();
```

**Closures**: anonymous functions, defined using `|args| body` syntax.\
`filter` and `map` above take closures as arguments.\
Closure = Function + Environment

```rust
// Closures can take more than one argument/capture variables/specify types of arguments & return type
let x = 42;
let add_x = |y: u32| x + y;
// Or
let add_x: fn(y:u32) -> u32 = |y| x + y;
```

After transforming an interator using combinators, can either iterate over the transformed values using a `for` loop or collect them into a collection using the `collect` method.

```rust
let numbers = vec![1,2,3,4];
// gives the Vec containing the squares of all even numbers
let outcome_vec: Vec<u32> = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect();
// Or with turbofish syntax
// <method_name>::<type>()
let outcome_vec = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect::<Vec<u32>>();
```

`collect` is generic over its return type, so need to provide a type hint for the compiler to infer the correct type.

### impl Trait

```rust
impl TicketStore{
    pub fn to_dos<'a>(&'a self) -> Vec<&'a Ticket>{
        self.tickets.iter().filter(|x| x.status == Status::ToDo).collect::<Vec<&Ticket>>()
    }
}
```

The function used in **Combinators** exercise `TicketStore::to_dos` returns a `Vec<&Ticket>`, which introduces a new heap allocation every time `to_dos` is called. It would be better if return an iterator instead of a `Vec`, thus the caller can decide whether to collect the results into a `Vec` or just iterate over them.

The `filter` method returns an instance of `std::iter::Filter`, with signature `pub struct Filter<I, P>` where `I` is the type of the iterator being filtered on and `P` is the  used to filter the elements.\
We know `I` is `std::slice::Iter<'_, Ticket>` but `P` is a closure which doesn't have a name. Use **impl Trait** instead.

`impl Trait` is a feature that allows to return a type without specifying its name. Just declare what trait(s) the type implements and Rust figures out the rest.

```rust
impl TicketStore {
    pub fn to_dos(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|t| t.status == Status::Todo)
    }
}
```

`impl Trait` is NOT a generic parameter/polymorphic. Polymorphic functions generates different implementation for each type parameter, but for `impl Trait` Compiler knows the exact (one) type at compile time. It's just the caller doesn't know.

`impl Trait` can also be used in argument position.

```rust
fn print_iter(iter: impl Iterator<Item = i32>){
    for i in iter{
        println!("{}", i);
    }
}
// Equivalent to
fn print_iter<T>(iter: T) where T: Iterator<Item = i32> {
    for i in iter{
        println!("{}", i);
    }
}
```

Prefer generics over `impl Trait` when used in argument position. Generics allow the caller to explicitly specify the type of the argument using turbofish syntax (`::<>`) which can be useful for disambiguation.

### Slices

`String` is a `Vec<u8>` in disguise. The vector slices for `Vec<T>` are the equivalent of `&str` for `String`.

`[T]` is a slice of a contiguous sequence of elements of type `T`, most commonly used in its borrowed form `&[T]`. To create:

```rust
let numbers = vec![1,2,3];
let slice: &[i32] = &numbers[..];
let slice: &[i32] = numbers.as_slice();
```

`Vec` implements the `Deref` trait using `[T]` as the target type. So can use slice methods on a `Vec` directly.

```rust
let numbers = vec![1,2,3];
// iter() is defined on &[T], but can call on a Vec by deref coercion
let sum: i32 = numbers.iter().sum();
```

`&[T]` is a fat pointer just like `&str`, consisting a pointer to the first element of the slice and the length of the slice.\
Prefer `&[T]` over `&Vec<T>`, as the earlier allows the function to accept any kind of slice.

```rust
// Slice from an array
let source: [i32; 3] = [1,2,3];
// Or from a Vec<i32>
let source: Vec<i32> = vec![1,2,3];
// Both works!
let slice: &[i32] = &array;
```

### Mutable slices

Slices can also be mutable:

```rust
let mut numbers = vec![1,2,3];
let slice: &mut [i32] = &mut numbers;
slice[0] = 42;
```

However, `push` won't work on a mutable slice as it is a method on `Vec` instead of slices. Rust won't allow to add/remove elements from a slice, but only able to modify/replace the elements.\
`&mut Vec` and `&mut String` are more powerful than `&mut [T]` or `&mut str`.

### Two states

We need a new identifier for the ticket, `pub id: TicketId,`, but we don't know the id before creating the ticket, we can make it to be optional: `pub id: Option<TicketId>,`, but handling the `None` case every single time retrieving a ticket is redundant.\
Best solution: Have two different ticket states:

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

### Index trait

`TicketStore::get` returns an `Option<&Ticket>` for a given `TicketId`, we can implement the `Index` trait to allow accessing via `TicketStore[]`.

```rust
pub trait Index<Idx> {
    type Output;
    fn index(&self, index: Idx) -> &Self::Output;
}
```

If want to allow mutability, implement the `IndexMut` trait.

```rust
pub trait IndexMut<Idx>: Index<Idx> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

### HashMap

Current `Index/IndexMut` implementation is not ideal as we have to iterate over the entire `Vec` to retrieve a ticket by id, time complexity O(n). Do better by using a `HashMap<K, V>`, expected cost of insertions, retrievals and removals constant O(1).\
`impl<K, V> HashMap<K, V> where K: Eq + Hash`\
The key must implement the `Eq` and `Hash` traits to enable functions like `get`.

**Hash Trait**: Hash function are functions that map a potentially infinite set of values to a bounded range. Many different hasing functions with different properties (speed, collision risk, reversibility, etc.)\
`HashMap` hashes the key and uses the hash to store/retrieve the associated value. The key must implement `Hash` trait:

```rust
pub trait Hash {
    fn hash<H>(&self, state: &mut H) where H: Hasher;
}
// Most of the time can derive the hash method:
#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
}
```

**Eq Trait**: While `PartialEq` is not enough as it doesn't guarantee reflexivity (`a == a`, e.g. in IEEE754 f32/f64 NAN != NAN), derive `Eq` if all members in the struct have implemented `Eq`.

### BTreeMap/Binary Tree Map

`HashMap` can iterate over the tickets but the order is random. Can recover a consistent ordering by switching from `HashMap` to `BTreeMap`: A `BTreeMap` guarantees entries are sorted by their keys, which is useful when need to iterate over the entries in a specific order or need to perform queries by range.

Like `HashMap`, `BTreeMap` doesn't have trait bounds on the definition of the map, but has on its methods:

```rust
impl<K, V> BTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> where K: Ord, {
        ...
    }
}
```

**`Ord` Trait**: Used to compare for ordering

## Threads

A thread is an execution context managed by the underlying operating system. Each thread has its own stack and instruction pointer. A single process can manage multiple threads sharing the same memory space (i.e. can access the same data).\
Threads are logical construct as a CPU core can only run one set of instructions physically at a time.

**main thread**: When a Rust program starts, the thread created by the OS which is responsible for running the `main` function.

**std::thread**: Rust's standard library's module to create and manage threads.

**`std::thread::spawn`** to create new threads and execute code on them:

```rust
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(3));
            println!("Hello from a thread!");
        }
    });
    
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("Hello from the main thread!");
    }
}
```

**Process termination**: When the main thread finishes, the overall process will exit, thus the spawned thread will be terminated as well.

`join` method to wait for a spawned thread to finish, implemented on the `JoinHandle` that `spawn` returns:

```rust
use std::thread;
fn main(){
    let handle: thread::JoinHandle = thread::spawn(||{
        println!("Hello!");
    })
    handle.join().unwrap();
}
```

### 'static lifetime

The referenced data captured by a closure must be of lifetime `'static`.\

```rust
error[E0597]: `v` does not live long enough
   |
   | pub fn sum(v: Vec<i32>) -> i32 {
   |            - binding `v` declared here
...
   |     let right = &v[split_point..];
   |                  ^ borrowed value does not live long enough
   |     let left_handle = spawn(move || left.iter().sum::<i32>());
   |                             -------------------------------- 
                     argument requires that `v` is borrowed for `'static`
   | }
   |  - `v` dropped here while still borrowed
```

`'static` lifetime is a special lifetime which means the value will be valid for the entire duration of the program.

**Detached threads**: A thread launched by `thread::spawn` can outlive the thread that spawned it.\
Since a thread can outlive its parent thread, it must not borrow any values that might be dropped before the program exits, so the closure passed to it must have the `'static` lifetime:

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{ ... }
```

All values in Rust have a lifetime. A type that owns its data satisfies the `'static` constraint: if own, can keep working with it for as long as you want, even after the function that originally created it has returned. Thus `'static` is either giving **an owned value** or **a reference valid for the entire duration of the program**.

References that are valid for the entire duration of the program\
Most common case: a reference to static data, such as string literals:\
`let s: &'static str = "Hello world!";`, `"Hello world!"` is stored in the executable's read-only data segment, all references pointing to the region will therefore be valid as long as the program runs.

### Leaking data

Main concern around passing references to spawned threads is the use-after-free bugs.\
If working with heap-allocated data, can avoid the issue by intentionally leak the memory, i.e. never reclaim the memory. E.g. with `Box::leak`:

```rust
// Allocate a u32 on heap using Box
let x = Box::new(42u32);
// Memory leak, get a 'static reference
let static_ref: &'static mut u32 = Box::leak(x);
```

If keep leaking memory, will eventually crash with an OOM error. Only when the process exits, the OS will reclaim the memory. It's ok to leak memory when the amount of memory need to leak is bounded/known, or the process is short-lived and confident it won't exhaust all the available memory before exits.

### Scoped threads

Common source of all the lifetime issues before: the spawned thread can outlive the parent. Can sidestep this issue using scoped threads.

```rust
let v = vec![1,2,3];
let midpoint = v.len()/2;
std::thread::scope(|scope| {
    scope.spawn(|| {
        let first = &v[..midpoint];
    });
    scope.spawn(|| {
        let second = &v[midpoint..];
    });
})
```

`std::thread::scope` creates a new **scope**, `std::thread::scope` takes a closure with a single `Scope` instance argument.

`Scope` exposes a `spawn` method, all threads spawned using a `Scope` will be automatically joined when scope ends.

### Channels

All spawned threads have been fairly short-lived as it shut down after running a computation. However we can have a client-server architecture which there will be one long-running server thread for managing the state that stores tickets, and multiple client threads running concurrently to send commands and queries to the server to change/retrieve information.

So far we know how to transfer data between threads by borrowing from parent context and by returning values to the parent when joined, which is not enough for a client-server design as clients need to be able to send and receive data from the server thread after it has been launched.

Rust's stdlib provides **multi-producer single-consumer**(mpsc) channels in `std::sync::mpsc` module, including two channel favlours of bounded and unbounded. Use unbounded for now.

```rust
// Channel creation
use std::sync::mpsc::channel;
let (sender, receiver) = channel();
```

After creation, call `send` on the `sender` to push data into the channel; call `recv` on the `receiver` to pull data from the channel.

In mpsc, `sender` is clonable so we can create multiple senders in different threads and they will push data into the same channel. `receiver` is not clonable and there can only be a single receiver for a channel.

Both `Sender` and `Receiver` are generic over a type parameter `T`. Both can fail as well, `send` returns an error if the receiver has been dropped, while `recv` returns an error if all senders have been dropped and the channel is empty. I.e, error when channel is effectively closed.

### Interior mutability

`Sender`'s `send` signature:

```rust
impl<T> Sender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        // ...
    }
}
```

`send` takes `&self` as an argument, but it's adding a new message to the channel so causing a mutation. And there might be multiple instances of `Sender` trying to modify the channel state at the same time from different threads.\
Question: How are we performing mutations via an immutable reference?

The two types of references are actually more accurately be called as:

- `&T`: shared references (instead of immutable references)
- `&mut T`: exclusive references (instead of mutable references)

**UnsafeCell**: Whenever a type allows to mutate data through a shared reference, that's interior mutability.\
By default, Rust compiler assumes shared references are immutable, and it optimises the code based on that assumption: The compiler can reorder operations, cache values, etc. to make the code faster.\
However, can always wrap the data in an `UnsafeCell` to mutate shared references.\
Using `UnsafeCell`, raw pointers and `unsafe` code, can mutate data through shared references.

`UnsafeCell` and `unsafe` code is an advanced tool to leverage to build safe abstractions whose safety can't be directly expressed in Rust's type system. Whenever use `unsafe` keyword, always know what is going on and never violate the invariants. Every time calling an `unsafe` function, there will be documentation explaining its safety preconditions under what circumstances it's safe to execute the `unsafe` block. Always check.

Examples:\
1. `Rc`\
`Rc`is a reference-counted pointer, wrapping around a value and keeps track of how many references to the value exist, when the last reference is dropped, the value is deallocated. (The value wrapped in `Rc` is immutable, can only get shared references)\
`Rc` uses `UnsafeCell` to allow shared references to increment and decrement the reference count.

```rust
use std::rc::Rc;
let a: Rc<String> = Rc::new("New String".to_string());
assert_eq!(Rc::strong_count(&a), 1);
let b = Rc::clone(&a); // the string data is not copied but referenced
assert_eq!(Rc::strong_count(&a), 2);
assert_eq!(Rc::strong_count(&b), 2);
// a and b point to the same data and share the same reference counter
```

2. `RefCell`\
`RefCell` is one of the most common examples of interior mutability, which allows to mutate the value wrapped in a `RefCell` even if only have the shared reference to the `RefCell` instance. Done by runtime borrow checking: the `RefCell` keeps track of the number and type of references to the value it contains at runtime. If try to  borrow the value mutably while it's already borrowed immutably, the program will panic to ensure Rust's borrowing rules are always enforced.

```rust
use std::cell:RefCell;
let x = RefCell:new(42);
let y = x.borrow(); // shared, immutable borrow
let z = x.borrow_mut(); // exclusive, mutable borrow, not allowed when there's an active immutable borrow, panic!
```

### Two-way communication

Simplest way: include a `Sender` channel in the message that the client sends to server. The server use the channel to send a response back to the client.

Interactions from the client side have been fairly low-level and need to be done manually so far. This is a lot of boilerplate code that could be abstracted away to construct a unified client struct.