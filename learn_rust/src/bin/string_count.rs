fn count(s: &str)-> usize {
    s.chars().count()
}

fn main(){
    let name = "hello world";
    println!("The string '{}' has {} characters",name,count(&name));
}