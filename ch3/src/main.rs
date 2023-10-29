fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6 This will throw error as x is not mutable
    let mut y = 5;
    y = 6;
    // success in compile as y var is mutable - No errors
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    // Concept of Shadowing
    let z = 1;
    // z = 2 is not allowed but
    let z = z + 1; //is allowed as shadowing(reassign) and will compile
    // e.g ==>
    let spaced = "   ";
    let spaced = spaced.len(); // allowed as reassignment 
    // anoteher case
    let mut space  = "    ";
    // space = space.len(); // will not be allowed and not compile 
    //due to change in var type from str -> int
    
    
    // ## Data Types ##
    
    // Interger => [ i8,i16,i32,i64];
    // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 
    // inclusive, where n is the number of bits that variant uses. 
    // So an i8 can store numbers from -(27) to 27 - 1, 
    // which equals -128 to 127
    
    let var_x = 2.0; // f64
    let var_y: f32 = 3.0; // f32
    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    // Compound Types
    // Compound types can group multiple values into one type. 
    // Rust has two primitive compound types: tuples and arrays.
    
    
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (num_1, num_2, num_3) = tup; // destructure the tuple 
    // or 
    let five_hundred = tup.0; // access the 1st value
    
    //Array
    let a = [1, 2, 3, 4, 5];
    let array :[i32;5] = [6,7,8,9,10]; // type of array with values of type i32 having array len of 5
    
    let fill_vals = [3; 5]; // [3, 3, 3, 3, 3]
    
    // Functions
    
    test_func();
    input_func(10,'S');
    
    let some_value = {
        let y = 10;
        y + 10
    }; // some_value = 20
    
    let ran_value = ret_type(5); // ran_value = 15
    
    // Control Flow
    if some_value > 5 {
        // Do This
    } else {
        // Do This
    }
    
    let condition = true;
    let con_num = if condition { 0 } else {1}; // conditional based assignment 
    // similar to js let var = conditon ? 0 : 1;
    // let number = if condition { 5 } else { "six" }; 
    // Upper thing will give error as it will expect int in else bloack too
    
    // loop {
    //     println!("Infinite loop...")
    // }
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // will return this value after break
        }
    };

    println!("The result is {result}");
    
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn test_func() {
    println!("Calling test func!!")
}

fn input_func(x:i32, c:char) {
    println!("Input value: {x}{c}");
}

fn ret_type(x:i32) -> i32 {
    x + 10
}