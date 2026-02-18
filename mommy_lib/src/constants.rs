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
pub const KW_MALLOC: &str     = "ibegyou";     // For dynamic memory allocation
pub const KW_FREE: &str       = "takeitback";     // For freeing dynamic memory

// Connectors
pub const KW_IN: &str         = "in";
pub const KW_AS: &str         = "as";
pub const KW_WITH: &str       = "with";
pub const KW_UPTO: &str       = "upto";

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

pub const KW_ALL: &str = "?";

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
pub const VALERIA_ANGRY_METER_LIMIT: usize = 3;

pub const SEPARATOR: &str = "────────────────────────────────────────────────────────────────";
pub const SEPARATOR_THICK: &str = "════════════════════════════════════════════════════════════════";
pub const SEPARATOR_DOUBLE: &str = "\n────────────────────────────────────────────────────────────────\n";

// Spacing (NEW)
pub const SPACE_SECTION: &str = "\n";      // Between major sections
pub const SPACE_SUBSECTION: &str = "\n";     // Between related items
pub const SPACE_PROMPT: &str = "> ";       // Before user input
pub const INDENT_OUTPUT: &str = "  │ ";      // For multi-line output

pub const SHELL_CMD_SAVE: &str = "SAVE";
pub const SHELL_CMD_EXIT: &str = "EXIT";
pub const SHELL_CMD_CLEAR: &str = "CLEAR";

pub const IDX_STARTING_COMMAND: usize = 0;




pub const SHELL_BASIC_COMMANDS: &str = r#"
    You are too greedy.
    ---------------
     1. tellme                      ->    List Commands
     2. mayileave                   ->    Exit the Terminal
     3. iamhere                     ->    Locate current Directory
     4. mommy?                      ->    List Files in current Directory
     5. walkwithme <filename>       ->    Move to another Directory
     6. goback                      ->    Return to Previous Directory
     7. canihave <filename>         ->    Create File
     8. takethe <filename>          ->    Delete File
     9. openthis <filename>         ->    Open the File
    10. readthis <filename>         ->    Read the File's contents
    11. doxxme                      ->    Windows Ip Configuration
    12. callmeplease <ip/dns>       ->    Ping device
    13. runthis <filename>          ->    Run File
    14. clear                       ->    Clear Terminal
    15. letusplayhouse <dir_name>   ->    Create Directory
    16. removethehouse <dir_name>   ->    Delete Directory
    ---------------
    "#;


pub const SHELL_ADVANCE_COMMANDS: &str = r#"
    You are too greedy.
    ---------------
     1. startcoding                 ->    Enter lite_IDE
     2. changeoutput <dir_name>     ->    Change Code Dir(sandbox default)
    ---------------
    "#;

pub const LETTER_SHELL_YES: &str = "Y";
pub const LETTER_SHELL_NO: &str = "N";
pub const LETTER_SHELL_T: &str = "T";

pub const SHELL_DBG_FILE: &str = "discipline-update-test.mommy";

pub const SHELL_DF_USER: &str = "Harold"; // DF means default
pub const SHELL_DF_PASS: &str = "Valeria";
pub const SHELL_DF_ANGERYNESS: usize = 2; 
