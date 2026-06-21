// Import access to filesystem and IO error types
use std::fs;

fn main() {
    // Read assembly file as text
    let source = fs::read_to_string("test.asm").expect("Failed to read .asm file"); 
    // creates variable to store raw bytes
    let mut bytes: Vec<u8> = Vec::new();

    for line in source.lines() {
        let opcode = assemble(line);
        println!("{:#06X}", opcode);
        // Returns bytes in big-endian format, msb is stored at first memory address
        // Big-endian, highest byte stored in memory first
        // ex. 0x600F => [0x60, 0x0F]
        // extend_from_slice appends both bytes to array bytes
        bytes.extend_from_slice(&opcode.to_be_bytes());
        println!("{:02X?}", bytes);
    }

    fs::write("out.ch8", &bytes).expect("Failed to write ROM");

}

// takes the hex digit in VX in instruction and converts to hex value to index register array
fn parse_register(token: &str) -> u8 {
    let hex_digit = &token[1..];
    u8::from_str_radix(hex_digit, 16).expect("Invalid register")
}

fn parse_value(token: &str) -> u8 {
    // strip_prefix returns an enum called Option<T>, which could either be Some(T) or None
    // Some(T) -> there is a value, wrapped in side
    // None -> no value determined from stripping the prefix
    // Use a match to unbundle a returned Option
    match token.strip_prefix("0x") {
        Some(rest) => u8::from_str_radix(rest, 16).expect("Invalid hex value"),
        None => u8::from_str_radix(token, 10).expect("Invalid decimal value"),
    }
}

fn assemble(line: &str) -> u16 {
    // returns a new string with all commas replaced with white space
    // line is &str (pointer), meaning it is borrowed and we can't edit it
    // line.replace() returns a String that we can edit
    let cleaned = line.replace("," , " ");
    // collect() products a Vec<&str>, vector of string pointers
    let tokens: Vec<&str> = cleaned.split_whitespace().collect();
    // print with :? for the debug format
    println!("{:?}", tokens);
    match tokens[0] {
        "LD" => {
            // using the opcode structure, set opcode hex value into respective variable
            let x = parse_register(tokens[1]) as u16;
            let nn = parse_value(tokens[2]) as u16;
            // performs bitwise or operation to combine into single opcode
            /*0x6000   0110 0000 0000 0000   (family)
              x<<8     0000 0000 0000 0000   (x = 0)
              nn       0000 0000 0000 1111   (nn = 0x0F)
              --------------------------------- OR
              result   0110 0000 0000 1111   = 0x600F  ✓ */
            0x6000 | (x << 8) | nn
        },
        "ADD" => {
            let x = parse_register(tokens[1]) as u16;
            if tokens[2].starts_with("V") {
                let y = parse_register(tokens[2]) as u16;
                0x8000 | (x << 8) | (y << 4) | 0x0004
            } else {
                let nn = parse_value(tokens[2]) as u16;
                0x7000 | (x << 8) | nn
            }
        },
        _ => {
            eprintln!("Unknown mnemonic: {}", tokens[0]);
            0x0000
        },
    }
}

