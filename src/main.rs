use crate::documentation::Documentation::Documentation;

mod documentation { pub mod Documentation; }
mod refactoring { pub mod Refactor; }
mod testing { pub mod Numbers; pub mod Palindrome; }
mod debugging {pub mod Debugging;}
mod advanced_code_editing{pub mod format; pub mod todo;}

mod code_generation {
    pub mod derive;
    pub mod create_structs;
    pub mod traits_and_structs;
    pub mod simple_macro;
}

fn main() {
    Documentation::add(10,10);
    refactoring::Refactor::refactor();
    debugging::Debugging::debugging();
    advanced_code_editing::format::cartesian_product("stephen", "bscs3");

    code_generation::derive::main();
    // code_generation::simple_macro::main();
}


