use std::io;


fn main() {
    loop{
        println!("Fahrenheit value: ");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature).expect("Incorrect line!");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please type numerical");
                continue
            }
        };

        let temperature: f64 = ((temperature - 32.0) * (5.0/9.0)).into();
        println!("Celcius value is {temperature}")
    }
}
