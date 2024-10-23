mod hannah { pub mod documentation; }
mod jerwin { pub mod refactor; }
mod leonhard { pub mod testing; }
mod marius {
    pub mod code_regeneration;
    pub mod code_gen_derive;
    pub mod code_gen_create_structs;
    pub mod code_gen_traits_and_structs;
}
mod sanchie {pub mod debugging;}
mod stephen { pub mod code_editing; }

fn main() {
    hannah::documentation::documentation();
    jerwin::refactor::refactor();
    leonhard::testing::testing();
    marius::code_regeneration::code_generation();
    // marius::code_gen_derive::main();
    // marius::code_gen_create_structs::main();
    // marius::code_gen_traits_and_structs::main();
    sanchie::debugging::debugging();
    stephen::code_editing::code_editing();
}


