fn main() {
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }
    //
    // fn value_in_cents(coin: Coin) -> u32 {
    //     match coin {
    //         Coin::Penny => {
    //             println!("Lucky Penny");
    //             1
    //         },
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    //
    // }

    // #[derive(Debug)]
    // enum UsState {
    //     Alabama,
    //     Alaska,
    //     // ...etc
    // }
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter(UsState),
    // }
    // fn value_in_cents(coin: Coin) -> u32 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter(state) => {
    //             println!("State Quarter from {:?}!", state);
    //             25
    //         },
    //     }
    // }
    // value_in_cents(Coin::Quarter(UsState::Alaska));

    // // Option형을 사용하는 방법
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i + 1),
    //     }
    // }
    //
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
    //
    // // println!("{:?}", five);
    // // println!("{:?}", six);
    // // println!("{:?}", none);

    // Placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
