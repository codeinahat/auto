mod read;
mod models;
mod utils;
mod commands;
mod eval;


fn main() {

    loop {
        
        let mut user_input_commands = String::new();

        read::read_line::prompt("\u{1F40F}(αυτό) >> ", &mut user_input_commands);
        
        let user_input_commands: Vec<&str> = user_input_commands.split_whitespace().collect();

        if user_input_commands[0] == ".exit" {
            println!("\n{} THANKS FOR USING AUTO HAT", '\u{1F44B}');
            break;
        }

        let list = commands::list();

        
        let active_command = match list.iter().find(|&c| c.name == user_input_commands[0]) {
            Some(cmd) => Some(cmd),
            None => None,
        };

        if active_command.is_none() {
            println!("Command not found")
        }
        else  {
            let cmnd = active_command.unwrap();
            cmnd.do_action();
        }

    }
}



