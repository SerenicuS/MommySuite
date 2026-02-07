// --- File System ---
pub const EXTENSION_SOURCE: &str = ".mommy";
pub const EXTENSION_C: &str = ".c";
pub const EXTENSION_EXE: &str = ".exe";
pub const IDE_OUTPUT_DIRECTORY: &str = "sandbox";

// --- Compiler Rules ---
pub const MIN_ARGS_MATH: usize = 4;
pub const MIN_ARGS_FILE_CMD: usize = 2;
pub const MIN_ARGS_LEN: usize = 2 ;
pub const MIN_INDEX_NAME_LEN: usize = 3;


// --- Tools ---
pub const COMPILER_TOOL: &str = "gcc";
pub const RUN_PYTHON: &str = "python";
pub const RUN_NOTEPAD: &str = "notepad.exe";


// --- Declaration Const ---

//  Syntax: mayihave <variable_name> in <NAME> as int
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

