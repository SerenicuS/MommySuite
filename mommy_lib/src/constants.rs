// ================================================================
// 1. SYSTEM & CONFIGURATION
// ================================================================
pub const EXT_SOURCE: &str = ".mommy";
pub const EXT_C: &str      = ".c";
pub const EXT_EXE: &str    = ".exe";

pub const DIR_OUTPUT: &str   = "sandbox";
pub const PATH_DEFAULT: &str = "./";

// Compiler Tools
pub const CMD_GCC: &str        = "gcc";
pub const CMD_GCC_FLAG: &str   = "-o";
pub const CMD_RUN_PYTHON: &str = "python";
pub const CMD_RUN_NOTEPAD: &str = "notepad.exe";

// Limits
pub const ARGS_MIN_MATH: usize = 4;
pub const ARGS_MIN_FILE: usize = 2;
pub const ARGS_MIN_LEN: usize  = 2;
pub const NAME_MIN_LEN: usize  = 3; // Was MIN_INDEX_NAME_LEN
pub const ARGS_MIN_COND: usize = 3;
pub const ARGS_MIN_IO: usize = 2;
pub const ARGS_MIN_IO_ARRAY: usize = 4;
pub const SIZE_UNKNOWN: usize = 0;


// ================================================================
// 2. MOMMY LANGUAGE (Input Syntax)
// ================================================================
// Keywords the user types
pub const KW_VAR_DEC: &str    = "mayihave"; // Was INDEX_KEYWORD_CREATE_VARIABLE (sort of)
pub const KW_ARRAY_DEC: &str  = "group";
pub const KW_ASSIGN: &str     = "replace";  // Was INDEX_KEYWORD_REPLACE (sort of)

// Connectors
pub const KW_IN: &str         = "in";
pub const KW_AS: &str         = "as";
pub const KW_WITH: &str       = "with";

// Pointers & Types
pub const KW_BOX: &str        = "box";     // Pointer type
pub const KW_ADDR: &str       = "address"; // & operator
pub const KW_DEREF: &str      = "inside";  // * operator
pub const KW_NULL: &str       = "null";
pub const KW_ARRAY: &str      = "array";   // Internal/Type keyword
pub const KW_POINTER: &str    = "pointer";

// Loop & Logic Keywords
pub const KW_IF: &str         = "if";
pub const KW_ELSE_BLOCK: &str = "}else {";

// Types
pub const TYPE_STRING: &str     = "String";  // Used by io.rs
pub const TYPE_INT: &str        = "int";
pub const TYPE_FLOAT: &str      = "float";
pub const TYPE_ASCII: &str      = "ascii";

// Unique

pub const TYPE_ALL: &str = "?";

// ================================================================
// 3. C LANGUAGE (Output Generation)
// ================================================================
// Boilerplate
pub const C_MAIN_START: &str  = "int main(){";
pub const C_MAIN_END: &str    = "}";
pub const C_EXIT_SUCC: &str   = "return 0;";

// C Keywords & Types
pub const C_TYPE_CHAR_PTR: &str = "char*";
pub const C_NULL: &str        = "NULL";
pub const C_VAL_ZERO: &str    = "0";
pub const C_TYPE_INT_PTR: &str  = "int*";
pub const C_KW_RETURN: &str = "return";
pub const C_KW_VOID: &str   = "void";
pub const C_KW_WHILE: &str  = "while";
pub const C_KW_CHAR: &str   = "char";

// C Operators
pub const C_OP_ADD: &str      = "+";
pub const C_OP_SUB: &str      = "-";
pub const C_OP_DIV: &str      = "/";
pub const C_OP_MUL: &str      = "*";
pub const C_OP_MOD: &str      = "%";

// Error Messages / Splitters
pub const MSG_ERR_LINE: &str  = "Line";
pub const SYM_SPLITTER: char  = ':';
pub const SYM_SLASH: char     = '/';
pub const SYM_BACKSLASH: char = '\\';
pub const SYM_WHITESPACE: &str = " ";

// ================================================================
// 4. PARSING INDICES (The "Slots")
// ================================================================

// Declaration: mayihave <VAL> in <NAME> as <TYPE>
// Indices:     0        1     2  3      4  5
pub const ARGS_MIN_DECL: usize      = 6;
pub const IDX_DECL_KW: usize        = 0; // "mayihave"
pub const IDX_DECL_VALUE: usize     = 1;
pub const IDX_DECL_KEY_IN: usize    = 2; // "in"
pub const IDX_DECL_NAME: usize      = 3;
pub const IDX_DECL_KEY_AS: usize    = 4; // "as"
pub const IDX_DECL_TYPE: usize      = 5;

// Assignment: replace <NAME> with <VAL>
// Indices:    0       1      2    3
pub const ARGS_MIN_ASSIGN: usize    = 4;
pub const IDX_ASSIGN_KW: usize      = 0; // "replace"


pub const IDX_ASSIGN_NAME: usize    = 1;
pub const IDX_ASSIGN_KEY_WITH: usize= 2; // "with"
pub const IDX_ASSIGN_VALUE: usize   = 3;

// Array Assignment: replace <NAME> in <IDX> with <VAL>
// Indices:          0       1      2  3     4    5
pub const ARGS_MIN_ARR_ASSIGN: usize = 6;
pub const IDX_ARR_KEY_IN: usize     = 2; // "in"
pub const IDX_ARR_INDEX: usize      = 3;
pub const IDX_ARR_KEY_WITH: usize   = 4; // "with"
pub const IDX_ARR_VALUE: usize      = 5;

// Math: add <TARGET> with <SOURCE>
// Indices: 0   1        2    3
pub const IDX_MATH_TARGET: usize    = 1;
pub const IDX_MATH_SOURCE: usize    = 3;
pub const IDX_FILE_NAME: usize      = 1;

// IO: say <MSG/VAR>
// IO Array: say <VAR> in <IDX>
pub const IDX_IO_VALUE: usize   = 1; // The thing being said
pub const IDX_IO_KEY_IN: usize  = 2; // "in" keyword
pub const IDX_IO_ARR_IDX: usize = 3; // The array index

// Loops: punishme <COUNT>
pub const IDX_LOOP_COUNT: usize = 1;

// ================================================================
// 5. SHELL CONSTANTS
// ================================================================
pub const SHELL_IDX_CMD: usize = 0;
pub const SHELL_LINE_INC: usize = 1;
pub const SHELL_PROMPT: &str = ">";
pub const SHELL_DIR_CURR: &str = ".";
pub const SHELL_DIR_PREV: &str = "..";
pub const SHELL_PATH_PREFIX: &str = "\\\\?\\";
pub const SHELL_EMPTY: &str = "";

pub const SHELL_CMD_SAVE: &str = "SAVE";
pub const SHELL_CMD_EXIT: &str = "EXIT";
pub const SHELL_CMD_CLEAR: &str = "CLEAR";

pub const IDX_STARTING_COMMAND: usize = 0;