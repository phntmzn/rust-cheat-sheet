// rust_cheat_sheet.rs
// A "cheat sheet" written as runnable Rust. Paste into main.rs and `cargo run`.
//
// Tip: search within this file for: OWNERSHIP, STRINGS, VEC, HASHMAP, RESULT, TRAITS.

use std::collections::HashMap;

fn main() {
    // =========================
    // 1) Variables + Types
    // =========================
    let x = 5;               // immutable
    let mut y: i32 = 10;     // mutable
    y += 1;

    const MAX: i32 = 99;     // constants must have a type
    let big = 1_000_000u64;  // underscores ok

    println!("x={x}, y={y}, MAX={MAX}, big={big}");

    // =========================
    // 2) Functions
    // =========================
    println!("add(2,3)={}", add(2, 3));

    // =========================
    // 3) Control Flow
    // =========================
    let n = 7;
    let v = if n > 5 { "big" } else { "small" };
    println!("n is {v}");

    for i in 0..3 {
        println!("for i={i}");
    }

    let mut i = 0;
    while i < 2 {
        println!("while i={i}");
        i += 1;
    }

    let mut loops = 0;
    loop {
        loops += 1;
        if loops == 2 {
            break;
        }
    }
    println!("looped {loops} times");

    // =========================
    // 4) Ownership + Borrowing (OWNERSHIP)
    // =========================
    let s = String::from("hello");
    borrow_str(&s);              // borrow immutably (no move)
    // takes_ownership(s);       // would move `s`
    println!("still have s: {s}");

    let mut t = String::from("yo");
    borrow_mut(&mut t);
    println!("after borrow_mut: {t}");

    // Copy vs Move
    let a = 123i32;      // Copy
    let b = a;           // copied
    println!("a={a}, b={b}");

    let v1 = vec![1, 2]; // Move (Vec not Copy)
    let v2 = v1;         // moved
    // println!("{:?}", v1); // error: use of moved value
    println!("v2 moved ok: {:?}", v2);

    // =========================
    // 5) Strings (STRINGS)
    // =========================
    let mut name = String::new();
    name.push_str("phntm");
    name.push('z');
    let greet = format!("hi, {}", name);
    println!("{greet}");

    let s2: &str = "borrowed str slice";
    println!("{s2}");

    // UTF-8 slicing: be careful. This is safe for ASCII:
    let ascii = String::from("hello");
    let slice = &ascii[0..2];
    println!("slice: {slice}");

    // =========================
    // 6) Collections: Vec (VEC)
    // =========================
    let mut nums = vec![1, 2, 3];
    nums.push(4);

    // indexing panics if out of bounds:
    let first = nums[0];
    // safe access:
    let maybe_first = nums.get(0); // Option<&i32>

    println!("nums={:?}, first={first}, maybe_first={maybe_first:?}");

    for n in &nums {
        print!("{n} ");
    }
    println!();

    for n in &mut nums {
        *n += 10;
    }
    println!("nums after +10: {:?}", nums);

    // =========================
    // 7) Collections: HashMap (HASHMAP)
    // =========================
    let mut m: HashMap<&str, i32> = HashMap::new();
    m.insert("a", 1);
    m.entry("b").or_insert(2);
    let a_val = m.get("a"); // Option<&i32>
    println!("map={:?}, a={a_val:?}", m);

    // =========================
    // 8) Structs + impl
    // =========================
    let mut user = User::new("alex", 20);
    user.birthday();
    println!("user: {:?}, greet={}", user, user.greet());

    // =========================
    // 9) Enums + match
    // =========================
    handle(Msg::Write("hey".into()));
    handle(Msg::Move { x: 3, y: 4 });
    handle(Msg::Quit);

    // =========================
    // 10) Option + Result (RESULT)
    // =========================
    let o = maybe_pos(-1);
    println!("maybe_pos(-1)={o:?}");

    match parse_i32("123") {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => println!("parse error: {e}"),
    }

    // ? operator demo (propagate errors)
    // In main you can’t use `?` unless main returns Result. Here’s a tiny wrapper:
    match wrapper_using_q() {
        Ok(n) => println!("wrapper_using_q ok: {n}"),
        Err(e) => println!("wrapper_using_q err: {e}"),
    }

    // =========================
    // 11) Generics + Traits (TRAITS)
    // =========================
    println!("id(9)={}", id(9));
    println!("id(\"hi\")={}", id("hi"));

    let spk: i32 = 42;
    println!("Speak: {}", spk.speak());

    // =========================
    // 12) Lifetimes (minimal)
    // =========================
    let longer = pick_longer("short", "looooong");
    println!("longer: {longer}");

    // =========================
    // 13) Pattern tricks
    // =========================
    let (p, q) = (1, 2);
    println!("tuple destructure: p={p}, q={q}");

    if let Some(x) = Some(5) {
        println!("if let got {x}");
    }

    match 5 {
        1..=3 => println!("1..=3"),
        4 | 5 => println!("4 or 5"),
        _ => println!("other"),
    }
}

// -------------------------
// Functions
// -------------------------
fn add(a: i32, b: i32) -> i32 {
    a + b // last expression is return value (no semicolon)
}

// Borrow immutably
fn borrow_str(s: &String) {
    println!("borrowed: {s}");
}

// Borrow mutably
fn borrow_mut(s: &mut String) {
    s.push('!');
}

// Takes ownership (moves)
#[allow(dead_code)]
fn takes_ownership(s: String) {
    println!("owned: {s}");
}

// -------------------------
// Structs + impl
// -------------------------
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: &str, age: u32) -> Self {
        Self { name: name.into(), age }
    }

    fn birthday(&mut self) {
        self.age += 1;
    }

    fn greet(&self) -> String {
        format!("hello {}, age {}", self.name, self.age)
    }
}

// -------------------------
// Enums + match
// -------------------------
enum Msg {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}

fn handle(m: Msg) {
    match m {
        Msg::Quit => println!("quit"),
        Msg::Write(s) => println!("write: {s}"),
        Msg::Move { x, y } => println!("move: {x},{y}"),
    }
}

// -------------------------
// Option / Result
// -------------------------
fn maybe_pos(n: i32) -> Option<i32> {
    if n > 0 { Some(n) } else { None }
}

fn parse_i32(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

// Using `?` to bubble errors up:
fn wrapper_using_q() -> Result<i32, std::num::ParseIntError> {
    let n: i32 = "77".parse()?;
    Ok(n + 1)
}

// -------------------------
// Generics + Traits
// -------------------------
fn id<T>(x: T) -> T {
    x
}

trait Speak {
    fn speak(&self) -> String;
}

impl Speak for i32 {
    fn speak(&self) -> String {
        format!("num {}", self)
    }
}

// -------------------------
// Lifetimes (minimal example)
// -------------------------
fn pick_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

// -------------------------
// Tests (run: `cargo test`)
// -------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_pick_longer() {
        assert_eq!(pick_longer("aa", "bbbb"), "bbbb");
    }
}
