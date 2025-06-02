# RUST 100 exercises

## 1. Welcome

Function declaration example

```rust
fn <function_name>(<var_identifier: var_type, ...>) -> <return_type>{<body>};
fn greeting() -> &'static str{};
```

## 2. Basic Calculator
Types e.g. u32\
Integers

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

Condition *MUST* be a bool type\
Boolean: true/false\
Comparison operators: ==, !=, <, >, <=, >=\
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

Panic: Signal an unrecoverable error.\
e.g. `100 / 0` -> `thread panicked: attempt to divide by zero`\
Can be intentionally triggered with `panic!("This is a panic")`

While Loop\
`while <condition> { ... }`

Variables are immutable by default, declare the variable as mutable using the `mut` keyword

For Loop: to execute for each element in an iterator\
`for <element> in <iterator> { ... }`\
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

Overflow/Underflow\
NO automatic promotion when overflow happens\
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

Case-by-case wrapping/panicking\
*wrapping_* & *saturating_* methods

```rust
let i:u8 = 255u8.wrapping_add(1u8);
assert_eq!(i, 0); // i=0, wrapped
let u:u8 = 255u8.saturating_add(1u8);
assert_eq!(u, 255); // i=255(u8::MAX), saturated
```

Explicit type conversion:\
`as` casting (infallible) to convert between integer types\
Recommended only convert from smaller type to larger type, or a truncation will occur

```rust
let i = 255u8 as u16; // i=255
let u = 257u16 as u8; // u=1
```

## 3. Ticket v1

Modelling a JIRA-like ticket, with concepts of `struct`, ownership, memory management, modules & visibility, and strings

### Structs

Assume need to keep track of the title, description and status of each ticket\
Start by using a `String` to represent each of them\
`String` - a type defined in stdlib to represent *UTF-8* encoded text\
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

To get proper encapsulation, visibility and modules are needed\
**module** is a way to group related code under a common namespace.\
Modules can be nested, forming a tree structure:\
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

Item paths and `use` statements\
In the same module, can access items directly with their name\
To access an entity from a different module, use a `path` to the entity:

- starting from the root of the current crate,\
`crate::module_name::MyStruct`
- starting from the parent module,\
`super::my_function`
- starting from the current module,\
`sub_module_name::MyStruct`

`crate` and `super` are Rust keywords\
use `use` statement to import an entity\
or use a star import to import all the items from a module\
`use crate::module_1::module_2::*;`\
**(generally discouraged because can pollute the current namespace)**

Can use `cargo-modules` to visualize the module tree structure

### Visibility

By default, everything in Rust is **private**\
A private entity can only be accessed **within the same module or its submodule**\
**Visibility modifiers**

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

If at least one field of a struct is private, the struct cannot be instantiated directly, need to provide public constructors\
Rust does not have a built-in accessor (getter/setter) generator

### Ownership

Ownership is designed to ensure:

- Data is never mutated when it's being read
- Data is never read while it's being mutated
- Data is never accessed after it has been destroyed

Constraints are enforced by the borrow checker (compiler). Ensures memory safety without affecting performance.

Each value has an owner.\
Ownership can be transferred.

```rust
// a is the owner of the String
let a = "hello, world".to_string();
let b = a; // ownership transferred to b
fn foo(x: String){ ... };
foo(b); // move semantics, foo takes ownership of the String
```

To build useful accessor methods we need to start with reference (or all the getters/setters can only run once).

**Borrowing**: to read the value of a variable without taking ownership of it.\
Done via **borrowing (&)**, tagged with privileges:

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

Last-In-First-Out/FIFO\
Used when the data size is known at compile time, e.g. a function is called to push function's arguments, store local variables, etc.\
If there are too many nested function calls, stack memories may be exhausted, causing stack overflow

`std::mem::size_of::<data_type>()` verifies how many bytes would the data type take on the stack

### Heap

Used when the data size is not known at compile time, e.g. dynamically-sized collections, strings, etc.\
Heap: big chunk of memory. When need to store data, use the `allocator` to reserve a subset of the heap (`heap allocation`), returning a `pointer` to the start of the block.\
Need to call the allocator to `free` the memory if no longer need.\
More flexible than stack, but allocation is slower.

`String`'s memory layout:\
The text itself is stored on the heap;\
The pointer to the heap, the length of the String & the capacity of the string (how much bytes reserved) are stored on the stack.\

How much space we need to store pointer, length & capacity? Address are represented with an integer, size = `usize` (u32 on 32-bit machines, u64 on 64-bit machines). Pointers, capacity, length are all represented as `usize`

Most references are represented as a pointer to a memory location (with size `usize`), e.g. &String points to the memory location of the String's metadata (String's pointer in Stack).

### Destructors

**Scope**: the region of code where a variable is valid/alive.\
Starts with the variable declaration, ends when one of the following happens:

- The block {} where the variable declared ends
- Ownership of the variable is transferred to someone else

**Destructor**: Invoked when the owner of a value goes out of scope, used to clean up the resources used by the value (in particular memory allocated to it). Can also manually invoke by passing the variable to `std::mem::drop`, aka "value has been dropped"\
When trasferring the ownership of a value to a function, the responsibility of cleaning it up is also transferred.\
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

