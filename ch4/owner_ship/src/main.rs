fn my_heap() {
    //shadow copy
    let s1 = String::from("Hello world!");
    let s2 = s1; //s1 is moved after borrowed
    println!("s2 is {}", s2);

    //Deep copy
    let s3 = String::from("Hello there!");
    let s4 = s3.clone(); //Deep copy of heap memory so s3 is not moved
    println!("s3 is {}, s4 is {}", s3, s4);

    let s5 = String::from("hello");
    println!("s5 len {}", cal_len(&s5)); //reference to the function

    let mut s6 = String::from("hi"); //Define mutable heap
    change(&mut s6); //function with mutable reference input
    println!("s6 expand is '{}'", s6);

    {
        let s7 = &s6;
        let s8 = &s6;
        // let s9 = &mut s6; //this line will cause error due to it's impossible to have a mutable reference while we have an immutable one.
        println!("s7 is {}, s8 is {}", s7, s8);
    }
    let s10 = &mut s6; //mutable reference of s6 is dropped when going out the {} so we can have mutable reference here
    println!("s10 is {}", s10);
    
    let s11 = no_dangle();
    println!("s11 is {}", s11);

    let mut s12 = String::from("hello world");
    let word = first_word(&s12);
    //s12.clear(); //error due to s12 is being borrowed
    println!("s12 first word is {}", word);
    s12.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}

fn no_dangle() -> String {
    let s = String::from("Hello! no dangle");
    // &s //This would cause dangle
    s //we return the whole string out from the func so there would be no dangle
}

fn change(s: &mut String) {
    s.push_str(" there!");
}

fn cal_len(s: &String) -> usize {
    s.len() //this is expression not statement so there's no ';'
}

fn main() {
    my_heap();
}