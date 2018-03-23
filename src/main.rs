extern crate segfault_crate;

mod segfault_mod;

fn main() {
    let num_slots = 4096;

    println!("running segfault module ...");
    let wheel = vec![ None ; num_slots];
    segfault_mod::next_timeout(&wheel);
    println!("ok");

    println!("running seqfault crate ...");
    let wheel = vec![ None ; num_slots];
    segfault_crate::next_timeout(&wheel);
    println!("ok");
}


