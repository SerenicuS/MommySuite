# MOMMYLANG SYNTAX SPECIFICATION

## 1. Core Keywords (The "Vocabulary")
These words are reserved by the parser and define the structure of the language.

| Keyword      | Function                  | Source File      |
|:-------------|:--------------------------|:-----------------|
| `mayihave`   | Variable Declaration      | `declaration.rs` |
| `group`      | Array Declaration         | `declaration.rs` |
| `replace`    | Assignment (Var & Array)  | `declaration.rs` |
| `in`         | Index / Container Marker  | `declaration.rs` |
| `as`         | Type Definition Marker    | `declaration.rs` |
| `with`       | Value Assignment Marker   | `declaration.rs` |
| `address`    | Pointer Reference (`&`)   | `declaration.rs` |
| `inside`     | Pointer Dereference (`*`) | `declaration.rs` |
| `punishme`   | Loop (Count / Infinite)   | `loops.rs`       |
| `punishmeif` | Loop Conditional          | `loops.rs`       |
| `satisfied`  | Break Loop                | `loops.rs`       |
| `done`       | End Block (`}`)           | `loops.rs`       |
| `ask`        | Condition Start (`if`)    | `conditions.rs`  |
| `or`         | Condition Else (`else`)   | `conditions.rs`  |
| `leave`      | End Program (`return 0`)  | `main.rs`        |
| `say`        | Print Output              | `io.rs`          |
| `ibegyou`    | Heap Allocation           | `declaration.rs` |
| `takeitback` | Free Heap Allocation      | `declaration.rs` |

---

## 2. Grammar Patterns

### A. Variables (The "Box")
**Declaration:**
`mayihave <VALUE> in <NAME> as <TYPE>`
* **Logic:** "Put 10 inside the box named 'age'."
* **Example:** `mayihave 10 in age as int`
* **Supported Types:** `int`, `float`, `char`, `String` (char*), `box` (int*), `ascii` (special character-array mode)

**Assignment:**
`replace <NAME> with <VALUE>`
* **Example:** `replace age with 20`

### B. Pointers (The "Finger")
**Get Address:**
`replace <PTR_NAME> with <VAR_NAME> address`
* **C Output:** `ptr = &var;`

**Write to Address (Dereference):**
`replace <PTR_NAME> with <VALUE> inside`
* **Safety:** Includes automatic `NULL` check.
* **C Output:** `if(ptr!=NULL) *ptr = value;`

### C. Arrays (The "Memory")
**Declaration:**
`group <SIZE> in <NAME> as <TYPE>`
* **Example:** `group 5 in hello as ascii`
* **Metadata:** Stored as `"array:<type>:<size>"`.

**Write to Slot:**
`replace <ARRAY> in <INDEX> with <VALUE>`
* **Example:** `replace hello in 0 with 72`

**Read from Slot:**
`replace <VAR> with <ARRAY> in <INDEX>`
* **Example:** `replace temp with hello in i`

### D. Heap Memory (The "Plea")
**Allocate:**
`ibegyou <SIZE> in <NAME> as <TYPE>`
* **Logic:** Allocates `<SIZE>` items of `<TYPE>` on the heap.
* **C Output:** `<TYPE>* <NAME> = (<TYPE>*)malloc(<SIZE> * sizeof(<TYPE>));`
* **Failure:** Emits a null check and prints an error before returning.

**Free:**
`takeitback <NAME>`
* **Logic:** Frees heap memory and nulls the pointer.
* **C Output:** `free(<NAME>); <NAME> = NULL;`

### E. Input/Output (The "Voice")
**Scalar Print:**
`say <NAME>`
* **Logic:** Detects type from symbol table and prints with `\n`.

**Array Peek (Specific Index):**
`say <ARRAY> in <INDEX>`
* **Example:** `say hello in 0` (Prints 'H')

**Array Dump (Wildcard Operator):**
`say <ARRAY> in ?`
* **Logic:** If type is `ascii`, generates a `for` loop to print the full string.
* **Example:** `say hello in ?` (Prints "HELLO")

### F. Math (The "Pain")
**Syntax:**
`<OPERATION> <TARGET> with <VALUE>`
* **Operations:** `add`, `subtract`, `multiply`, `divide`, `mod`
* **Example:** `add temp with 1`

### G. Control Flow (The "Discipline")
**Conditions:**
```text
ask if <CONDITION>
    ...
or
    ...
done

punishme <COUNT>
    ...
    ask if <CONDITION>
        satisfied  <-- Break
    done
done

punishmeif <CONDITION>
    say "hello"
done
```
