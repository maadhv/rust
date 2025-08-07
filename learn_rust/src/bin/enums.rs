enum Barca{
    Player(String,u32),
    Coach(String),
}

impl Barca{
    fn print_player(&self) {
        if let Barca::Player(name,number) = self{
            println!("Player name: {},Jersey number: {}",name,number);
        }
        else{
            println!("This is not a player.");
        }
    }

    fn print_coach(&self) {
        if let Barca::Coach(name) = self{
            println!("Coach name: {}",name);
        }
        else{
            println!("This is not a coach.");
        }
    }
}

fn main(){
    let player1 = Barca::Player(String::from("Pedri"),8);
    player1.print_player();
    let coach= Barca::Coach(String::from("HansiFlick"));
    coach.print_coach();
}