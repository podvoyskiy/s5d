pub trait Colorize {
    fn green(&self)  -> String;
    fn red(&self)    -> String;
    fn orange(&self) -> String;
    fn cyan(&self)   -> String;
}

impl<T: AsRef<str>> Colorize for T {
    fn green(&self)  -> String { format!("\x1b[32m{}\x1b[0m", self.as_ref()) }
    fn red(&self)    -> String { format!("\x1b[31m{}\x1b[0m", self.as_ref()) }
    fn orange(&self) -> String { format!("\x1b[33m{}\x1b[0m", self.as_ref()) }
    fn cyan(&self)   -> String { format!("\x1b[36m{}\x1b[0m", self.as_ref()) }
}