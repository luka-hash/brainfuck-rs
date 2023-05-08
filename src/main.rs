use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read, Write},
};

fn read_file(file_name: &str) -> Result<Vec<char>, io::Error> {
    let content = fs::read_to_string(file_name)?;
    return Ok(content.chars().collect());
}

fn create_jump_table(instructions: &[char]) -> Result<HashMap<usize, usize>, String> {
    let mut jump_table: HashMap<usize, usize> = HashMap::new();
    let mut stack: Vec<usize> = Vec::new();
    for (i, instruction) in instructions.iter().enumerate() {
        match instruction {
            '[' => {
                stack.push(i);
            }
            ']' => {
                if let Some(j) = stack.pop() {
                    jump_table.insert(i, j);
                    jump_table.insert(j, i);
                } else {
                    return Err("File is not valid".to_owned());
                }
            }
            _ => {}
        }
    }
    if !stack.is_empty() {
        return Err("File is not valid!".to_owned());
    }
    return Ok(jump_table);
}

fn execute(instructions: &[char], jump_table: &HashMap<usize, usize>) -> Result<(), String> {
    let mut tape: [u8; 30_000] = [0; 30_000];
    let mut pointer: usize = 0;
    let mut i = 0;
    while i < instructions.len() {
        match instructions[i] {
            '>' => pointer = (pointer + 1) % 30_000,
            '<' => {
                pointer = if pointer == 0 {
                    tape.len() - 1
                } else {
                    pointer - 1
                }
            }
            '+' => tape[pointer] = tape[pointer].wrapping_add(1),
            '-' => tape[pointer] = tape[pointer].wrapping_sub(1),
            '.' => {
                print!("{}", tape[pointer] as char);
                match io::stdout().flush() {
                    Ok(_) => {}
                    Err(error) => {
                        return Err(format!("Failed printing character to stdout: {}", error));
                    }
                }
            }
            ',' => {
                let mut input: [u8; 1] = [0; 1];
                match io::stdin().read_exact(&mut input) {
                    Ok(_) => tape[pointer] = input[0],
                    Err(error) => {
                        return Err(format!("Failed reading character from stdin: {}", error));
                    }
                }
            }
            '[' => {
                if tape[pointer] == 0 {
                    i = jump_table.get(&i).unwrap() + 1;
                    continue;
                }
            }
            ']' => {
                if tape[pointer] != 0 {
                    i = jump_table.get(&i).unwrap() + 1;
                    continue;
                }
            }
            _ => (),
        }
        i += 1;
    }
    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args {
        // Execute brainfuck for each given file(name).
        println!("=== BEGIN ===\tRunning: {}", &arg);
        let instructions = match read_file(&arg) {
            Ok(instructions) => instructions,
            Err(error) => {
                eprintln!("Problem reading the file: {:?}", error);
                println!("=== END ===");
                continue;
            }
        };
        let jump_table = match create_jump_table(&instructions) {
            Ok(jump_table) => jump_table,
            Err(error) => {
                eprintln!("Problem with the file: {:?}", error);
                println!("=== END ===");
                continue;
            }
        };
        let _ = match execute(&instructions, &jump_table) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Problem executing the file: {:?}", error);
                println!("=== END ===");
                continue;
            }
        };
        println!("\n=== END ===")
    }
}
