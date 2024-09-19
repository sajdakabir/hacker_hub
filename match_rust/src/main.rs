#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
   
//    let coin = Coin::Penny;
//    let coin = Coin::Quarter(UsState::Alabama);
   let coin = Coin::Quarter(UsState::Alaska);
    let x= get_coin(coin);
    println!("the {x}");
}

fn get_coin(coin:Coin)->u8{
    match coin {
        Coin::Penny=> {
            println!("this is prnny");
            1
        },
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter(UsState::Alaska)=>{
            println!("this hi from Alaska");
            25
        },
        Coin::Quarter(state)=>{
            println!("the Q is {state:?}");
            25
        }
    }
}
