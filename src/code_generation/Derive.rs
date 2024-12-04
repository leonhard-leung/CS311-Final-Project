// #[derive(Debug, Clone, PartialEq)]
// #[derive(Debug)]
struct User {
    name: String,
    age: u32,
}
impl User {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    //----WE CAN IMPLEMENT CLONE FUNCTION HERE----
}


pub fn main() {
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
    };

    // // Debug trait allows us to print the struct
    //  println!("Print: {:?}", user1);
    //
    // // Clone trait allows us to duplicate the struct
    // let mut user2 = user1.clone();
    // // user2.set_age(300);
    // println!("Clone: {:?}", user2);
    //
    // println!("Are users equal? {}", user1 == user2);
}
