# 🦀 MommySuite: Rust-Based Systems Ecosystem
> *A Custom Shell, Compiler, and Standard Library written in Rust.*

## 🤔 Why did I create this unhinged masterpiece?
As a student who wants to learn system concepts, I want to understand how each system feature *actually* works. Building a terminal seemed like a good way to learn, but I wanted to go deeper.

But honestly, creating a standard terminal is boring. So I built a **complete language ecosystem** (Shell + Transpiler + Library) for fun and to practice what I've learned in **Rust** and **Low-Level Memory Management**.

I'm hoping to keep improving and get better as a **Systems Programmer** through projects like this.

---

## Architecture
* Lexer & Parser: Custom-built tokenizer that handles state management (e.g., handling spaces inside string literals vs. code blocks).
* Transpilation: Maps abstract syntax to optimized C code, leveraging GCC for binary generation.
* Process Management: Uses Rust's std::process to spawn child processes for compilation and system commands.
* Memory Safety: While MommyLang allows raw pointers (box), the compiler (Rust) ensures the transpiler itself is memory-safe.

## ⚠️ DISCLAIMER
**This project is for EDUCATIONAL PURPOSES ONLY.**
* 🎓 Created to practice system-level concepts (Processes, Memory, Pointers).
* 🚫 Not intended for actual production use.
* 🤪 The naming convention is a "creative constraint" used purely for satire and entertainment.
* 🔨 This project is still under active development.

> **💥 SAFETY WARNING**
> This shell has **real system access**. It can:
> 1. Delete files in the current directory (even System32, so **do not use admin rights** ❌).
> 2. Create new files and folders.
> 3. Run basic Windows/Linux process commands.

## 🐚 Environment (MommyShell)

### Basic Navigation
| MommyShell Command | Standard Equivalent | Function |
| :--- | :--- | :--- |
| **`tellme`** | `help` | List available commands. |
| **`mayileave`** | `exit` | Exit the terminal. |
| **`iamhere`** | `pwd` | Locate current directory. |
| **`mommy?`** | `ls` / `dir` | List files in current directory. |
| **`walkwithme <dir>`** | `cd <dir>` | Move to another directory. |
| **`goback`** | `cd ..` | Return to previous directory. |

### File & System Management
| MommyShell Command | Standard Equivalent | Function |
| :--- | :--- | :--- |
| **`canihave <file>`** | `touch` | Create a new file. |
| **`takethe <file>`** | `del` / `rm` | Delete a file. |
| **`letusplayhouse <dir>`** | `mkdir` | Create a directory. |
| **`removethehouse <dir>`** | `rmdir` | Delete a directory. |
| **`openthis <file>`** | `start` / `open` | Open a file in default app. |
| **`readthis <file>`** | `cat` / `type` | Read file contents to console. |
| **`doxxme`** | `ipconfig` | Show network configuration. |
| **`callmeplease <ip>`** | `ping` | Ping a device. |

### 🛠️ Developer Tools (Advanced)
| Command | Function | Description |
| :--- | :--- | :--- |
| **`runthis <file>`** | **Compile & Run** | Transpiles your `.mommy` file to C, compiles with GCC, and executes it. |
| **`startcoding`** | **Enter Lite_IDE** | Launches the internal code editor for writing MommyLang scripts. |

## 📖 Syntax Reference (MommyLang)

| Feature | MommyLang Syntax | C Equivalent | Description |
| :--- | :--- | :--- | :--- |
| **Declare Integer** | `mayihave [VAL] in [NAME] as int` | `int name = val;` | Declares an integer variable. |
| **Declare Pointer** | `mayihave null in [NAME] as box` | `int *name = NULL;` | Declares a pointer (Box). |
| **Assignment** | `replace [NAME] with [VAL]` | `name = val;` | Updates a variable's value. |
| **Reference (&)** | `replace [PTR] with [NAME] address` | `ptr = &name;` | Stores the address of a variable in a pointer. |
| **Dereference (*)** | `replace [PTR] with [VAL] inside` | `*ptr = val;` | Changes the value *inside* the address the pointer holds. |
| **Addition** | `add [NAME] with [VAL]` | `name = name + val;` | Adds a value to a variable. |
| **Subtraction** | `subtract [NAME] with [VAL]` | `name = name - val;` | Subtracts a value from a variable. |
| **Multiplication** | `multiply [NAME] with [VAL]` | `name = name * val;` | Multiplies a variable by a value. |
| **Division** | `divide [NAME] with [VAL]` | `name = name / val;` | Divides a variable by a value. |
| **Print Text** | `say "[MESSAGE]"` | `printf("%s\n", "msg");` | Prints a literal string. **Must use quotes.** |
| **Print Variable** | `say [NAME]` | `printf("...", name);` | Prints a variable. Auto-detects type (`%d`, `%f`, `%s`). |
| **If Condition** | `ask [CONDITION]` | `if (cond) {` | Starts a logic check (supports `>`, `<`, `==`, `!=`). |
| **Else** | `or` | `} else {` | The alternative path. |
| **Break** | `satisfied` | ` break; ` | Immediately exits the current loop scope. |
| **Loop** | `punishme [AMOUNT]` | `for(int i=0; i<amt; i++){` | Repeats the block `amount` times. |
| **End Block** | `done` | `}` | Closes an `ask`, `or`, or `punishme`. |
| **Exit** | `leave` | `return 0;` | Ends the program. |
