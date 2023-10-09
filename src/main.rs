use std::time::Instant;

fn main() {
    variables();
    divide_by_zero_help();
    vector();
    double_queue();
    strings();
    slices();
    integers();
    hashmaps();
    tuples();
    structs();
    enums();
    iterator();
}

//////////////////////////
/// Variables
//////////////////////////

fn variables() {
    let a = "555";
    let a = a.to_owned() + "a"; //

    let b = 5.5;
    let b = b + 2.5; // shadowed

    println!("{}, {}", a, b);

    eprintln!("a = {:#?}", a);
}

//////////////////////////
/// Arithmetic
//////////////////////////

// In real code use checked div, which returns an Option
// Some or None that way it is handled
fn divide_by_zero_help() {
    // println!("{}", 1 / 0); // rust prevents this

    let one = 1;
    let zero = 0;
    // println!("{}", one / zero); // Rust prevents this

    let one = 1;
    let zero = one - 1;
    // println!("{}", one / zero); // Rust prevents this

    let one = { || 1 }();
    let zero = { || 0 }();
    // println!("{}", one / zero); // panics
}

//////////////////////////
/// Vectors
//////////////////////////

// A vector is a
fn vector() {
    let mut vec = vec![1, 2, 3];
    vec.resize(10, 0);

    vec.push(3);
    vec[4] = 5;

    println!("{:?}", vec);

    // slice from vec
    let slice = vec.as_slice(); //
    println!("{:?}", slice);
}

//////////////////////////
/// Slices
//////////////////////////

fn slices() {
    // An array of size 64 of 0 unsigned 8 bit
    let array = [0u8; 64];

    // Burrowing the array into a slice
    let slice = &array;

    // Split the slice in half
    let (first_half, second_half) = slice.split_at(32);

    println!(
        "first_half.len()={} second_half.len()={}",
        first_half.len(),
        second_half.len()
    );

    let words = "one,two,three,four";

    // Splitting a string makes a slice
    let wordlist = words.split(',');

    for word in wordlist {
        println!("word={}", word);
    }
}

//////////////////////////
/// Strings
//////////////////////////

// String = owned string, cannot reuse after calling
// &String = burrowed but returned when this exits
fn print_string(s: String) {
    println!("String: {}", s);
}

fn print_str(s: &str) {
    println!("&str: {}", s);
}

fn strings() {
    // str cannot be changed
    let base = "toast";

    let mut a = String::new(); // empty string ready to be changed
    a += "Howdy!";
    println!("{}", a);

    // Convert str to String
    let converted = String::from(base);
    println!("{}", converted);

    // clones the string
    let copy = converted.clone().to_owned();

    // pass String to str with &
    print_str(&copy);
    print_string(copy);
    // print_string(copy); // This will intentionally error: `value moved here` (it is owned by a different fn).
}

//////////////////////////
/// Integers
//////////////////////////

fn integers() {
    let value = 0u8;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0b1u16;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0o2u32;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0x3u64;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 4u128;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));

    println!("Binary (base 2)         0b1111_1111={}", 0b1111_1111);
    println!("Octal (base 8)          0o1111_1111={}", 0o1111_1111);
    println!("Decimal (base 10)         1111_1111={}", 1111_1111);
    println!("Hexadecimal (base 16)   0x1111_1111={}", 0x1111_1111);
    println!("Byte literal            b'A'={}", b'A');
}

//////////////////////////
/// HashMaps
//////////////////////////

fn hashmaps() {
    basic_hashmap();
    compound_key_hashmap();
}

fn basic_hashmap() {
    let mut map = std::collections::HashMap::<&str, u8>::new();
    map.insert("seth", 1);
    map.insert("seth", 2); // overwrite
    map.insert("chelsea", 1); // overwrite
    println!("{:?}", map);
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct CompoundKey {
    name: String,
    value: i32,
}

fn compound_key_hashmap() {
    let mut map = std::collections::HashMap::<CompoundKey, u8>::new();

    map.insert(
        CompoundKey {
            name: String::from("toast"),
            value: 3,
        },
        1,
    );
    println!("{:?}", map);
}

//////////////////////////
/// Queue
//////////////////////////

fn double_queue() {
    let mut dqueue = std::collections::VecDeque::new();
    dqueue.push_back(1);
    dqueue.push_back(1);
    dqueue.push_back(1);
    dqueue.push_front(5);
    println!("{:?}", dqueue);
}

//////////////////////////
/// Tuples
//////////////////////////

fn tuples() {
    let tuple = (1, 2, 3);
    println!("tuple = ({}, {}, {})", tuple.0, tuple.1, tuple.2); //

    let (one, two, three) = tuple;
    println!("tuple destructring one={} two={} three={}", one, two, three);

    // Pattern matching
    match tuple {
        (one, two, three) => println!("pattern {}, {}, {}", one, two, three), //
    }

    return_multiple_values_with_tuple();
}

fn swap<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

fn return_multiple_values_with_tuple() {
    let a = 1;
    let b = 2;

    println!("return multiple values with tuple: {:?}", swap(a, b));
}

//////////////////////////
/// Structs
//////////////////////////

#[derive(Debug)]
struct EmptyStruct {}

#[derive(Debug, Default, Clone)]
struct TupleStruct(String);

fn structs() {
    let empty = EmptyStruct {};
    println!("{:?}", empty);

    let tuple = TupleStruct("example".into());
    println!("{:?}", tuple);

    let defaulted = TupleStruct::default();
    println!("{:?}", defaulted)
}

//////////////////////////
/// Enums
//////////////////////////

#[derive(Debug)]
enum Countries {
    UnitedStates,
    Canada,
    Mexico,
}

fn enums() {
    let country = Countries::Canada;
    println!("{:?}", country);
    println!("{:?}", Countries::UnitedStates as u32);
}

//////////////////////////
/// Alias
//////////////////////////

// Example of making a type alias for ease of use
pub type MyMap = std::collections::HashMap<String, Countries>;

//////////////////////////
/// Iterator
//////////////////////////

fn iterator() {
    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    for i in big_vec {
        if i < 0 {
            println!("this never prints");
        }
    }
    println!("First loop took {}s", now.elapsed().as_secs_f32());

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();

    for i in big_vec.into_iter() {
        if i < 0 {
            println!("this never prints");
        }
    }
    println!("Second loop took {}s", now.elapsed().as_secs_f32());

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec.into_iter().for_each(|i| {
        if i < 0 {
            println!("this never prints");
        }
    });
    println!("Third loop took {}s", now.elapsed().as_secs_f32());
}
