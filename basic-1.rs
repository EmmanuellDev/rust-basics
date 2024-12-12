fn main() {
    //VARIABLES
    let x = 5; // immutable variable
    let mut y = 10; //mutable variable
    println!("{}", y);
    y = 9;
    println!("{}", y);

    //DATA TYPES
    // Scalar Type
    // Integer
    let x1: i8 = 3;
    let x2: i16 = 3;
    let x3: i32 = 3;
    let x4: i64 = 3;
    let x5: i128 = 3;
    let x6: isize = 3;

    println!("{x}");
    println!("{x1}");
    println!("{x2}");
    println!("{x3}");
    println!("{x4}");
    println!("{x5}");
    println!("{x6}");

    // Floating Point
    let v1: f32 = 7.5;
    let v2: f64 = 7.5;

    println!("{v1}");
    println!("{v2}");

    // Boolean
    let ismen: bool = true;

    println!("{ismen}");

    // Character
    let s: char = 'E';

    println!("{s}");

    // String
    let ss: &str = "I am Emmanuel";

    println!("{ss}");
    // Compound Type
    // Tuples

    let person: (&str, i32, f64) = ("Alice", 30, 5.5);
    println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);

    // Array
    let numbers = [10, 20, 30, 40, 50];
    println!("First Number: {},{},{}", numbers[0], numbers[3], numbers[2]);
    println!("All Numbers: {:?}", numbers);

    // COMMENTS
    // This is a single-line comment

    /* This
    is
    a
    multi-line
    comment */
    
    // OPERATORS
    let mut a: i32 = 5;
    let b: i32 = 7;
    let a1: bool = true;
    let a2: bool = false;

    // Arithmetic Operator
    println!("{} + {} = {}",a, b, a+b); // Addition
    println!("{} + {} = {}",a, b, a-b); // Subtraction
    println!("{} + {} = {}",a, b, a*b); // Multiplication
    println!("{} + {} = {}",a, b, a/b); // Division
    println!("{} + {} = {}",a, b, a%b); // Modulus

    // Comparison Operator
    println!("{}", a == b);
    println!("{}", a != b);

    // Logical Operator
    println!("{}", a1&&a2);
    println!("{}", a1||a2);
    println!("{}", !a1);

    // Bitwise Operator
    println!("{}", a&b);
    println!("{}", a|b);
    println!("{}", a^b);
    println!("{}", a<<b);
    println!("{}", a>>b);

    // Assignment Operator
    a+=b;
    println!("{a}");
    a-=b;
    println!("{a}");
    a*=b;
    println!("{a}");
    a/=b;
    println!("{a}");
    a%=b;
    println!("{a}");

    // Miscellaneous Operator
    let abc = ['2','4','6'];
    println!("{}", abc.len());
    // -> pointer
    // 1..5 range

    // CONDITIONAL STATEMENTS
    // if, else if, else
    let age: i64 = 20;

    if age > 18 {
        println!("You can vote");
    }
    else if age < 18 {
        println!("You cannot vote");
    }
    else {
        println!("Enter a valid age");
    }

    // match (switch-case)
    let num = 2;

    match num {
        1 => println!("This is 1"),
        2 => println!("This is 2"),
        3 => println!("This is 3"),
        _ => println!("This is 4"),
    }

    // ITERATIVE STATEMENTS
    // for loop, while loop, loop
    let mum = [2,4,6,8,10];
    for nums in mum {
        println!("{}", nums);
    }

    let mut num = 5;

    while num > 0 {
        println!("Countdown: {}", num);
        num -= 1;
    }

    let mut count = 0;
    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 5 {
            break;
        }
    }
    
    // FUNCTIONS
    greet();

    // FUNCTION WITH PARAMETERS
    add(4,7);

    // FUNCTION WITH RETURN VALUE
    let result = square(4);
    println!("{result}");

    //CLOSURES (ANONYMOUS FUNCTION)
    let clousre = |r: i32, s: i32| r+s;
    println!("{}", clousre(4,2));
}

fn greet() {
    println!("Hello, Emman!");
}

fn add(a: i32, b: i32) {
println!("{}", a+b);
}

fn square(num: i32) -> i32 {
    num * num
}
