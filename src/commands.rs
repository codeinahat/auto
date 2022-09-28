pub mod version;
pub mod clear;
pub mod note;

use crate::models::commands_model::Command;


/// Create commands list
/// 
pub fn list() -> [Command<'static>; 2] { 
    [
        version::create_version(),
        note::create_note()
        // clear::create_clear()
    ]
}
