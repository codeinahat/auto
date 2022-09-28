



/// Cleans a `&mut` [String]
/// 
/// Removes `white spaces` and new `lines`
pub fn clean_string(cmd: &mut String) {

    // removes white spaces
    *cmd = String::from(cmd.trim());
    
    // removes new line
    *cmd = cmd.replace("\n", "");
}