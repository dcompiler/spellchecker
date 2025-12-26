#[macro_use]
extern crate criterion;

mod conseq;
mod train;
mod repeats;

use std::env;
use std::fs;
use crate::train::train;
use crate::repeats::train_repeats;

fn main() {
    let inp = get_inp();

    let v = train_repeats(&inp);

    println!("{:#?}", v);
}  

fn get_inp() -> String {
    let mut inp = env::args().collect::<Vec<_>>()[1].clone();
    if inp == "-f" {
	let fnm = &env::args().collect::<Vec<_>>()[2];
	inp = fs::read_to_string(fnm)
            .expect(&format!("Cannot read the file {}", fnm));
    }
    inp
}

