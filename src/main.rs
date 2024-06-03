use std::fmt::{Display, Formatter};
use std::io;
use std::process::exit;
use crate::MetaCommandResult::{META_COMMAND_SUCCESS, META_COMMAND_UNRECOGNIZED_COMMAND};

enum MetaCommandResult {
    META_COMMAND_SUCCESS,
    META_COMMAND_UNRECOGNIZED_COMMAND
}

impl Display for MetaCommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Unrecognized command")
    }
}

enum PrepareResult {
    PREPARE_SUCCESS,
    PREPARE_UNRECOGNIZED_STATEMENT
}

fn do_meta_command(input_buffer:&str, prompt:&str) -> MetaCommandResult {
    if input_buffer == ".exit" {
        print!("{} Bye", prompt);
        exit(0);
    } else if input_buffer == "." {
        print!("{} Please give command..", prompt);
        return META_COMMAND_SUCCESS;
    }
    else
    {
    return META_COMMAND_UNRECOGNIZED_COMMAND;
    }
}


fn main() {
    let _CREATE_TABLE: &str = "create table";
    let _DROP_TABLE: &str = "drop table";
    let _ALTER_TABLE: &str = "alter table";
    let prompt: &str = "tinyDB:_>";


    print!("Welcome to tinyDB:\nThis is development version May 2024. Server Started:...\n");
    loop {
        let mut input_cmd: String = "".to_string();
        let n: io::Result<usize> = io::stdin().read_line(&mut input_cmd);
        if input_cmd.starts_with(".") {
            println!("{}", do_meta_command(input_cmd.trim(), prompt));
        } else {
            print!("{} {} DONE", prompt, input_cmd);
        }

    }
}
