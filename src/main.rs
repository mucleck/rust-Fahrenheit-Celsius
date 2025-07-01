use std::io;

fn main() {
    loop {
        println!("Select the temperature you want to convert to");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");

        //Recieve user input
        let mut menu_selector: String = String::new();
        io::stdin().read_line(&mut menu_selector).expect("Error when selecting menu");

        let menu_selector: u8 = match menu_selector.trim().parse() {
            Ok(num) => num,
            Err(_) => {continue}
        };

        match menu_selector {
            1 => fahrenheit_to_celsius(),
            2 => celsius_to_fahrenheit(),
            _ => println!("Not a menu option")
        }
        continue;
    }
}

fn fahrenheit_to_celsius(){
    println!("\n------------------------");
    println!("Enter the number to convert to Celsius");
    
    let value_to_convert = match get_user_input() {
        Some(num) => num,
        None => return,
    };

    //Formula para convertirlo (X °F − 32) × 5 / 9

    let converted_value: f64 = (&value_to_convert - 32.0) * (5.0 / 9.0);
    println!("Result is {value_to_convert}F is {converted_value}C");

}

fn celsius_to_fahrenheit(){
    println!("\n------------------------");
    println!("Enter the number to convert to Fahrenheit");

    let value_to_convert = match get_user_input() {
        Some(num) => num,
        None => return,
    };

    //Formula para convertirlo (X °C × 9 / 5) + 32

    let converted_value: f64 = (&value_to_convert * 9.0 / 5.0) + 32.0;
    println!("Result is {value_to_convert}C is {converted_value}F");
}

fn get_user_input() -> Option<f64>{

    let mut value_to_convert = String::new();
    io::stdin().read_line(&mut value_to_convert).expect("Can't get the number");

    match value_to_convert.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Invalid number entered");
            None
        }
    }
}