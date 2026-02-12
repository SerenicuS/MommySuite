# MOMMYLANG SYNTAX SPECIFICATION

## 1. Core Keywords (The "Vocabulary")
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

## 2. Grammar Patterns

### A. Variables (The "Box")
**Declaration:**
`mayihave <VALUE> in <NAME> as <TYPE>`
* **Logic:** "Put 10 inside the box named 'age'."
* **Example:** `mayihave 10 in age as int`
* **Supported Types:** `int`, `float`, `char`, `String` (char*), `box` (int*)

**Assignment:**
`replace <NAME> with <VALUE>`
* **Example:** `replace age with 20`

### B. Pointers (The "Finger")
**Get Address:**
`replace <PTR_NAME> with <VAR_NAME> address`
* **Logic:** "Make 'ptr' look at 'age's address."
* **C Output:** `ptr = &age;`

**Write to Address (Dereference):**
`replace <PTR_NAME> with <VALUE> inside`
* **Logic:** "Put value INSIDE the box the pointer is pointing at."
* **Safety:** Includes automatic `NULL` check.
* **C Output:** `if(ptr!=NULL) *ptr = value;`

### C. Arrays (The "Memory")
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

### D. Math (The "Pain")
**Syntax:**
`<OPERATION> <TARGET> with <VALUE>`
* **Operations:** `add`, `subtract`, `multiply`, `divide`, `mod`
* **Example:** `add age with 1`
* **Constraint:** Cannot perform math on `String` types.

### E. Control Flow (The "Discipline")
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