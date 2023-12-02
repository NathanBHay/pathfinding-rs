use std::mem::size_of;

mod search;
mod heuristics;
mod domains;
mod mapf;

fn main() {
    let x: u32 = 4;
    //print sizeof x
    let y: Vec<u32> = vec![0; 16];
    println!("Hello, world! {}", size_of::<usize>());
}
