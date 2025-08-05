struct User{
    name: String,
    age: u32,
    goals: u32,
}

impl User{
    fn goal_ratio(&self) -> i32{
        (self.goals * self.age) as i32
    }
}

fn main(){
    let user1 = User{
        name : String::from("Pedri"),
        age: 20,
        goals: 10,
    };

    println!("Username: {}, age: {}",user1.name,user1.age);
    println!("Goal ratio of {} is {}",user1.name,user1.goal_ratio());
}