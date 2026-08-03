# Rust Mastery Learning Plan

Goal: become strong enough in Rust to answer deep interview questions, write DSA solutions confidently, and explain how Rust works internally.

Time commitment: 2 hours per day.

Recommended daily structure:

```text
20 min  - Read and understand the concept
40 min  - Code examples by hand
40 min  - Solve DSA/interview problems in Rust
20 min  - Revise by explaining out loud and writing notes
```

Use this file as both a roadmap and a reference. Do not only read it. Rust mastery comes from repeatedly writing code, breaking it, reading compiler errors, and fixing ownership/lifetime problems yourself.

---

## Table Of Contents

- [x] 1. [Rust Mental Model](#rust-mental-model)
- [ ] 2. [Day-Wise Roadmap](#day-wise-roadmap)
  - [x] [Day 1: Setup, Compilation, And Rust Mindset](#day-1-setup-compilation-and-rust-mindset)
  - [x] [Day 2: Variables, Mutability, Types](#day-2-variables-mutability-types)
  - [x] [Day 3: Control Flow](#day-3-control-flow)
  - [x] [Day 4: Functions And Expressions](#day-4-functions-and-expressions)
  - [x] [Day 5: Stack And Heap Basics](#day-5-stack-and-heap-basics)
  - [x] [Day 6: Ownership Rules](#day-6-ownership-rules)
  - [x] [Day 7: Copy And Clone](#day-7-copy-and-clone)
  - [x] [Day 8: Borrowing With References](#day-8-borrowing-with-references)
  - [x] [Day 9: Borrowing Rules](#day-9-borrowing-rules)
  - [x] [Day 10: Slices And String Slices](#day-10-slices-and-string-slices)
  - [x] [Day 11: Structs](#day-11-structs)
  - [x] [Day 12: Enums And Option](#day-12-enums-and-option)
  - [x] [Day 13: Pattern Matching](#day-13-pattern-matching)
  - [x] [Day 14: Lifetimes Basics](#day-14-lifetimes-basics)
  - [x] [Day 15: Generics](#day-15-generics)
  - [x] [Day 16: Traits](#day-16-traits)
  - [x] [Day 17: Trait Bounds And `impl Trait`](#day-17-trait-bounds-and-impl-trait)
  - [x] [Day 18: Important Standard Traits](#day-18-important-standard-traits)
  - [x] [Day 19: Vector Internals](#day-19-vector-internals)
  - [x] [Day 20: Strings In Rust](#day-20-strings-in-rust)
  - [x] [Day 21: HashMap Usage](#day-21-hashmap-usage)
  - [x] [Day 22: HashMap Internals And Collision Handling](#day-22-hashmap-internals-and-collision-handling)
  - [ ] [Day 23: `Result` And Error Handling](#day-23-result-and-error-handling)
  - [ ] [Day 24: Panic And Unrecoverable Errors](#day-24-panic-and-unrecoverable-errors)
  - [ ] [Day 25: Iterators](#day-25-iterators)
  - [ ] [Day 26: Closures](#day-26-closures)
  - [ ] [Day 27: Modules And Visibility](#day-27-modules-and-visibility)
  - [ ] [Day 28: Testing](#day-28-testing)
  - [ ] [Day 29: `Box<T>`](#day-29-boxt)
  - [ ] [Day 30: `Rc<T>`](#day-30-rct)
  - [ ] [Day 31: `RefCell<T>` And Interior Mutability](#day-31-refcellt-and-interior-mutability)
  - [ ] [Day 32: `Rc<RefCell<T>>`](#day-32-rc-refcellt)
  - [ ] [Day 33: `Drop` And RAII](#day-33-drop-and-raii)
  - [ ] [Day 34: `Deref` And Deref Coercion](#day-34-deref-and-deref-coercion)
  - [ ] [Day 35: Advanced Lifetimes](#day-35-advanced-lifetimes)
  - [ ] [Day 36: Threads](#day-36-threads)
  - [ ] [Day 37: `Send` And `Sync`](#day-37-send-and-sync)
  - [ ] [Day 38: `Arc<T>` And `Mutex<T>`](#day-38-arct-and-mutext)
  - [ ] [Day 39: Channels](#day-39-channels)
  - [ ] [Day 40: Async Basics](#day-40-async-basics)
  - [ ] [Day 41: Unsafe Rust](#day-41-unsafe-rust)
  - [ ] [Day 42: Raw Pointers](#day-42-raw-pointers)
  - [ ] [Day 43: Memory Layout](#day-43-memory-layout)
  - [ ] [Day 44: Performance Thinking](#day-44-performance-thinking)
  - [ ] [Day 45: Macros](#day-45-macros)
  - [ ] [Day 46: Arrays, Slices, And Two Pointers](#day-46-arrays-slices-and-two-pointers)
  - [ ] [Day 47: HashMap Problems](#day-47-hashmap-problems)
  - [ ] [Day 48: Stack Problems](#day-48-stack-problems)
  - [ ] [Day 49: Queues, Deques, BFS](#day-49-queues-deques-bfs)
  - [ ] [Day 50: Linked Lists In Rust](#day-50-linked-lists-in-rust)
  - [ ] [Day 51: Trees](#day-51-trees)
  - [ ] [Day 52: Recursion And Backtracking](#day-52-recursion-and-backtracking)
  - [ ] [Day 53: Dynamic Programming](#day-53-dynamic-programming)
  - [ ] [Day 54: Graphs](#day-54-graphs)
  - [ ] [Day 55: Heaps And Priority Queues](#day-55-heaps-and-priority-queues)
  - [ ] [Day 56: Sorting And Searching](#day-56-sorting-and-searching)
  - [ ] [Day 57: Rust Interview Deep Dive](#day-57-rust-interview-deep-dive)
  - [ ] [Day 58: Build A CLI Tool](#day-58-build-a-cli-tool)
  - [ ] [Day 59: Build A DSA Template Library](#day-59-build-a-dsa-template-library)
  - [ ] [Day 60: Mock Interview Day](#day-60-mock-interview-day)
- [x] 3. [Core Rust Concepts](#core-rust-concepts)
- [x] 4. [Memory Management Internals](#memory-management-internals)
- [x] 5. [Ownership, Borrowing, And Lifetimes](#ownership-borrowing-and-lifetimes)
- [x] 6. [Structs, Enums, Pattern Matching](#structs-enums-pattern-matching)
- [x] 7. [Traits And Generics](#traits-and-generics)
- [ ] 8. [Collections And HashMap Internals](#collections-and-hashmap-internals)
- [ ] 9. [Error Handling](#error-handling)
- [ ] 10. [Iterators And Closures](#iterators-and-closures)
- [ ] 11. [Smart Pointers](#smart-pointers)
- [ ] 12. [Concurrency And Async](#concurrency-and-async)
- [ ] 13. [Unsafe Rust](#unsafe-rust)
- [ ] 14. [DSA In Rust](#dsa-in-rust)
- [ ] 15. [Interview Questions](#interview-questions)
- [ ] 16. [Practice Projects](#practice-projects)

---

# Rust Mental Model

Rust is a systems programming language focused on:

- Memory safety without garbage collection
- Zero-cost abstractions
- Fearless concurrency
- Strong compile-time guarantees
- Predictable performance

The most important thing to understand:

```text
Rust does not prevent all bugs.
Rust prevents entire classes of memory and concurrency bugs at compile time.
```

Rust gives you C/C++-level control, but the compiler enforces strict rules so that common bugs like dangling pointers, use-after-free, data races, and double-free are prevented before your program runs.

## The Three Pillars

```text
                 Rust
                  |
     --------------------------------
     |              |               |
 Ownership       Borrowing        Lifetimes
 Who owns data?  Who can access?  How long valid?
```

If you master these three, Rust becomes much easier.

---

# Day-Wise Roadmap

This is a 60-day plan. At 2 hours/day, it gives enough structure for interview preparation and practical mastery.

## Phase 1: Rust Foundations

### Day 1: Setup, Compilation, And Rust Mindset

Learn:

- What Rust is used for
- `rustc`, `cargo`, `rustup`
- Project structure
- `main.rs`, `Cargo.toml`
- `cargo build`, `cargo run`, `cargo test`, `cargo fmt`, `cargo clippy`

Practice:

```bash
cargo new hello_rust
cargo run
cargo build
cargo check
```

Code:

```rust
fn main() {
    println!("Hello, Rust!");
}
```

Interview understanding:

Rust is compiled ahead of time. `cargo check` type-checks without producing the final binary, so it is faster during development.

---

### Day 2: Variables, Mutability, Types

Learn:

- Immutable by default
- `let`, `mut`, `const`
- Scalar types: integers, floats, bool, char
- Compound types: tuple, array
- Type inference
- Shadowing

Code:

```rust
fn main() {
    let x = 5;
    let mut y = 10;
    y += 1;

    let x = x + 1; // shadowing

    let tuple: (i32, f64, bool) = (42, 3.14, true);
    let arr: [i32; 3] = [1, 2, 3];

    println!("x={x}, y={y}, tuple={:?}, arr={:?}", tuple, arr);
}
```

Key point:

```text
mutability belongs to the binding, not the value itself.
```

---

### Day 3: Control Flow

Learn:

- `if`, `else`
- `loop`, `while`, `for`
- `break`, `continue`
- Expressions vs statements

Code:

```rust
fn main() {
    let n = 7;

    let label = if n % 2 == 0 { "even" } else { "odd" };
    println!("{n} is {label}");

    for i in 0..5 {
        println!("{i}");
    }

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 3 {
            break count * 10;
        }
    };

    println!("result={result}");
}
```

Interview point:

In Rust, `if` and `loop` can return values because they are expressions.

---

### Day 4: Functions And Expressions

Learn:

- Function syntax
- Return types
- Implicit return expression
- Statements end with `;`
- Expressions do not need `;`

Code:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(2, 3);
    println!("{result}");
}
```

Mistake to understand:

```rust
fn broken(a: i32, b: i32) -> i32 {
    a + b; // semicolon turns expression into statement
}
```

---

### Day 5: Stack And Heap Basics

Learn:

- Stack memory
- Heap memory
- Fixed-size vs dynamic-size data
- Why `String` uses heap but `i32` does not

Diagram:

```text
Stack                         Heap
-----                         ----
x: 5
s: ptr ---------------------> "hello"
   len: 5
   capacity: 5
```

Code:

```rust
fn main() {
    let x = 5; // stack
    let s = String::from("hello"); // stack metadata + heap buffer

    println!("x={x}, s={s}");
}
```

Interview point:

A `String` value itself contains pointer, length, and capacity on the stack. The actual UTF-8 bytes live on the heap.

---

### Day 6: Ownership Rules

Learn:

Rust ownership rules:

```text
1. Each value has exactly one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.
```

Code:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}"); // error: s1 was moved
    println!("{s2}");
}
```

Diagram:

```text
Before move:
s1 -> heap bytes "hello"

After move:
s1 invalid
s2 -> heap bytes "hello"
```

Interview point:

Rust moves ownership instead of shallow-copying heap-owning values. This prevents double-free.

---

### Day 7: Copy And Clone

Learn:

- `Copy` for cheap stack copies
- `Clone` for explicit duplication
- Why `i32` is `Copy` but `String` is not

Code:

```rust
fn main() {
    let a = 10;
    let b = a; // copy
    println!("a={a}, b={b}");

    let s1 = String::from("rust");
    let s2 = s1.clone(); // deep copy
    println!("s1={s1}, s2={s2}");
}
```

Rule:

```text
Copy = implicit bitwise copy and old variable still usable.
Clone = explicit duplication, may allocate.
```

---

## Phase 2: Borrowing, Lifetimes, And Data Modeling

### Day 8: Borrowing With References

Learn:

- Immutable references `&T`
- Mutable references `&mut T`
- Borrowing without taking ownership

Code:

```rust
fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello");
    let len = length(&s);
    println!("{s} has length {len}");
}
```

Interview point:

References let a function use a value without becoming responsible for freeing it.

---

### Day 9: Borrowing Rules

Learn:

Borrowing rules:

```text
At any time, either:
1. Any number of immutable references, or
2. Exactly one mutable reference.

But not both at the same time.
```

Code:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");

    let r3 = &mut s;
    r3.push_str(" world");
    println!("{r3}");
}
```

Why this exists:

```text
Multiple readers are safe.
One writer is safe.
Readers + writer together can cause data races or invalid reads.
```

---

### Day 10: Slices And String Slices

Learn:

- `&str`
- `&[T]`
- String slices are views into UTF-8 data
- Prefer `&str` over `&String` in function parameters

Code:

```rust
fn first_word(s: &str) -> &str {
    for (i, b) in s.bytes().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    let s = String::from("hello rust");
    println!("{}", first_word(&s));
}
```

Interview point:

`String` owns heap data. `&str` borrows a UTF-8 string slice.

---

### Day 11: Structs

Learn:

- Named structs
- Tuple structs
- Unit-like structs
- Methods with `impl`
- Associated functions

Code:

```rust
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    let user = User::new(String::from("Asha"), 21);
    println!("{}", user.is_adult());
}
```

---

### Day 12: Enums And Option

Learn:

- Enums with data
- `Option<T>`
- No null in safe Rust

Code:

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find(nums: &[i32], target: i32) -> Option<usize> {
    for (i, &num) in nums.iter().enumerate() {
        if num == target {
            return Some(i);
        }
    }
    None
}
```

Interview point:

Rust uses `Option<T>` instead of null. This forces you to handle missing values explicitly.

---

### Day 13: Pattern Matching

Learn:

- `match`
- `if let`
- `while let`
- Destructuring

Code:

```rust
fn describe(num: Option<i32>) {
    match num {
        Some(x) if x > 0 => println!("positive {x}"),
        Some(0) => println!("zero"),
        Some(x) => println!("negative {x}"),
        None => println!("nothing"),
    }
}
```

---

### Day 14: Lifetimes Basics

Learn:

- Lifetimes describe how long references are valid
- Lifetimes do not change runtime behavior
- They are compile-time checks

Code:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Meaning:

```text
The returned reference is valid as long as both x and y are valid.
```

Interview point:

Lifetimes prevent dangling references. They do not keep data alive.

---

## Phase 3: Traits, Generics, Collections

### Day 15: Generics

Learn:

- Generic functions
- Generic structs
- Type parameters
- Monomorphization

Code:

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.get(0)
}
```

Interview point:

Rust generics are usually monomorphized. The compiler generates concrete versions for used types, giving static dispatch and high performance.

---

### Day 16: Traits

Learn:

- Trait definitions
- Trait implementations
- Trait bounds
- Default methods

Code:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}
```

---

### Day 17: Trait Bounds And `impl Trait`

Learn:

- `T: Trait`
- `where` clauses
- `impl Trait` in parameters and return types

Code:

```rust
use std::fmt::Display;

fn print_twice<T: Display>(value: T) {
    println!("{value}");
    println!("{value}");
}
```

---

### Day 18: Important Standard Traits

Learn:

- `Debug`
- `Display`
- `Clone`
- `Copy`
- `Default`
- `PartialEq`, `Eq`
- `PartialOrd`, `Ord`
- `Hash`

Code:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
```

Interview point:

Types used as `HashMap` keys generally need `Eq` and `Hash`.

---

### Day 19: Vector Internals

Learn:

- `Vec<T>` stores elements contiguously on heap
- Pointer, length, capacity
- Reallocation
- Amortized O(1) push

Diagram:

```text
Stack
-----
v: ptr -----------+
   len: 3         |
   cap: 4         |
                  v
Heap        [10][20][30][unused]
```

Code:

```rust
fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    println!("len={}, cap={}", v.len(), v.capacity());
}
```

Interview point:

When capacity is exceeded, `Vec` allocates a larger buffer, moves/copies elements, and frees the old buffer.

---

### Day 20: Strings In Rust

Learn:

- `String` vs `&str`
- UTF-8 encoding
- Indexing strings is not allowed by integer index
- `.chars()`, `.bytes()`

Code:

```rust
fn main() {
    let s = String::from("नमस्ते");

    for ch in s.chars() {
        println!("{ch}");
    }
}
```

Interview point:

Rust strings are UTF-8. Direct indexing like `s[0]` is disallowed because one character may use multiple bytes.

---

### Day 21: HashMap Usage

Learn:

- Insert, get, remove
- Entry API
- Counting frequency

Code:

```rust
use std::collections::HashMap;

fn frequency(nums: &[i32]) -> HashMap<i32, usize> {
    let mut map = HashMap::new();

    for &num in nums {
        *map.entry(num).or_insert(0) += 1;
    }

    map
}
```

---

## Phase 4: Internals, Error Handling, Iterators

### Day 22: HashMap Internals And Collision Handling

Learn:

- Hashing
- Buckets
- Collision
- Rust's `HashMap` uses hashbrown's SwissTable implementation
- Open addressing with probing, not separate chaining

Simplified diagram:

```text
key -> hash -> bucket index

Buckets:
[0] empty
[1] key A
[2] key B
[3] empty
[4] key C

Collision:
key X hashes near bucket 2
bucket 2 occupied
probe next candidate bucket
place key X in another suitable slot
```

Interview answer:

Rust's standard `HashMap` is based on the `hashbrown` crate, which implements Google's SwissTable design. It uses open addressing and SIMD-friendly control bytes. When two keys collide, it probes for another slot rather than storing a linked list in the bucket. Equality is checked after hash matching to confirm the correct key.

Important details:

- Average insert/search/delete: O(1)
- Worst case: O(n)
- Uses `Hash` to compute hash
- Uses `Eq` to confirm key equality
- Default hasher is designed to resist HashDoS attacks, not to be the fastest possible hasher

Code:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);

    if let Some(count) = map.get("apple") {
        println!("apple count = {count}");
    }
}
```

---

### Day 23: `Result` And Error Handling

Learn:

- `Result<T, E>`
- `?` operator
- Recoverable errors
- `unwrap`, `expect`

Code:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

Interview point:

`?` returns early from the function if the result is `Err`, otherwise unwraps the `Ok` value.

---

### Day 24: Panic And Unrecoverable Errors

Learn:

- `panic!`
- Stack unwinding
- Abort strategy
- Difference between `panic` and `Result`

Use `Result` when failure is expected and recoverable. Use panic when the program has reached an invalid state that should not happen.

---

### Day 25: Iterators

Learn:

- `.iter()`
- `.iter_mut()`
- `.into_iter()`
- Lazy evaluation
- Adapters vs consumers

Code:

```rust
fn main() {
    let nums = vec![1, 2, 3, 4];

    let squares: Vec<i32> = nums
        .iter()
        .map(|x| x * x)
        .filter(|x| x % 2 == 0)
        .collect();

    println!("{:?}", squares);
}
```

---

### Day 26: Closures

Learn:

- Closure syntax
- Capture by reference
- Capture by mutable reference
- Capture by move
- `Fn`, `FnMut`, `FnOnce`

Code:

```rust
fn main() {
    let factor = 2;
    let multiply = |x| x * factor;

    println!("{}", multiply(10));
}
```

Interview point:

Closures implement one or more of `Fn`, `FnMut`, and `FnOnce` depending on how they capture variables.

---

### Day 27: Modules And Visibility

Learn:

- `mod`
- `pub`
- `use`
- Crate structure
- Library vs binary crates

Example:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("{}", math::add(2, 3));
}
```

---

### Day 28: Testing

Learn:

- Unit tests
- Integration tests
- `assert_eq!`
- `#[test]`

Code:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

---

## Phase 5: Smart Pointers And Advanced Ownership

### Day 29: `Box<T>`

Learn:

- Heap allocation
- Recursive types
- Trait objects

Code:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

Why `Box` is needed:

```text
Without Box:
List contains List contains List forever -> infinite size.

With Box:
List contains pointer to next List -> known size.
```

---

### Day 30: `Rc<T>`

Learn:

- Reference-counted shared ownership
- Single-threaded only
- Cloning `Rc` increments count

Code:

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("shared"));
    let a = Rc::clone(&data);
    let b = Rc::clone(&data);

    println!("count={}", Rc::strong_count(&data));
}
```

---

### Day 31: `RefCell<T>` And Interior Mutability

Learn:

- Borrowing checked at runtime
- Allows mutation through immutable reference
- Panics on borrow rule violation

Code:

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(5);
    *value.borrow_mut() += 1;
    println!("{}", value.borrow());
}
```

Interview point:

`RefCell<T>` moves borrow checking from compile time to runtime. It is useful when the compiler cannot prove a pattern is safe but you know it is.

---

### Day 32: `Rc<RefCell<T>>`

Learn:

- Shared ownership plus mutation
- Common in graphs and trees
- Risk of reference cycles

Use case:

```text
Graph node needs multiple owners and mutable neighbors.
Rc gives shared ownership.
RefCell gives interior mutability.
```

---

### Day 33: `Drop` And RAII

Learn:

- Resource Acquisition Is Initialization
- Automatic cleanup
- `Drop` trait

Code:

```rust
struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        println!("cleaning up");
    }
}
```

Interview point:

Rust frees memory and resources deterministically when the owner goes out of scope.

---

### Day 34: `Deref` And Deref Coercion

Learn:

- `Deref` trait
- Smart pointer behavior
- `&String` can coerce to `&str`

Code:

```rust
fn print_str(s: &str) {
    println!("{s}");
}

fn main() {
    let s = String::from("hello");
    print_str(&s);
}
```

---

### Day 35: Advanced Lifetimes

Learn:

- Lifetime elision rules
- Structs holding references
- `'static`

Code:

```rust
struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }
}
```

---

## Phase 6: Concurrency And Async

### Day 36: Threads

Learn:

- `std::thread::spawn`
- `join`
- `move` closures

Code:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("from thread");
    });

    handle.join().unwrap();
}
```

---

### Day 37: `Send` And `Sync`

Learn:

- `Send`: type can be moved to another thread
- `Sync`: type can be shared between threads by reference

Interview answer:

A type `T` is `Sync` if `&T` is `Send`. In simple words, if references to a type can be safely shared across threads, it is `Sync`.

---

### Day 38: `Arc<T>` And `Mutex<T>`

Learn:

- Atomic reference counting
- Mutual exclusion
- Shared mutable state across threads

Code:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}
```

---

### Day 39: Channels

Learn:

- Message passing
- Multiple producer, single consumer
- `mpsc`

Code:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("hello")).unwrap();
    });

    println!("{}", rx.recv().unwrap());
}
```

---

### Day 40: Async Basics

Learn:

- `async fn`
- `Future`
- `.await`
- Async needs runtime such as Tokio

Interview point:

An async function returns a future. The future does nothing until it is polled by an executor.

Simplified model:

```text
async fn -> Future state machine -> executor polls -> completes
```

---

## Phase 7: Unsafe, Performance, And Internals

### Day 41: Unsafe Rust

Learn:

Unsafe allows five extra powers:

```text
1. Dereference raw pointers
2. Call unsafe functions
3. Access or modify mutable static variables
4. Implement unsafe traits
5. Access union fields
```

Key point:

```text
unsafe does not disable the borrow checker globally.
It only allows specific operations the compiler cannot verify.
```

---

### Day 42: Raw Pointers

Learn:

- `*const T`
- `*mut T`
- Raw pointers can be null
- Raw pointers are not automatically dereferenced safely

Code:

```rust
fn main() {
    let mut x = 10;
    let ptr = &mut x as *mut i32;

    unsafe {
        *ptr += 1;
    }

    println!("{x}");
}
```

---

### Day 43: Memory Layout

Learn:

- Size and alignment
- Padding
- `repr(C)`
- Niche optimization

Code:

```rust
use std::mem::size_of;

fn main() {
    println!("usize: {}", size_of::<usize>());
    println!("Option<&i32>: {}", size_of::<Option<&i32>>());
}
```

Interview point:

`Option<&T>` is often the same size as `&T` because Rust can use null pointer value as the `None` niche.

---

### Day 44: Performance Thinking

Learn:

- Avoid unnecessary clones
- Prefer borrowing when ownership is not needed
- Understand allocation
- Big-O still matters
- Iterator chains are usually optimized well

Checklist:

- Did this allocate?
- Did this clone?
- Is data contiguous?
- Is this O(n), O(log n), or O(1)?
- Can I borrow instead of own?

---

### Day 45: Macros

Learn:

- `println!`, `vec!`, `format!`
- Declarative macros
- Procedural macros conceptually

Code:

```rust
macro_rules! say_hello {
    () => {
        println!("hello");
    };
}

fn main() {
    say_hello!();
}
```

---

## Phase 8: DSA In Rust

### Day 46: Arrays, Slices, And Two Pointers

Practice:

- Two Sum sorted
- Remove duplicates
- Container with most water

Template:

```rust
fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len().checked_sub(1)?;

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    None
}
```

---

### Day 47: HashMap Problems

Practice:

- Two Sum
- Group Anagrams
- Longest consecutive sequence
- Subarray sum equals K

Code:

```rust
use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let need = target - num;
        if let Some(&j) = seen.get(&need) {
            return Some((j, i));
        }
        seen.insert(num, i);
    }

    None
}
```

---

### Day 48: Stack Problems

Practice:

- Valid parentheses
- Min stack
- Daily temperatures
- Largest rectangle in histogram

Code:

```rust
fn is_valid(s: &str) -> bool {
    let mut stack = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => {}
        }
    }

    stack.is_empty()
}
```

---

### Day 49: Queues, Deques, BFS

Practice:

- Binary tree level order
- Rotting oranges
- Shortest path in grid

Use:

```rust
use std::collections::VecDeque;
```

Template:

```rust
use std::collections::VecDeque;

fn bfs(start: usize, graph: &[Vec<usize>]) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut order = Vec::new();
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &next in &graph[node] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    order
}
```

---

### Day 50: Linked Lists In Rust

Learn:

- Why linked lists are harder in Rust
- Ownership makes pointer-heavy structures explicit
- Use `Option<Box<ListNode>>` in interviews

Code:

```rust
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
```

Interview point:

Rust makes linked lists difficult because ownership of nodes must be explicit. This is good because it prevents dangling pointers and accidental aliasing.

---

### Day 51: Trees

LeetCode-style tree node:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
```

Why this shape:

```text
Rc      -> multiple references to nodes
RefCell -> mutate node contents at runtime
Option  -> child may be absent
```

Practice:

- Max depth
- Inorder traversal
- Validate BST
- Lowest common ancestor

---

### Day 52: Recursion And Backtracking

Practice:

- Subsets
- Permutations
- Combination sum
- N queens

Template:

```rust
fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(i: usize, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            ans.push(path.clone());
            return;
        }

        backtrack(i + 1, nums, path, ans);
        path.push(nums[i]);
        backtrack(i + 1, nums, path, ans);
        path.pop();
    }

    let mut ans = Vec::new();
    let mut path = Vec::new();
    backtrack(0, &nums, &mut path, &mut ans);
    ans
}
```

---

### Day 53: Dynamic Programming

Practice:

- Climbing stairs
- House robber
- Coin change
- Longest increasing subsequence

Code:

```rust
fn climb_stairs(n: usize) -> i32 {
    if n <= 2 {
        return n as i32;
    }

    let mut prev2 = 1;
    let mut prev1 = 2;

    for _ in 3..=n {
        let cur = prev1 + prev2;
        prev2 = prev1;
        prev1 = cur;
    }

    prev1
}
```

---

### Day 54: Graphs

Practice:

- DFS
- BFS
- Number of islands
- Course schedule
- Clone graph

Representations:

```rust
let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![0], vec![0]];
```

Use `HashMap<i32, Vec<i32>>` when node labels are not contiguous.

---

### Day 55: Heaps And Priority Queues

Learn:

- `BinaryHeap`
- Max heap by default
- Use `Reverse` for min heap

Code:

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(2));
    min_heap.push(Reverse(8));

    println!("{:?}", min_heap.pop());
}
```

---

### Day 56: Sorting And Searching

Learn:

- `sort`
- `sort_unstable`
- `binary_search`
- Custom sorting

Code:

```rust
fn main() {
    let mut nums = vec![3, 1, 4, 2];
    nums.sort_unstable();
    println!("{:?}", nums);

    println!("{:?}", nums.binary_search(&3));
}
```

---

## Phase 9: Interview Mastery And Projects

### Day 57: Rust Interview Deep Dive

Be able to answer:

- What is ownership?
- What is borrowing?
- What are lifetimes?
- How does Rust manage heap memory?
- Why does Rust not need garbage collection?
- What is `Copy` vs `Clone`?
- What is `Box`, `Rc`, `Arc`, `RefCell`, `Mutex`?
- How does `HashMap` handle collisions?
- What are `Send` and `Sync`?
- What does unsafe allow?

---

### Day 58: Build A CLI Tool

Project:

Build a command-line todo app.

Requirements:

- Add task
- List tasks
- Mark complete
- Store tasks in a file
- Use `Result` for errors
- Write tests

Concepts used:

- Structs
- Enums
- File I/O
- Error handling
- Modules
- Testing

---

### Day 59: Build A DSA Template Library

Create reusable templates for:

- BFS
- DFS
- Binary search
- Union find
- Trie
- Heap usage
- Sliding window
- Backtracking

Goal:

You should be able to paste these patterns quickly during interviews.

---

### Day 60: Mock Interview Day

Do a 2-hour simulation:

- 30 min: Rust theory questions
- 60 min: 2 DSA problems in Rust
- 20 min: Explain your code out loud
- 10 min: Review mistakes

---

# Core Rust Concepts

## Statements Vs Expressions

Statement:

```rust
let x = 5;
```

Expression:

```rust
5 + 10
```

Blocks are expressions:

```rust
let x = {
    let a = 5;
    a + 1
};
```

## Shadowing Vs Mutability

Shadowing creates a new binding:

```rust
let x = "5";
let x: i32 = x.parse().unwrap();
```

Mutability changes the value through the same binding:

```rust
let mut x = 5;
x += 1;
```

Use shadowing when transforming a value into a new type or new logical state.

---

# Memory Management Internals

## How Rust Handles Heap Memory

Rust does not use garbage collection. It uses ownership and deterministic destruction.

Example:

```rust
fn main() {
    let s = String::from("hello");
    println!("{s}");
} // s goes out of scope, Drop runs, heap memory is freed
```

What happens:

```text
1. String::from allocates bytes on heap.
2. String metadata is stored on stack.
3. The variable s owns the heap allocation.
4. At end of scope, Rust calls Drop for String.
5. Drop frees the heap allocation.
```

Diagram:

```text
Inside main:

Stack                         Heap
-----                         ----
s.ptr ----------------------> h e l l o
s.len = 5
s.cap = 5

After scope ends:

Stack frame gone
Heap allocation freed
```

## Why No Garbage Collector Is Needed

Garbage collectors track which objects are still reachable at runtime. Rust avoids that by deciding ownership at compile time.

```text
GC language:
Runtime periodically checks which heap objects are still reachable.

Rust:
Compiler knows who owns each value.
When owner goes out of scope, cleanup is inserted automatically.
```

## Move Prevents Double-Free

Problem in unsafe shallow-copy languages:

```text
s1.ptr -> heap data
s2.ptr -> same heap data

If both destructors free the same pointer, double-free bug happens.
```

Rust solution:

```rust
let s1 = String::from("hello");
let s2 = s1;
// s1 invalid now
```

Only `s2` will free the heap data.

---

# Ownership, Borrowing, And Lifetimes

## Ownership In Function Calls

```rust
fn take(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("hello");
    take(s);
    // s cannot be used here
}
```

To avoid moving:

```rust
fn borrow(s: &str) {
    println!("{s}");
}

fn main() {
    let s = String::from("hello");
    borrow(&s);
    println!("{s}");
}
```

## Mutable Borrowing

```rust
fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{s}");
}
```

## Lifetimes In Plain English

Lifetime annotations explain relationships between references.

```rust
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

This means:

```text
The returned reference lives as long as x.
It has no relationship with y.
```

Bad code Rust rejects:

```rust
fn dangling() -> &String {
    let s = String::from("hello");
    &s
}
```

Why rejected:

```text
s is destroyed when function ends.
Returning &s would return a dangling reference.
```

Correct version:

```rust
fn not_dangling() -> String {
    String::from("hello")
}
```

---

# Structs, Enums, Pattern Matching

## Struct Use Cases

Use structs when data belongs together.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

## Enum Use Cases

Use enums when a value can be one of several variants.

```rust
enum Payment {
    Cash,
    Card { last_four: String },
    Upi(String),
}
```

## Option And Result

```rust
Option<T> = Some(T) or None
Result<T, E> = Ok(T) or Err(E)
```

Use `Option` for absence. Use `Result` for failure.

---

# Traits And Generics

## Static Dispatch

```rust
fn print<T: std::fmt::Display>(x: T) {
    println!("{x}");
}
```

The compiler generates specialized code for each concrete type used.

## Dynamic Dispatch

```rust
fn print(x: &dyn std::fmt::Display) {
    println!("{x}");
}
```

Uses a trait object and vtable at runtime.

Diagram:

```text
&dyn Trait
   |
   +-- data pointer
   +-- vtable pointer -> method implementations
```

Static dispatch is usually faster. Dynamic dispatch is useful when you need heterogeneous values behind one interface.

---

# Collections And HashMap Internals

## Vec

Use when:

- You need dynamic array behavior
- Fast indexing
- Cache-friendly iteration

Complexities:

```text
push: amortized O(1)
pop: O(1)
index: O(1)
insert/remove middle: O(n)
```

## VecDeque

Use when:

- Need push/pop from both ends
- BFS queue

## HashMap

Use when:

- Need key-value lookup
- Frequency counting
- Index mapping

HashMap collision answer:

```text
Rust HashMap uses SwissTable-style open addressing.
It stores metadata/control bytes to speed lookup.
On collision, it probes other slots.
It compares keys using Eq after candidate hash match.
```

## BTreeMap

Use when:

- Need sorted keys
- Need range queries
- Want deterministic ordering

---

# Error Handling

## `unwrap` Vs `expect` Vs `?`

```rust
value.unwrap(); // panic if Err/None
value.expect("message"); // panic with better message
value?; // return early if Err/None in compatible function
```

Interview rule:

Use `?` in production-style code. Use `unwrap` only when failure is impossible or acceptable in quick scripts/tests.

---

# Iterators And Closures

## Iterator Ownership Difference

```rust
let v = vec![1, 2, 3];

for x in v.iter() {
    // x: &i32
}

for x in v.clone().into_iter() {
    // x: i32
}
```

```text
iter      -> borrow elements
iter_mut  -> mutably borrow elements
into_iter -> consume collection and own elements
```

---

# Smart Pointers

## Quick Comparison

```text
Box<T>        single owner, heap allocation
Rc<T>         multiple owners, single-threaded
Arc<T>        multiple owners, thread-safe
RefCell<T>    runtime borrow checking
Mutex<T>      thread-safe interior mutability
```

## When To Use What

```text
Need heap allocation? Box<T>
Need shared ownership single-threaded? Rc<T>
Need shared ownership multi-threaded? Arc<T>
Need mutation through shared reference single-threaded? RefCell<T>
Need mutation across threads? Mutex<T> or RwLock<T>
```

---

# Concurrency And Async

## Why Rust Is Good At Concurrency

Rust prevents data races at compile time.

Data race requires:

```text
1. Two or more threads access same memory
2. At least one writes
3. No synchronization
```

Rust's ownership and type system prevent this in safe code.

## Threading Choices

```text
thread::spawn        OS thread
Arc<Mutex<T>>        shared mutable state
mpsc::channel        message passing
async/await          concurrent tasks, not necessarily OS threads
```

---

# Unsafe Rust

Unsafe is for cases where you can guarantee safety but the compiler cannot prove it.

Common use cases:

- Building low-level libraries
- FFI with C
- Implementing data structures
- Performance-critical internals

Important:

```text
Unsafe Rust should expose a safe API whenever possible.
```

---

# DSA In Rust

## Common Imports

```rust
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
```

## Sliding Window Template

```rust
use std::collections::HashMap;

fn longest_unique(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut last_seen = HashMap::new();
    let mut left = 0;
    let mut best = 0;

    for right in 0..chars.len() {
        if let Some(&prev) = last_seen.get(&chars[right]) {
            if prev >= left {
                left = prev + 1;
            }
        }
        last_seen.insert(chars[right], right);
        best = best.max(right - left + 1);
    }

    best
}
```

## Binary Search Template

```rust
fn lower_bound(nums: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
```

## Union Find Template

```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let pa = self.find(a);
        let pb = self.find(b);

        if pa == pb {
            return false;
        }

        if self.rank[pa] < self.rank[pb] {
            self.parent[pa] = pb;
        } else if self.rank[pa] > self.rank[pb] {
            self.parent[pb] = pa;
        } else {
            self.parent[pb] = pa;
            self.rank[pa] += 1;
        }

        true
    }
}
```

---

# Interview Questions

## How Does Rust Handle Heap Memory?

Rust handles heap memory through ownership. Heap-owning types like `String`, `Vec`, and `Box` own their allocations. When the owner goes out of scope, Rust automatically calls `Drop`, which releases the heap memory. Moves transfer ownership so only one variable is responsible for freeing the allocation. Borrowing allows temporary access without ownership transfer.

## Why Does Rust Not Have Null?

Rust avoids null references in safe code. Instead, it uses `Option<T>`. This forces the programmer to handle absence explicitly with `match`, `if let`, or combinators.

## What Is The Difference Between Move And Copy?

Move transfers ownership and invalidates the old binding. Copy duplicates simple values and keeps both bindings valid. Types like integers are `Copy`; heap-owning types like `String` are not.

## What Is The Difference Between `String` And `&str`?

`String` owns a growable UTF-8 heap buffer. `&str` is a borrowed view into UTF-8 string data. Use `String` when you need ownership or mutation. Use `&str` when you only need to read string data.

## How Does HashMap Handle Collisions In Rust?

Rust's `HashMap` uses SwissTable-style open addressing. A key is hashed to locate candidate buckets. If a bucket is occupied or multiple keys map nearby, the table probes other slots. It uses hash metadata to speed search and `Eq` to confirm exact key equality. Average operations are O(1), but worst-case can be O(n).

## What Are Lifetimes?

Lifetimes are compile-time annotations that describe how long references are valid and how reference lifetimes relate to each other. They prevent dangling references. Lifetimes do not allocate memory or extend the life of values.

## What Is `Box<T>`?

`Box<T>` is a smart pointer that stores data on the heap with single ownership. It is useful for recursive types, large values, and trait objects.

## What Is `Rc<T>` Vs `Arc<T>`?

`Rc<T>` provides reference-counted shared ownership for single-threaded code. `Arc<T>` provides atomic reference-counted shared ownership for multi-threaded code. `Arc` is thread-safe but has atomic overhead.

## What Is `RefCell<T>`?

`RefCell<T>` provides interior mutability with runtime borrow checking. It allows mutation through an immutable reference, but violating borrow rules causes a runtime panic.

## What Is `Send` And `Sync`?

`Send` means ownership of a value can be transferred to another thread. `Sync` means references to a value can be safely shared across threads.

## What Is Unsafe Rust?

Unsafe Rust allows operations the compiler cannot guarantee are safe, such as dereferencing raw pointers or calling unsafe functions. Unsafe should be minimized and wrapped behind safe abstractions.

---

# Practice Projects

## Project 1: CLI Todo App

Use:

- Structs
- Enums
- File I/O
- `Result`
- Tests

## Project 2: Mini Grep

Build a simplified `grep` command.

Use:

- Command-line arguments
- File reading
- Iterators
- Lifetimes
- Error handling

## Project 3: LRU Cache

Use:

- `HashMap`
- Linked list design discussion
- Ownership tradeoffs
- Performance analysis

## Project 4: Thread Pool

Use:

- Threads
- Channels
- `Arc<Mutex<T>>`
- Graceful shutdown

## Project 5: DSA Notebook

Create one Rust file per topic:

- arrays.rs
- strings.rs
- hashmap.rs
- stack.rs
- queue.rs
- tree.rs
- graph.rs
- dp.rs

---

# Daily Revision Method

At the end of each day, answer these:

```text
1. What did I learn today?
2. What compiler error confused me?
3. Did I understand ownership in today's examples?
4. Could I explain this concept in an interview?
5. Did I solve at least one problem using this concept?
```

Use this explanation method:

```text
Concept -> Example -> Memory behavior -> Interview answer -> DSA use case
```

---

# Final Advice

To master Rust for interviews, focus on these repeatedly:

- Ownership and borrowing until they feel natural
- `String`, `Vec`, `HashMap`, `Option`, `Result`
- Lifetimes enough to explain and debug compiler errors
- Smart pointers and when to use each one
- DSA templates written from memory
- Explaining memory behavior clearly

If someone asks how Rust works internally, do not answer only with syntax. Explain ownership, stack vs heap, moves, drops, borrowing, and compile-time safety guarantees.
