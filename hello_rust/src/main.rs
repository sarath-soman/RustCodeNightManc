fn takes_ownership(s: String) {
    println!("{}", s);
}

fn borrows(s: &String) {
    println!("{}", s);
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; //move: s1 is now invalid
    println!("{}", s2);
    takes_ownership(s2);
    // println!("{}", s2);

    let s3 = String::from("world");
    borrows(&s3); //borrow: s3 still valid
    println!("{}", s3);

    //Uncomment to see compiler error:
    // println!("{}", s1);
}