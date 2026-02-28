//! # MommyLang Compiler Infrastructure
//!
//! This is the entry point for the MommyLang compiler. It handles file parsing,
//! C transpilation, and GCC invocation.
//!
//! ---
//!
//!  Started: Jan 26, 2026
//!  Amount of Time Spent(Too late to track): 20(H), 28(M).
//!  Current: February 26, 2026
//!
//! ## 🛠️ Development Roadmap
//!
//! ### Phase 2: The Discipline Update (Postponed)
//! Focused on memory safety, data structures, and stricter control.
//! - [x] Modular architecture refactor
//! - [x] Unified shell formatting system (`shell_format.rs`)
//! - [x] Data persistence (`mommy_conf.memory`)
//! - [x] Constants module (100+ named constants)
//! - [x] Pointer support (`address`, `inside`)
//! - [x] Heap allocation (`ibegyou`)
//! - [x] Standard input (`listen`)
//! - [x] Package system (`makeme`)
//! - [ ] Bitwise operations
//! - [ ] Functions
//! - [ ] System calls
//! - [ ] Enhanced error messages
//! - [ ] Security & sandboxing
//!
//! ### Phase 3: The Stockholm Update (current)
//! Focused on OS-level features and system dependency.
//! - [ ] System Pseudo Startup (custom init process)
//! - [ ] Custom IDE editor (syntax highlighting, real-time editing) < only basic
//! - [ ] Multi-file project support
//! - [ ] Advanced debugging features
//! - [ ] Performance optimizations
//! - [x] Terminal UI improvements
//! - [ ] Cleanup (refactoring & optimization)
//!
//! ### Phase 4: OS Features
//! - [ ] MommyOS kernel concepts
//! - [ ] Process management
//! - [ ] Memory allocation tracking
//! - [ ] Custom standard library expansion
//!
//! ### Bonus Objectives
//! - [ ] **Mommy's Fingers:** Registry-like assembly manipulation.
//!
//! ---
//!
//! ## 🧠 Psychological Phases (The Lore)
//!
//! The compiler's personality evolves with the user's proficiency.
//!
//! 1.  **Phase 1 (Abusive):** Rejection. *"You are stupid."* (Syntax Errors = Insults)
//! 2.  **Phase 2 (Discipline):** Correction. *"Do it my way."* (Strict Typing/Borrow Checking)
//! 3.  **Phase 3 (Stockholm):** Acceptance. *"This is my home."* (Vendor Lock-in)
//!
//! ### Future Expansions
//! * **Phase 3.5 (Gaslighting):** Confusion. *"Did I do that?"* (Randomized warnings)
//! * **Phase 4 (Domestic):** Responsibility. *"I must feed the system."* (Manual memory management)
//! * **Phase 5 (Freedom):** False Hope. *"I can leave... but do I want to?"* (The final test)


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
    
    if let Err(e) = transpile_code_to_c(&config){ //Convert mommylang to C
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

