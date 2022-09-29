use crate::models::commands_model::{Command};
use crate::read::read_line::{use_default_prompt, prompt};
use crate::utils::sanitation;

pub fn create_note() ->  Command<'static> {
    Command::new("note", &root, None)
}




fn root(_cmd: &Command) {
    note_interface(true);
}

fn note_interface(first: bool) {
    
    if first == true {
        let mut reply = use_default_prompt("Local or remote note? (local/remote)");
        sanitation::clean_string(&mut reply);
        if reply.to_lowercase() == "local" {
            create_local_note_dir().expect("Error creating local dirctory");
        } 
    }

    let mut text = String::new();
    let mut my_note = String::new();

    println!("End note with --end\n\n");

    writing_interface(&mut text, &mut my_note);

    println!("\n");

    sanitation::clean_string(&mut my_note);

    let my_note: String = my_note.drain(..(my_note.chars().count() - 5)).collect();

    check_if_note_is_ready(&my_note);
}


/// Checks if the written note is ready
/// 
fn check_if_note_is_ready(note: &String) {
    
    println!("\"{}\"\n", note);
    
    let mut is_ready = use_default_prompt("Is the above note ready? (y/n)");

    sanitation::clean_string(&mut is_ready);

    if is_ready == "y" {
        let mut note_name = use_default_prompt("Whats the name of the note?");

        sanitation::clean_string(&mut note_name);

        match save_note(&note_name, note) {
            Ok(_rslt) => println!("Note have been saved"),
            Err(error) => println!("Failed to save note, {}", error)
        };
        
    }
    else if is_ready == "n" {
        note_interface(false);
    }
}
 

/// Creates multiline read from STDIN
fn writing_interface(text: &mut String, my_note: &mut String) {
    loop {
        std::io::stdin().read_line(text).expect("Error from STDIN");

        my_note.push_str(&text);
        
        sanitation::clean_string( text);     
        
        if text.chars().count() >= 5 {
            let count = text.chars().count() - 5;
            text.drain(..count);
        }

        // println!(" from drain \n{text}\n");

        if  text == "--end" {
            break;
        };

        *text = String::new();
    }
}

/// Create local auto directore for notes
/// 
fn create_local_note_dir() -> Result<bool, std::io::Error> {

    std::fs::create_dir_all("./.auto/notes")?;

    Ok(true)
}

fn save_note(note_name: &String, note_content: &String) -> Result<bool, std::io::Error> {

    let path = format!("{}{}","./.auto/notes/", note_name);

    println!("{path}");
    
    std::fs::write(path, note_content)?;

    Ok(true)
}