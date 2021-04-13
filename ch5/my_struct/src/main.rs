#[derive(Debug)] //Print debuggin information

//Define a struct
struct Rectangle {
    withd: u32,
    height: u32
}

//function with input struct as reference
fn area (rectangle: &Rectangle) -> u32 {
    rectangle.withd * rectangle.height
}

// Method
impl Rectangle {
    fn area(&self) -> u32 {
        self.withd * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.withd > other.withd && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {withd: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {withd: 30, height: 50};
    let rect2 = Rectangle {withd: 20, height: 40};
    let rect3 = Rectangle::square(60); //Associated function

    println!("Debugging of rect1 {:?}", rect1);
    println!("Full debugging of rect1 {:#?}", rect1);

    println!(
        "The area of rectangle of func is {}",
        area(&rect1)
    );

    println!(
        "The area of rectangle of method is {}",
        rect1.area()
    );

    for i in [&rect2, &rect3].iter() {
        if rect1.can_hold(&i) {
            println!("rect1 can hold {:?}", i);
        } else {
            println!("rect1 can not hold {:?}", i);
        }
    }
}

