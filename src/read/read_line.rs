use crate::utils::sanitation;
use std::io::{self, Write};



/// STDOUT prompt. takes `&str display_text`.
/// 
/// Returns user input
/// 
/// 
pub fn prompt(display_text: &str, commands_storage: &mut String) {

    print!("{display_text}");

    io::stdout().flush().expect("Error writing STDOUT");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading from STDIN");

    sanitation::clean_string(&mut input);

    *commands_storage = input;
}


/// Returns the main program prompt and adds to it
/// 
pub fn use_default_prompt(appended_text: &str) -> String {
    
    let mut reply = String::new();
    
    let user_prompt: String = format!("{}{}{}", "\u{1F40F}(αυτό) [", appended_text, "] >> ");

    prompt(&user_prompt, &mut reply);

    reply
}