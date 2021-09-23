fn main() {

    // ---------------------------------------- if ---------------------------------------------
    // -----------------------------------------------------------------------------------------
    // ----------------------------------------------------------------------------------------

    let number = 8;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4 or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    //let number = if condition { 5 } else { "six" }; Error!
    println!("The value of number is: {}", number);

    // -------------------------------- loop ----------------------------------------------
    // -----------------------------------------------------------------------------------
    // -----------------------------------------------------------------------------------

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // --------------------------------------- while -----------------------------------------
    // ----------------------------------------------------------------------------------------
    // ---------------------------------------------------------------------------------------- 

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    };
    println!("LIFTOFF!!!");

    // ------------------------------------------------ for -----------------------------------------------------
    // --------------------------------------------------------------------------------------------------------------
    // ------------------------------------------------------------------------------------------------------------

    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // ---------------------------------------------------- fn ---------------------------------------------------------
    // -----------------------------------------------------------------------------------------------------------------
    // --------------------------------------------------------------------------------------------------------------

    println("{}", add(10,20));

    let point = (3, 5);
    print_coordinates(&point);


}

fn add(a: i32, b: i32) -> i32 {
    a+b
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("location: ({},{})", x, y);
}
