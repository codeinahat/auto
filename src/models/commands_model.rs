
pub struct Command<'a> {
    pub name: &'a str,
    run: &'a dyn Fn(&Self),
    pub flags: Option<Vec<Command<'a>>>,
}

impl<'a> Command<'a> {

    /// Creates new [Command]
    /// 
    pub fn new<F: Fn(&Self)>(name: &'a str, func: &'a F, subs: Option<Vec<Command<'a>>>) -> Self {
        Command {
            name,
            run: func,
            flags: match subs {
                None => {
                    // println!("empty for {name}");
                    None
                }
                Some(subs) => {
                    // println!("some flags for {name}");
                    Some(subs)
                }
            }
        }
    }

    
    /// Executes command action
    /// 
    pub fn do_action(&self)  {
        (self.run)(self);
    }

}