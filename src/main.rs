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
}

fn another_function(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}

fn valid_function(x: i32, y: i32) -> i32 {
  x + y
}

// Path: src\main.rs