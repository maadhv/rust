fn main(){

    println!("is even: {}", is_even(4));
}

fn is_even(n:i32) -> bool{
    if n % 2 == 0{
        return true;
    }
    else{
        return false;
    }
}