Traits are Rust's way of defining **interfaces**\
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

When write `x==y`, compiler will look for implementation of the `PartialEq` trait for the types of `x` and `y` and replace with `x.eq(y)`.\
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

Implementing `PartialEq` is a bit tedious as we have to manually compare each field of tohe struct.\
The implementation is brittle as if the struct definition changes we always have to remember to update the `PartialEq` implementation\
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

**Macros** are code generators.\
**Derive macro** is a particular macro specified as an attribute on top of a struct.\
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

We can define a string literal by enclosing the raw text in double quotes:\
`let s = "Hello World!";` <- type of `s`: `&str`, a ref to a string slice

`&str` and `String` are different types and not interchangeable.

`String`:\
Stack: Metadata (Pointer to heap, length, capacity)\
Heap: Text

**String slices**: `&str` is a view into a String, a reference to a sequence of UTF-8 bytes stored elsewhere.

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from s:
let slice: &str = &s[1..];
```

`&str` In memory:\
Stack: Pointer ponts to the first byte of the slice on the Heap, length.\
Heap: String slice reference doesn't own data on Heap but just points to a `String`'s allocated data.

Use `&str` instead of `&String` whenever need a reference to textual data. `&str` is more flexible and more idiomatic in Rust.\
If a method returns `&String`, it's promised that there is heap-allocated UTF-8 text that matches exactly the same.\
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

**Supertrait/Subtrait** `From: Sized` implies that `From` is a subtrait of `Sized`: any type that implements `From` must also implement `Sized`.\
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

Implies that "`T` may or may not be `Sized`", allows us to bind `T` to a DST (e.g. `Foo<str>`).\
Special case: negative trait bounds are exclusive to `Sized`, can't use them with other traits. 

**&str to String**\
`String` types implement the `From` trait. Thus, can write:\
`let title = String::from("title");`\
or:\
`let title = "title".into();`\
However, `Into<String>` is not implemented for `&str`, but we can still use `into()` on `&str`:\
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

For `From`, `T` is a generic parameter\
For `Deref`, `Target` is an associated type

`Target`(Associated type): at most one implementation. E.g. `String` can only deref to `str`. Uniquely determined by the trait implementation only once.

`T` (Generic traits): can implement `From` multiple times for a type as long as the input type `T` is different. E.g. `impl From<u32> for Wrapping` and `impl From<u16> for Wrapping` (considered as different traits and no ambiguity).

Use an `associated type` when the type must be uniquely determined for a given trait implementation;\
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

`Copy` is another marker trait defined in the stdlib\
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

`Drop` trait is a mechanism to define additional cleanup logic beyond what the compiler does automatically.\
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

`match` is exhaustive, must handle all enum variants, or there will be a compile-time error ("pattern xxx not covered").\
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

`Option` encodes the data that a value must be present(`Some(T)`) or absent(`None`).\
Forces to explicitly handle both cases or there will be a compiler error.\
`Some` is a tuple-like variant that holds unnamed fields. Tuple-like variants are often used when there is a single field to store.

To define tuple-like structs: `struct Point(i32, i32);`
To access: `point.0`/`point.1`...

Tuples: another primitive Rust type, group together a fixed number of values with (potentially different) types.`let i: (i32, &str) = (3, "hello");`

### Fallibility

We use `panic!` when the checks fail in `Ticket::new` before, which is not ideal as it doesn't give the caller a chance to handle the error.\
Use the `Result` type defined in the stdlib instead:

```rust
enum Result<T,E> {
    Ok(T),
    Err(E),
}
```

`Ok(T)` represents a successful operation, `Err(E)` represents a failed operation.\
Rust does not have exceptions but uses `Result`, which forces programmers to encode fallibility in the function's signature.\
`Result` makes fallibility explicit.

### Unwrap

`Result` forces the caller to handle errors at the call site.\
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

There are no constraints on the type of the `Err` variant in a `Result`, but it's a good practice to use a type that implements the `Error` trait `pub trait Error: Debug + Display { ... }`, which is the cornerstone of Rust error handling.\
`Debug` and `Display` both have the only function `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;`, but `Debug` is for developers but `Display` is for end-users. `Debug` can be implemented with #[derive(Debug)] while `Display` must be manually implemented.

### Libraries and packages

It's not very convenient to implement the `Error` trait as we have to implement `Display` as well.\
Can remove some of the boilerplate by using `thiserror` third-party crate which provides a procedural macro to simplify the creation of custom error types.

A Rust package is defined by the [package] section in `Cargo.toml` file (the manifest), can set the package's metadata (name, version, etc.). \
Inside a package, can have one or more crates, also known as targets. Most common crate types: binary crates and library crates.\

- Binaries: a program that can be compiled to an executable file, must include the `main` function.
- Libraries: not executable on their own, to be imported to another package that depends on it.

Convensions:

- The package's source code is usually in the `/src` directory
- If there is `/src/lib.rs`, `cargo` would infer the package contains a library crate
- If there is `/src/main.rs`, `cargo` would infer the package contains a binary crate

Can override conventions by explicitly declaring targets in `Cargo.toml` file.\
A package can have multiple crate, but only one library crate.

