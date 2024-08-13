use std::io::Write;

pub fn get_with_prompt(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input.replace("\n", "")
}
