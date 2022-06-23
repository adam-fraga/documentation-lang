#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    //Match is a controll flow which compare a value against different pattern.
    //Opposite to if it can return any type not only bool (here the type is Coin)
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            //Arm of branch is composed by a pattern "Coin", separator "=>" and code to exec (1)
            //the code after the separator is an expression it will be return by match if the pattern
            //is matching.
            Coin::Penny => {
                println!("Lucky penny!"); //branch with multiple line
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            //State is here an other type of variable (enum UsState)
            //We can then specifi a coin with more specificities and extract or display state value
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    //Match with Penny
    let coin: u8 = value_in_cents(Coin::Penny);
    //Match with Quarter and retrieve Alabama Type
    let coin2: u8 = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", coin);
    println!("{}", coin2);

    //-------------------------------------------------------------------------------------------

    //Match with option function add ont if x exist and nothing if x does not exist
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //-------------------------------------------------------------------------------------------

    //Match with default case
    let dice_roll = 9; //here user get dice roll 9
    match dice_roll {
        //It's possible to have litteral in pattern like here 3 and 7
        3 => add_fancy_hat(),    //user don't get fancy hat
        7 => remove_fancy_hat(), //user don't remove his fancy hat
        //other here match with all patterns is not 3 or 7 (must be call at last)
        other => move_player(other), //move player is called user move
    }

    fn add_fancy_hat() {
        println!("You win a Fancy hat");
    }
    fn remove_fancy_hat() {
        println!("You lose your Fancy hat");
    }
    fn move_player(num_spaces: u8) {
        println!("User move from {} position", num_spaces);
    }

    //-------------------------------------------------------------------------------------------

    //If the value is not necessary you can use  the "_" which does not bind value
    //This time we don't need the dice roll result we just said the user must reroll
    let dice_roll2 = 9;
    match dice_roll2 {
        3 => add_fancy_hat2(),
        13..=19 => println!("Dice cannot contain that number"), //Range matching
        7 => remove_fancy_hat2(),
        _ => reroll(),
    }

    fn add_fancy_hat2() {
        println!("You win a Fancy hat");
    }
    fn remove_fancy_hat2() {
        println!("You Loose a Fancy hat");
    }
    fn reroll() {
        println!("Reroll untill you get 3 or 7");
    }

    //-------------------------------------------------------------------------------------------

    //If we does not need specify nothin else append we use ()
    //(empty tuple type)
    let dice_roll3 = 9;
    match dice_roll3 {
        3 => add_fancy_hat3(),
        7 => remove_fancy_hat3(),
        2 | 8 => println!("You move 2 or 8 position "),
        _ => (), // if no branch match do nothing
    }

    fn add_fancy_hat3() {
        println!("You Win a Fancy hat");
    }
    fn remove_fancy_hat3() {
        println!("You Loose a Fancy hat");
    }
}
