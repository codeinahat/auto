use crate::models::commands_model::{Command};
use crate::read::read_line::{use_default_prompt, prompt};
use crate::utils::sanitation;

pub fn create_note() ->  Command<'static> {
    Command::new("note", &root, None)
}



fn root(_cmd: &Command) {
    
    let reply = use_default_prompt("Local or remote note? (local/remote)");

    let mut text = String::new();
    let mut my_note = String::new();

    println!("End note with --end");

    loop {
        std::io::stdin().read_line(&mut text).expect("Error from STDIN");

        my_note.push_str(&text);
        
        sanitation::clean_string(&mut text);     
        
        if text.chars().count() >= 5 {
            let count = text.chars().count() - 5;
            text.drain(..count);
        }

        // println!(" from drain \n{text}\n");

        if  text == "--end" {
            break;
        };

        text = String::new();
    }

    let my_note: String = my_note.drain(..(my_note.chars().count() - 6)).collect();

    println!("\n\n");

    println!("{my_note}");
}