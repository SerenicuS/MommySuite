use hive_std::insert_token;
use std::fs;
use std::io::Write;
use std::process::Command;

fn parse_line(tokens: Vec<String>) -> String{
    if tokens.is_empty(){
        return String::new();
    }

    match tokens[0].as_str(){
        // Syntax: mayihave [VALUE] in [NAME] as [TYPE]
        // Tokens: [0]        [1]   [2]  [3] [4]  [5]

        "mayihave" =>{
            let value = &tokens[1];
            let name = &tokens[3];
            let variable_type = &tokens[5];

            // Return int x = 5
            format!("{} {} = {};", variable_type, name, value)
        }

        // Syntax say [Message]
        // Tokens [0]   [1]

        "say" => {

            let message = &tokens[1];
            if message.starts_with("\""){
                format!("printf(\"%s\\n\", {});", message) // string
            }

            else{
                format!("printf(\"%d\\n\", {});", message) // without quotation print messages
            }

        }

        // Syntax: punishme (variable)
        //   printf
        // done
        "punishme" =>{
            let repeat_count = &tokens[1];

            format!("for (int i = 0; i < {}; i++) {{", repeat_count)
        }

        // Syntax: add variable with 5
        "add" => {
            let value = &tokens[1];

            let add_to_value = &tokens[3];

            format!("{} = {} + {};", value, value, add_to_value)
        }

        //Syntax: ask if (variable) (operator) (variable)

        "ask" =>{
            let condition = &tokens[2..].join(" ");
            format!("if ({}) {{", condition)
        }

        // Syntax: else
        "or" => {
            format!("else {{")
        }

        // For brackets
        "done" =>{
            format!("}}")
        }

        "leave" => {
            "return 0;".to_string()
        }

        _ => {
            format!("//Unknown command {}", tokens[0])
        }
    }
}





fn main(){
    let content = fs::read_to_string("sandbox/test.mommy").expect("Could not read the file Does test Mommy exist?");

    let mut output_file = fs::File::create("sandbox/test.c").expect("Could not create file");

    writeln!(output_file, "#include <stdio.h>").unwrap();
    writeln!(output_file, "int main(){{").unwrap();

    for line in content.lines(){
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        let tokens = insert_token(trimmed_line);
        let c_code = parse_line(tokens);

        writeln!(output_file, "    {}", c_code).unwrap();


    }

    writeln!(output_file, "}}").unwrap();
    drop(output_file);
    println!("Transpilation successful. Now summoning GCC...");


    println!("Compiling...");

    let output = Command::new("gcc")
        .arg("sandbox/test.c")
        .arg("-o")
        .arg("sandbox/test.exe")
        .output()
        .expect("Failed to execute GCC");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    println!("GCC finished. Running the program...\n");
    println!("--- MOMMY OUTPUT BEGINS ---");

    let _ = Command::new("./sandbox/test.exe")
        .status()
        .expect("Failed to run the executable.");

    println!("\n--- MOMMY OUTPUT ENDS ---");
}