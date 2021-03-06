use std::io;

pub fn read_user_input() -> Result<u16, &'static str> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => parse_input_to_int(input.replace("\n", "")),
        Err(_) => Err("Couldn't read input")
    }
}

fn parse_input_to_int(input: String) -> Result<u16, &'static str> {
    match input.parse() {
        Ok(result) => Ok(result),
        Err(_) => Err("Couldn't parse number")
    }
}
