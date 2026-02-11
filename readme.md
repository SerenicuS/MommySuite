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
* C and Rust Combination: Using Rust to translate my eso lang into C

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

| Keyword      | Function                   | Source File      |
|:-------------|:---------------------------|:-----------------|
| `mayihave`   | Variable Declaration       | `declaration.rs` |
| `group`      | Array Declaration          | `declaration.rs` |
| `replace`    | Assignment                 | `declaration.rs` |
| `in`         | Container/Location Marker  | `declaration.rs` |
| `as`         | Type Definition Marker     | `declaration.rs` |
| `with`       | Value Assignment Marker    | `declaration.rs` |
| `address`    | Pointer Reference (`&`)    | `declaration.rs` |
| `inside`     | Pointer Dereference (`*`)  | `declaration.rs` |
| `punishme`   | Basic Loop                 | `loops.rs`       |
| `punishmeif` | Conditional Loop           | `loops.rs`       |
| `satisfied`  | Break Loop                 | `loops.rs`       |
| `done`       | End Block (`}`)            | `loops.rs`       |
| `ask`        | Condition Start (`if`)     | `conditions.rs`  |
| `or`         | Condition Else (`else`)    | `conditions.rs`  |
| `leave`      | End Program (`return 0`)   | `main.rs`        |


---

## 2. Grammar Patterns

### A. Variables (The "Box")
**Declaration:**
`mayihave <VALUE> in <NAME> as <TYPE>`
* *Logic:* "Put 10 inside the box named 'age'."
* *Example:* `mayihave 10 in age as int`
* *Special Types:*
  * `String` (Compiles to `char*`)
  * `box` (Compiles to `int*`)

**Assignment:**
`replace <NAME> with <VALUE>`
* *Example:* `replace age with 20`

### B. Pointers (The "Finger")
**Get Address:**
`replace <PTR_NAME> with <VAR_NAME> address`
* *Logic:* "Make pointer look at variable's address."
* *C Output:* `ptr = &var;`

**Write to Address (Dereference):**
`replace <PTR_NAME> with <VALUE> inside`
* *Logic:* "Put value INSIDE what the pointer is pointing at."
* *Safety:* Includes automatic `NULL` check before writing.
* *C Output:* `if(ptr==NULL){...} *ptr = value;`

### C. Arrays (The "Group") - [PHASE 2]
**Declaration:**
`group <SIZE> in <NAME> as <TYPE>`
* *Logic:* "Create a group of size 5 inside 'scores'."
* *Example:* `group 5 in scores as int`
* *Metadata:* Stored in symbol table as `"array:<type>:<size>"`.

**Write to Slot:**
`replace <NAME> with <VALUE> in <INDEX>`
* *Logic:* "Replace scores with 100 in slot 0."
* *Example:* `replace scores with 100 in 0`

**Read from Slot:**
`say <NAME> in <INDEX>`
* *Example:* `say scores in 0`

### D. Math (The "Pain")
**Syntax:**
`<OPERATION> <TARGET> with <VALUE>`
* *Operations:* `add`, `subtract`, `multiply`, `divide`, `mod`
* *Example:* `add age with 1`
* *Constraint:* Cannot perform math on `String` types.

### E. Control Flow (The "Discipline")
**Loops:**
```text
punishme <COUNT>
    ...
    satisfied  <-- (Optional break)
done

punishmeif <CONDITION>
    ...
    satisfied  <-- (Optional break)
done
