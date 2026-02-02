# MommyLang

## 📖 Syntax Reference

| Feature | MommyLang Syntax                       | C Equivalent | Description |
| :--- |:---------------------------------------| :--- | :--- |
| **Declare Integer** | `mayihave [VAL] in [NAME] as int`      | `int name = val;` | Declares an integer variable. |
| **Declare Float** | `mayihave [VAL] in [NAME] as float`    | `float name = val;` | Declares a floating-point variable. |
| **Declare String** | `mayihave "[VAL]" in [NAME] as String` | `char name[] = "val";` | Declares a string variable. |
| **Addition** | `add [NAME] with [VAL]`                | `name = name + val;` | Adds a value to a variable. |
| **Subtraction** | `subtract [NAME] with [VAL]`           | `name = name - val;` | Subtracts a value from a variable. |
| **Multiplication** | `multiply [NAME] with [VAL]`           | `name = name * val;` | Multiplies a variable by a value. |
| **Division** | `divide [NAME] with [VAL]`             | `name = name / val;` | Divides a variable by a value. |
| **Print Text** | `say "[MESSAGE]"`                      | `printf("%s\n", "msg");` | Prints a literal string. **Must use quotes.** |
| **Print Variable** | `say [NAME]`                           | `printf("...", name);` | Prints a variable. Auto-detects type (`%d`, `%f`, `%s`). |
| **If Condition** | `ask if [CONDITION]`                   | `if (cond) {` | Starts a logic check (supports `>`, `<`, `==`, `!=`). |
| **Else** | `or`                                   | `} else {` | The alternative path. |
| **Loop** | `punishme [AMOUNT]`                    | `for(int i=0; i<amt; i++){` | Repeats the block `amount` times. |
| **End Block** | `done`                                 | `}` | Closes an `ask`, `or`, or `punishme`. |
| **Exit** | `leave`                                | `return 0;` | Ends the program. |

## 🛠 Usage
Write your code in `sandbox/test.mommy`.
