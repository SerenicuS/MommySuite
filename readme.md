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
* C and Rust Combination: Using Rust to translate my eso lang into C.

![Hello World!](readme_assets/hello-world.gif)

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

## MOMMYLANG SYNTAX SPECIFICATION

### 1. Core Keywords (The "Vocabulary")
These words are reserved by the parser and define the structure of the language.

| Keyword     | Function                  | Source File      |
|:------------|:--------------------------|:-----------------|
| `mayihave`  | Variable Declaration      | `declaration.rs` |
| `group`     | Array Declaration         | `declaration.rs` |
| `replace`   | Assignment (Var & Array)  | `declaration.rs` |
| `in`        | Index / Container Marker  | `declaration.rs` |
| `as`        | Type Definition Marker    | `declaration.rs` |
| `with`      | Value Assignment Marker   | `declaration.rs` |
| `address`   | Pointer Reference (`&`)   | `declaration.rs` |
| `inside`    | Pointer Dereference (`*`) | `declaration.rs` |
| `punishme`  | Loop (Count / Infinite)   | `loops.rs`       |
| `satisfied` | Break Loop                | `loops.rs`       |
| `done`      | End Block (`}`)           | `loops.rs`       |
| `ask`       | Condition Start (`if`)    | `conditions.rs`  |
| `or`        | Condition Else (`else`)   | `conditions.rs`  |
| `leave`     | End Program (`return 0`)  | `main.rs`        |
| `say`       | Print Output              | `io.rs`          |

---

### 2. Grammar Patterns

#### A. Variables (The "Box")
**Declaration:**
`mayihave <VALUE> in <NAME> as <TYPE>`
* **Logic:** "Put 10 inside the box named 'age'."
* **Example:** `mayihave 10 in age as int`
* **Supported Types:** `int`, `float`, `char`, `String` (char*), `box` (int*)

**Assignment:**
`replace <NAME> with <VALUE>`
* **Example:** `replace age with 20`

#### B. Pointers (The "Finger")
**Get Address:**
`replace <PTR_NAME> with <VAR_NAME> address`
* **Logic:** "Make 'ptr' look at 'age's address."
* **C Output:** `ptr = &age;`

**Write to Address (Dereference):**
`replace <PTR_NAME> with <VALUE> inside`
* **Logic:** "Put value INSIDE the box the pointer is pointing at."
* **Safety:** Includes automatic `NULL` check.
* **C Output:** `if(ptr!=NULL) *ptr = value;`

#### C. Arrays (The "Memory")
**Declaration:**
`group <SIZE> in <NAME> as <TYPE>`
* **Logic:** "Reserve 5 slots in memory called 'scores'."
* **Example:** `group 5 in scores as int`
* **Metadata:** Stored in symbol table as `"array:<type>:<size>"`.

**Write to Slot:**
`replace <ARRAY> in <INDEX> with <VALUE>`
* **Logic:** "Go to slot 0 of 'scores' and write 100."
* **Example:** `replace scores in 0 with 100`

**Read from Slot:**
`replace <VAR> with <ARRAY> in <INDEX>`
* **Logic:** "Read slot 0 of 'scores' and put it into 'temp'."
* **Example:** `replace temp with scores in 0`

#### D. Math (The "Pain")
**Syntax:**
`<OPERATION> <TARGET> with <VALUE>`
* **Operations:** `add`, `subtract`, `multiply`, `divide`, `mod`
* **Example:** `add age with 1`
* **Constraint:** Cannot perform math on `String` types.

#### E. Control Flow (The "Discipline")
**Conditions:**
```text
ask if <CONDITION>
    ...
or
    ...
done

Loops:
Plaintext

punishme <COUNT>
    ...
    ask if <CONDITION>
        satisfied  <-- Break
    done
done
