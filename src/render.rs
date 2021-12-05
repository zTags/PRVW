use std::fmt::{Display, Formatter, Result};

struct Config {
    format: String,
    output: String,
    function: String,
    show_objects: bool,
    show_triggers: bool,
    verbose: u8,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "format: {}, output: {}, show_objects{}, show_triggers: {}, function: {}, verbose: {}", self.format, self.output, self.show_objects, self.show_triggers, self.function, self.verbose)
    }
}

pub fn render(argv: Vec<String>) {
    let mut conf = Config {
        format: "png".to_string(),
        output: "PRVW.png".to_string(),
        show_objects: false,
        show_triggers: true,
        function: "whole_file".to_string(),
        verbose: 0
    };
    // parse argv
    for i in 0..argv.len() {
        if argv[i] == "-v" || argv[i] == "--verbose" {
            conf.verbose = 1;
        } else if argv[i] == "-V" || argv[i] == "--very-verbose" {
            conf.verbose = 2;
        } else if argv[i] == "-F" || argv[i] == "--format"{
            conf.format = argv[i+1].to_string();
        } else if argv[i] == "f" || argv[i] == "--function" {
            conf.function = argv[i+1].to_string();
        }  else if argv[i] == "-o" || argv[i] == "--output" {
            conf.output = argv[i+1].to_string();
        } else if argv[i] == "-t" || argv[i] == "--triggers" {
            if argv[i+1] == "false" {
                conf.show_triggers = false;
            }
        } else if argv[i] == "-O" || argv[i] == "--objects" {
            if argv[i+1] == "false" {
                conf.show_triggers = false;
            }
        }
    }

    println!("{}", conf);
}