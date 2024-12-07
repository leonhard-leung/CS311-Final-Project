macro_rules! say_hello
    {
        () =>{
            println!("Hello World")
        }
    }

pub fn main()
    {
        // Call the macro
        say_hello!();
    }