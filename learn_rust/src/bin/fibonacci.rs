fn main(){

    println!("fibonacci of 10 is {}",fibo(10));
}

fn fibo(n: i32)-> i32{
    let mut first = 0;
    let mut second = 1;

    if n == 0{
        return first;
    }else if n == 1{
        return second;
    }

    for i in 1..n-2{
        let next = first + second;
        first = second;
        second = next;
    }
    return second;
}