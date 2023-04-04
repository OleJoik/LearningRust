use std::io;

fn main() {
    println!("Hello there! Would you like to transform Celcius to Fahrenheit?");
    println!("Write the temperature you would like to transform here:");

    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Not a valid number format!")
        }
    };

    println!("Your input: {temperature}C");
    let temperature = transform_temperature(temperature);
    println!("The temperature in Fahrenheit is {temperature}F");
}

fn transform_temperature(celcius: f64) -> f64 {
    (celcius * (9.0 / 5.0)) + 32.0
}
