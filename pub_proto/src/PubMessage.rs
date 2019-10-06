use std::fmt;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaMessage {
    pub a: u32,
    pub b: u32,
}

pub struct CommandMessage {
    pub meta: MetaMessage,
    pub command: String,
}

impl fmt::Display for MetaMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "this is:{},{}", self.a, self.b)
    }
}

impl Deref for CommandMessage {
    type Target = MetaMessage;

    fn deref<'a>(&'a self) -> &'a MetaMessage {
        &self.meta
    }
}

impl fmt::Display for CommandMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "command is {}", self.command)
    }
}
