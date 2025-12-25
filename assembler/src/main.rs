// use std::error::Error;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(r#"Arguments: {:?}"#, args);

    if let Some(arg) = args.get(1) {
        println!("Processing file: {}", arg);
        match read_to_string(&args[1]) {
            Ok(content) => process_file(content),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    } else {
        eprintln!("No file provided. Usage: cargo run <filename>");
        return;
    }
}

fn process_file(content: String) {
    for line in content.lines() {
        match parse_line(line) {
            Ok(Some(parsed)) => println!("{}", parsed),
            Ok(None) => {} // Skip empty lines and comments
            Err(e) => eprintln!("Error parsing line '{}': {}", line, e),
        }
    }
}

fn parse_line(line: &str) -> Result<Option<String>, String> {
    let trimmed = line.trim();

    // Skip empty lines
    if trimmed.is_empty() {
        return Ok(None);
    }

    // Skip comments
    if trimmed.starts_with("//") {
        return Ok(None);
    }

    // Remove inline comments
    let code = trimmed.split("//").next().unwrap_or("").trim();
    if code.is_empty() {
        return Ok(None);
    }

    // Parse A-instruction (@number)
    if let Some(addr_str) = code.strip_prefix('@') {
        if let Ok(addr) = addr_str.parse::<u16>() {
            return Ok(Some(format!("{:016b}", addr)));
        } else {
            return Err(format!("Invalid address: {}", addr_str));
        }
    }

    // Parse C-instruction (for now, just a placeholder)
    // TODO: Implement proper C-instruction parsing
    Ok(Some(format!("1110000000000000"))) // Placeholder
}
