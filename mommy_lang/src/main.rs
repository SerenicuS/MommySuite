//! # MommyLang Compiler Infrastructure
//!
//! This is the entry point for the MommyLang compiler. It handles file parsing,
//! C transpilation, and GCC invocation.
//!
//! ---
//!  Started: Jan 26, 2026
//!  Amount of Time Spent(Too late to track): 28(H), 57(M).
//!  Current: March 3, 2026
//! ---
//!
//!

mod config;
mod compiler;
mod pipeline;

use std::env;

use mommy_lib::responses;
use mommy_lib::shell_format::{print_line, eprint_line};

use crate::compiler::{show_c_conversion_error, transpile_code_to_c};
use crate::config::Config;
use crate::pipeline::{compile_to_gcc, run_mommy_file};


fn main() {
    let args: Vec<String> = env::args().collect();


    let config = match Config::new(&args){ // Prepare the file
        Ok(cfg) => cfg,
        Err(e) => {
            eprint_line(e);
            std::process::exit(1);
        }
    };
    
    if let Err(e) = transpile_code_to_c(&config){ //Convert mommy_lang to C
        print_line(responses::MommyLangError::ErrorBegins);
        eprint_line(e);
        show_c_conversion_error(&config); // show fragmented c code
        eprint_line(responses::MommyLangError::ConvertLangFailed);
        print_line(responses::MommyLangError::ErrorEnds);
        std::process::exit(1);
    }

    if let Err(e) = compile_to_gcc(&config){ //use GCC to create exe file for the converted C
        print_line(responses::MommyLangError::ErrorBegins);
        eprint_line(responses::MommyLangError::GCCError);
        eprint_line(e);
        print_line(responses::MommyLangError::ErrorEnds);
        std::process::exit(1);
    }

    print_line(responses::MommyLangStatus::CodeOutputBegins);
    if let Err(e) = run_mommy_file(&config){ // Run the exe file
        print_line(responses::MommyLangError::ErrorBegins);
        eprint_line(responses::MommyLangError::RuntimeError);
        eprint_line(e);
        print_line(responses::MommyLangError::ErrorEnds);
        std::process::exit(1);
    }
    print_line(responses::MommyLangStatus::CodeOutputEnds);
}

