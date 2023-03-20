fn main() {
  let a = 10; // a is immutable
  let mut b = 20; // b is mutable

  println!("a = {}", a);
  println!("b = {}", b);

  b = 30;

  println!("b = {}", b);

  const HIGEST_PRICE: u32 = 10_000_000; // HIGEST_PRICE is constant

  println!("HIGEST_PRICE = {}", HIGEST_PRICE);

  let x = 10; // x is immutable

  println!("x = {}", x);

  let x = "ten"; // x is shadowed

  println!("x = {}", x);

  // scalar types: integers, floating numbers, booleans, and characters
  let int: i32 = 10; // i32
  let float: f64 = 10.0; // f64
  let valid: bool = true; // bool
  let invalid: bool = false; // bool
  let character: char = 'a'; // char
  let string: &str = "a"; // &str

  println!("int = {}", int);
  println!("float = {}", float);
  println!("valid = {}", valid);
  println!("invalid = {}", invalid);
  println!("character = {}", character);
  println!("string = {}", string);

  // compound types: tuples and arrays
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup; // destructuring
  let five_hundred = tup.0; // access by index
  let six_point_four = tup.1; // access by index
  let one = tup.2; // access by index

  println!("x = {}", x);
  println!("y = {}", y);
  println!("z = {}", z);
  println!("five_hundred = {}", five_hundred);
  println!("six_point_four = {}", six_point_four);
  println!("one = {}", one);

  let tup2 = (); // unit type

  println!("tup2 = {:?}", tup2);

  let a = [1, 2, 3, 4, 5]; // array
  let first = a[0]; // access by index
  let second = a[1]; // access by index
  let third = a[2]; // access by index
  let fourth = a[3]; // access by index
  let fifth = a[4]; // access by index

  println!("first = {}", first);
  println!("second = {}", second);
  println!("third = {}", third);
  println!("fourth = {}", fourth);
  println!("fifth = {}", fifth);

  let a = [3; 5]; // array with 5 elements and each element is 3
  let first = a[0]; // access by index
  let second = a[1]; // access by index
  let third = a[2]; // access by index
  let fourth = a[3]; // access by index
  let fifth = a[4]; // access by index

  println!("first = {}", first);
  println!("second = {}", second);
  println!("third = {}", third);
  println!("fourth = {}", fourth);
  println!("fifth = {}", fifth);

  // functions
  another_function(5, 6);

  let res = valid_function(1, 2);
  println!("res = {}", res);

  // inline comments
  /*
    block comments
  */

  // control flow
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else if number % 3 == 0 {
    println!("condition is divisible by 3");
  } else {
    println!("condition was false");
  }

  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {}", number);

  // loops
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {}", result);

  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("LIFTOFF!!!");

  let a = [10, 20, 30, 40, 50];

  for element in a {
    println!("the value is: {}", element);
  }

  for number in 1..=5 {
    println!("{}!", number);
  }

  println!("LIFTOFF!!!");

  // ownership
  {
    let s1 = "hello"; // s1 comes into scope

    println!("{}", s1); // s1 is valid here
  } // s1 goes out of scope and is no longer valid here

  // string literals vs string slices
  //let mut s1 = "hello"; // string literal
  let mut s2 = String::from("hello"); // string slice

  //s1.push_str(", world!"); // error!
  s2.push_str(", world!"); // push_str() appends a literal to a String

  // Copy trait
  let x = 5; // x comes into scope
  let y = x; // x is copied to y

  println!("x = {}, y = {}", x, y); // both are valid here

  let s1 = String::from("hello"); // s1 comes into scope
  let s2 = s1; // s1 is moved to s2

  //println!("{}, world!", s1); // error! s1 is no longer valid here
  println!("{}, world!", s2); // s2 is valid here

  let s1 = String::from("hello"); // s1 comes into scope
  let s2 = s1.clone(); // s1 is cloned to s2

  println!("s1 = {}, s2 = {}", s1, s2); // both are valid here

  // ownership and functions
  let s = String::from("hello"); // s comes into scope
  let s2 = s.clone(); // s is cloned to s2

  takes_ownership(s); // s's value moves into the function...
                      // ... and s is no longer valid here

  //println!("{}", s); // error! s is no longer valid here
  println!("{}", s2); // s2 is valid here

  let x = 5; // x comes into scope

  makes_copy(x); // x would move into the function,
                 // but i32 is Copy, so it's okay to still
                 // use x afterward

  println!("{}", x); // x is still valid here

  // return values and scope
  let s1 = gives_ownership(); // gives_ownership moves its return
                              // value into s1

  println!("{}", s1); // s1 is valid here

  let s2 = String::from("world"); // s2 comes into scope

  let s3 = takes_and_gives_back(s2); // s2 is moved into
                                     // takes_and_gives_back, which also
                                     // moves its return value into s3

  //println!("{}", s2); // error! s2 is no longer valid here
  println!("{}", s3); // s3 is valid here

  // references and borrowing
  let s1 = String::from("hello"); // s1 comes into scope
  let (s2, len) = calculate_length(&s1); // s2 is a reference to s1 and len is the length of s1

  println!("The length of '{}' is {}.", s2, len);
}

fn another_function(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}

fn valid_function(x: i32, y: i32) -> i32 {
  x + y // return value
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn gives_ownership() -> String {
  let some_string = String::from("hello"); // some_string comes into scope

  some_string // some_string is returned and
              // moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> (&String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length) // s is returned and moves out to the calling function
}

// Path: src\main.rs