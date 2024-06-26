use std::io::Read;

pub fn process_from_input(input: &str) -> Result<String, anyhow::Error> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let str = buffer.trim();
    Ok(String::from(str))
}
