#![allow(non_snake_case)] // because ill name my crate whatever i want

mod render;
use render::render;

use std::env;

fn help(argv: Vec<String>) {
    println!("PRVW v0.1.0");
    if argv.len() >= 3 && argv[1] == "help" {
        match argv[2].as_str() {
            _ => unimplemented!()
        }
    } else {
        if argv.len() <= 1 { println!("no command supplied"); } // ugly ikik
        if argv.len() >= 2 && argv[1] != "help" { println!("unknown command") }

        println!("\nrender or r: renders a preview of a spwn file/function");
        println!("--- flags ---");
        println!("-F or --format: specifies the format to use, current formats: none");
        println!("-f or --function: spefices which function to render, defaults to whole_file");
        println!("-t or --triggers: specified if PRWV should render triggers, is either `true` or `false");
        println!("-O or --objects: specified if PRWV should render objects, is either `true` or `false");
        println!("-o or --output: specified where PRWV should save the file, defaults to the PRVW.png");
        println!("-v or --verbose: output some debug info");
        println!("-V or --very-verbose: extrovert");
        println!("");
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    match argv[1].as_str() {
        "render" | "r" => render(argv),
        "help" | "h" => help(argv),
        _ => unimplemented!()
    }
}