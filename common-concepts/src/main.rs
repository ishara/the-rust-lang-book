fn main() {
    let x: i32 = 5;
    println!("Variable x {}", x);
    // x = 6  cannot assign twice to immutable variable `

    //---------------
    let mut y: i32 = 5;
    println!("Variable y {}", y);
    y = 6; // y mutable
    println!("Variable y {}", y);

    // ----------------------------
    const CONSTENT_VIABLE: i32 = 100_0000;
    println!("Constant Variable = {}", CONSTENT_VIABLE);
    //const mut INVALID_CONSTENT_VIABLE :i32 = 100_0000;  can't mut constant variable

    // -----------------------------
    let z: i32 = 10;
    let z: i32 = 20; //Shadow z variable
    let z: &str = "Shadow variable";

    //-------------------------------

    let v: i32 = 98_123;
    let v: i32 = 0x009;
    let v: i32 = 0o77;
    let v: i32 = 0b1111_0000;
    let v: u8 = b'A';

    // let v:u8 = 259;
    /*
    literal out of range for `u8`
    Note: `#[deny(overflowing_literals)]` on by default
    Note: the literal `259` does not fit into the type `u8` whose range is `0..=255
     */

    let f: f32 = 2.0;
    let g: f32 = 3.0;

    let operator: f32 = f + g;
    let operator: f32 = f - g;
    let operator: f32 = f / g;
    let operator: f32 = f % g;

    let b: bool = true; //true/false
    let c: char = 'A'; // Unicode character

    //----------------

    let tupple: (f32, &str) = (32.0, "test");
    let chanel = tupple;
    let sub_count = chanel.0;

    let array: [i32; 3] = [10, 20, 30];
    let x: i32 = array[0];
    let byte: [i32; 8] = [0; 8];

//     ------------------

    my_function();
    my_function_with_arguments(10, 20);
    let x: i32 = my_function_with_arguments_return(10, 20);
    println!("My function with argument return {} ", x);

    //-----------------

    let number: i32 = 10;
    if (number > 10)
    {} else if number < 10
    {} else {}

    //------
    loop {
        println!("Loop");
        break;
    }

    let mut count :i32 = 0;
    let result :i32 = loop {
        count = count+1;
        if count==10
        {
            break count;
        }
    };
    println! ("Counter : {}",result);

    //------------------

    let mut count:i32 = 3;
    while count > 0 {
        println!("Count {}",count);
        count -=1;
    }
    println!("While end");

    //------------------
    let array: [i32; 5] = [10,20,30,40,40];
    for x in array {
       println!( " X {}" ,x) ;
    }

    for number in 1..4 {
        println!("Y {}", number);
    }
}

fn my_function_with_arguments_return(p0: i32, p1: i32) -> i32 {
    p0 + p1
}

fn my_function_with_arguments(p0: i32, p1: i32) {
    println!("My function with argument {} {}", p0, p1)
}

fn my_function() {
    println!("My function")
}
