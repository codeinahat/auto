use crate::models::commands_model::{Command};
use crate::read::read_line::{use_default_prompt};

pub fn create_note() ->  Command<'static> {
    Command::new("note", &root, None)
}



fn root(_cmd: &Command) {
    
    let reply = use_default_prompt("Local or remote note? (local/remote)");

    println!("Your answer {}", reply);

}