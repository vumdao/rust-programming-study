//#[derive(Debug)]

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

fn some_value(x: Option<u8>) {
    match x {
        Some(0) => println!("value is 0"),
        Some(1) => println!("Value is 1"),
        _ => println!("Nothing matches"),
    }
}

fn main() {
    //Matching Option<T> type
    some_value(Some(1));

    //Some in Option<T> type
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => ()
    }

    //Hanlde pattern None
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
