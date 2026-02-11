# MOMMYLANG SYNTAX SPECIFICATION (v0.1.8 - Pre-Discipline Update)

## 1. Core Keywords (The "Banned" List)
These words are reserved by the parser and cannot be used as variable names.

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