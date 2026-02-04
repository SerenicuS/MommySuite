# MommyLang

## 📖 Syntax Reference

| Feature | MommyLang Syntax | C Equivalent | Description |
| :--- | :--- | :--- | :--- |
| **Declare Integer** | `mayihave [VAL] in [NAME] is int` | `int name = val;` | Declares an integer variable. |
| **Declare Pointer** | `mayihave null in [NAME] is box` | `int *name = NULL;` | Declares a pointer (Box). |
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

## 🛠 Usage
Write your code in `sandbox/test.mommy`.
