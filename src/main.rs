use std::fmt::{Display, Formatter};
use std::io;
use std::process::exit;
use crate::MetaCommandResult::{META_COMMAND_SUCCESS, META_COMMAND_UNRECOGNIZED_COMMAND};
use crate::PrepareResult::{PREPARE_SUCCESS, PREPARE_UNRECOGNIZED_STATEMENT};
use crate::StatementType::{STATEMENT_INSERT, STATEMENT_SELECT};

static PROMPT: &str = "tinyDB:_>";
enum MetaCommandResult {
    META_COMMAND_SUCCESS,
    META_COMMAND_UNRECOGNIZED_COMMAND
}

impl Display for MetaCommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            META_COMMAND_SUCCESS => write!(f, "{} {}", PROMPT, "OK"),
            META_COMMAND_UNRECOGNIZED_COMMAND => write!(f, "{} {}", PROMPT, "Unrecognized Command")
        }
    }
}

enum PrepareResult {
    PREPARE_SUCCESS,
    PREPARE_UNRECOGNIZED_STATEMENT
}

impl Display for PrepareResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PREPARE_SUCCESS => write!(f, "{} {}", PROMPT, "OK"),
            PREPARE_UNRECOGNIZED_STATEMENT => write!(f, "{} {}", PROMPT, "Unrecognized Command")
        }
    }
}

enum  StatementType { STATEMENT_INSERT, STATEMENT_SELECT }

fn prepare_statement(input_buffer:&str) -> PrepareResult {
    return match input_buffer {
        "insert" => {
            let statement = StatementType::STATEMENT_INSERT;
            PREPARE_SUCCESS
        },
        "select" => {
            let statement = StatementType::STATEMENT_SELECT;
            PREPARE_SUCCESS
        },
        _ => {
            PREPARE_UNRECOGNIZED_STATEMENT
        }
    }
}

fn do_meta_command(input_buffer:&str) -> MetaCommandResult {
    if input_buffer == ".exit" {
        print!("{} Bye", PROMPT);
        exit(0);
    } else if input_buffer == "." {
        return META_COMMAND_SUCCESS;
    }
    else {
        return META_COMMAND_UNRECOGNIZED_COMMAND;
    }
}


fn main() {
    print!("Welcome to tinyDB:\nThis is development version May 2024. Server Started:...\n");
    loop {
        let mut input_cmd: String = "".to_string();
        let n: io::Result<usize> = io::stdin().read_line(&mut input_cmd);
        if input_cmd.starts_with(".") {
            println!("{}", do_meta_command(input_cmd.trim()));
        } else {
            println!("{}", prepare_statement(input_cmd.trim()));
        }

    }
}
