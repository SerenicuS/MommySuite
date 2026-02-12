// --- File System ---
pub const EXTENSION_SOURCE: &str = ".mommy";
pub const EXTENSION_C: &str = ".c";
pub const EXTENSION_EXE: &str = ".exe";
pub const IDE_OUTPUT_DIRECTORY: &str = "sandbox";
pub const SYMBOL_SINGLE_SLASH: char = '/';
pub const SYMBOL_DOUBLE_SLASH_REVERSE: char = '\\';
pub const SYMBOL_SELECTED_DEFAULT_FILE_PATH: &str = "./";


// --- Compiler Rules ---
pub const MIN_ARGS_MATH: usize = 4;
pub const MIN_ARGS_FILE_CMD: usize = 2;
pub const MIN_ARGS_LEN: usize = 2 ;
pub const MIN_INDEX_NAME_LEN: usize = 3;
pub const GCC_OUTPUT_FLAG: &str = "-o";


// --- Tools ---
pub const COMPILER_TOOL: &str = "gcc";
pub const RUN_PYTHON: &str = "python";
pub const RUN_NOTEPAD: &str = "notepad.exe";


// --- declaration.rs CONSTANTS ---

//  Syntax: mayihave <variable_name> in <NAME> as int
// Syntax: group <variable_size> in <name> as int
pub const MIN_CREATE_VAR_ARGS: usize = 6;
pub const INDEX_KEYWORD_CREATE_VARIABLE: usize = 0; //mayihave
pub const INDEX_KEYWORD_CREATE_INSERT_VARIABLE_VALUE: usize = 1; // value
pub const INDEX_KEYWORD_CREATE_CONNECTOR_VALUE_TO_VARIABLE_NAME: usize = 2; // in
pub const INDEX_KEYWORD_CREATE_DEFINE_VARIABLE_NAME: usize = 3; // variable_name
pub const INDEX_KEYWORD_CREATE_CONNECTOR_VARIABLE_NAME_TO_VARIABLE_TYPE: usize = 4; // as
pub const INDEX_KEYWORD_CREATE_VARIABLE_TYPE: usize = 5; // int



// syntax:replace <variable_name> with <value>
pub const MIN_REPLACE_VAR_ARGS: usize = 4;
pub const INDEX_KEYWORD_REPLACE: usize = 0; // replace
pub const INDEX_KEYWORD_REPLACE_VARIABLE_TARGET: usize = 1; // variable_name
pub const INDEX_KEYWORD_REPLACE_CONNECTOR_VARIABLE_NAME_TARGET_TO_VALUE: usize = 2; // with
pub const INDEX_KEYWORD_REPLACE_VALUE: usize = 3; // value


// --- mommy_shell CONSTANTS ---
pub const INDEX_DEFAULT_STARTING_COMMAND_ARGS: usize = 0;
pub const SHELL_LINE_INCREMENTOR: usize = 1;
pub const SHELL_LINE_INDICATOR: &str = ">";
pub const SHELL_CURRENT_DIRECTORY_KEYWORD: &str = ".";
pub const SHELL_PREVIOUS_DIRECTORY_KEYWORD: &str = "..";

pub const WINDOWS_EXTENDED_LENGTH_PATH_PREFIX: &str = "\\\\?\\";
pub const SHELL_EMPTY_STRING: &str = "";

pub const SHELL_IDE_SAVE_FILE_KEYWORD: &str = "SAVE";
pub const SHELL_IDE_EXIT_KEYWORD: &str = "EXIT";
pub const SHELL_IDE_CLEAR_KEYWORD: &str = "CLEAR";

// --- conditions.rs CONSTANTS ---

pub const MIN_ARGS_CONDITIONS_LEN: usize = 3;
pub const CONDITIONS_IF_KEYWORD: &str = "if";
pub const CONDITIONS_OR_KEYWORD: &str = "}else {";


// --- mommy_lang.rs CONSTANTS ---
pub const SYMBOL_OPERAND_ADDITION: &str = "+";
pub const SYMBOL_OPERAND_SUBTRACTION: &str = "-";
pub const SYMBOL_OPERAND_DIVISION: &str = "/";
pub const SYMBOL_OPERAND_MULTIPLICATION: &str = "*";
pub const SYMBOL_OPERAND_MODULO: &str = "%";

pub const KEYWORD_START_C_FILE: &str = "int main(){";
pub const SYMBOL_END_C_FILE: &str = "}";
pub const KEYWORD_EXIT_C_FILE: &str = "return 0;";

pub const TRANSPILE_ERROR_SPECIFIC_LINE: &str = "Line";
pub const INDEX_FILE_NAME: usize = 1;


// --- alu.rs CONSTANTS ---

pub const INDEX_VARIABLE_TARGET: usize = 1;
pub const INDEX_VARIABLE_SOURCE: usize = 3;

pub const VARIABLE_TYPE_STRING: &str = "String";
pub const VARIABLE_TYPE_C_STRING: &str = "char*";
pub const OPERATOR_DIVISION: &str = "/";
pub const VALUE_ZERO: &str = "0";

