use std::io;

fn convert(celcius: f32) -> f32{
    celcius * 1.8 + 32.0
}

fn main(){

    println!("enter temperature in celcius: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");

    let celcius: f32 = input.trim().parse().expect("enter a valid value");
    println!("{} celcius in farenheit is {}",celcius,convert(celcius));
}