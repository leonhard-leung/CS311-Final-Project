// Generating repetitive code with just one struct
macro_rules! create_structs {
    ($($name:ident { $($field:ident: $type:ty),* $(,)? }),*) => {
        $(
            #[derive(Debug, Clone, PartialEq)]
            struct $name {
                $(
                    $field: $type,
                )*
            }
        )*
    };
}

create_structs! {
    User {id: u32, name: String},
    Product { id: u32, price: f64 , qty: u32},
    Order { id: u32, user_id: u32, total: f64 }
}

pub fn main() {
    let user = User { id: 1, name: String::from("Alice") };
    let product = Product { id: 2, price: 19.99, qty:45 };
    let order = Order { id: 3, user_id: 1, total: 39.99 };
    println!("{:?}", user);
    println!("{:?}", product);
    println!("{:?}", order);
}