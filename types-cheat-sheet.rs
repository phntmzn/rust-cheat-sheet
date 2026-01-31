// types_cheat_sheet.rs
// Rust "types cheat sheet" written in Rust (with tiny examples).
// Paste into main.rs and `cargo run`.
//
// Focus: primitives, strings, slices, tuples, arrays, vecs, options/results,
// refs, pointers-ish, structs/enums, generics/traits, conversions.

use std::collections::{HashMap, HashSet};

fn main() {
    // =========================
    // PRIMITIVES
    // =========================
    let b: bool = true;

    let c: char = 'A';            // 4-byte Unicode scalar value
    let byte: u8 = b'A';          // a single byte literal

    let i: i32 = -123;
    let u: u64 = 123;
    let isz: isize = -1;
    let usz: usize = 10;

    let f1: f32 = 3.14;
    let f2: f64 = 2.71828;

    println!("{b} {c} {byte} {i} {u} {isz} {usz} {f1} {f2}");

    // Integer families:
    // signed:  i8 i16 i32 i64 i128 isize
    // unsigned:u8 u16 u32 u64 u128 usize

    // =========================
    // STRINGS
    // =========================
    let s_str: &str = "hello";           // string slice (borrowed)
    let mut s: String = "hi".to_string(); // owned, growable
    s.push_str(" there");
    println!("{s_str} | {s}");

    // &String coerces to &str automatically in many places
    takes_str(&s);

    // =========================
    // REFERENCES & MUTABILITY
    // =========================
    let mut n: i32 = 5;
    let r1: &i32 = &n;          // shared borrow (read-only)
    let r2: &mut i32 = &mut n;  // exclusive borrow (mutable)
    *r2 += 1;
    println!("r1={r1}, n={n}");

    // =========================
    // UNIT TYPE
    // =========================
    let unit: () = ();
    println!("{unit:?}");

    // =========================
    // TUPLES
    // =========================
    let t: (i32, &str, bool) = (1, "x", true);
    let (a, b2, c2) = t;
    println!("{a} {b2} {c2}");
    println!("t.0 = {}", t.0);

    // =========================
    // ARRAYS & SLICES
    // =========================
    let arr: [i32; 3] = [10, 20, 30]; // fixed-size array
    let slc: &[i32] = &arr[0..2];     // slice view
    println!("arr={arr:?} slc={slc:?}");

    // =========================
    // VEC (growable array)
    // =========================
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    println!("v={v:?}");

    // =========================
    // OPTION / RESULT (very common)
    // =========================
    let maybe: Option<i32> = Some(7);
    let none: Option<i32> = None;
    println!("{maybe:?} {none:?}");

    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("nope");
    println!("{ok:?} {err:?}");

    // =========================
    // STRING/SLICE COLLECTIONS
    // =========================
    // Vec<&str> (borrowed string slices)
    let words: Vec<&str> = vec!["a", "b", "c"];

    // Vec<String> (owned strings)
    let owned_words: Vec<String> = words.iter().map(|w| (*w).to_string()).collect();

    println!("{words:?} {owned_words:?}");

    // =========================
    // HASHMAP / HASHSET
    // =========================
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    map.entry("b").or_insert(2);

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(10);
    set.insert(10);

    println!("map={map:?} set={set:?}");

    // =========================
    // STRUCT / ENUM
    // =========================
    let p = Point { x: 1.0, y: 2.0 };
    println!("point={p:?}");

    let msg = Msg::Move { x: 3, y: 4 };
    println!("msg={:?}", describe(msg));

    // =========================
    // GENERICS
    // =========================
    let a = id(9);
    let b = id("hi");
    println!("id: {a} {b}");

    // =========================
    // TRAIT OBJECT (dynamic dispatch)
    // =========================
    let things: Vec<Box<dyn Speak>> = vec![Box::new(7i32), Box::new(String::from("yo"))];
    for t in things {
        println!("speak: {}", t.speak());
    }

    // =========================
    // COMMON CONVERSIONS
    // =========================
    let num: i32 = "123".parse().unwrap(); // str -> i32
    let s2: String = num.to_string();      // i32 -> String
    println!("{num} -> {s2}");

    // Into / From (idiomatic)
    let s3: String = "hey".into();
    println!("{s3}");

    // =========================
    // OWNERSHIP CLONES (when needed)
    // =========================
    let orig = String::from("data");
    let copy = orig.clone(); // deep copy
    println!("{orig} {copy}");
}

// --------- helpers / types ---------
fn takes_str(s: &str) {
    println!("takes_str: {s}");
}

#[derive(Debug)]
struct Point<T = f64> {
    x: T,
    y: T,
}

#[derive(Debug)]
enum Msg {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}

fn describe(m: Msg) -> &'static str {
    match m {
        Msg::Quit => "quit",
        Msg::Write(_) => "write",
        Msg::Move { .. } => "move",
    }
}

fn id<T>(x: T) -> T {
    x
}

trait Speak {
    fn speak(&self) -> String;
}

impl Speak for i32 {
    fn speak(&self) -> String {
        format!("num {self}")
    }
}

impl Speak for String {
    fn speak(&self) -> String {
        format!("str {self}")
    }
}
