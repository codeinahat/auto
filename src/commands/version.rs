use crate::models::commands_model::Command;



/// Creates the version command
/// 
pub fn create_version() -> Command<'static> {
    Command::new("version", &root, Some(vec![

        Command::new("--pretty", &pretty_print, None)
    
    
    ]))
}


/// Root command for version command
/// 
fn root(_cmd: &Command) {
    println!("v0.0.0");
}

/// pretty print version command
/// 
fn pretty_print(_cmd: &Command) {
    println!("\u{1F40F} (αυτό)v0.0.0 \u{1F40F}");
}