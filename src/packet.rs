
#[derive(Debug)]
pub enum Component {
    Text(String),
    Emoji(String),
    Tag(String),
    URL(String),
}

#[derive(Debug)]
pub enum Event {
    Start { new: bool },
    Follow { success: bool },
    Subscribe { streak: usize },
    Host { success: bool },
    Join { success: bool },
}

#[derive(Debug)]
pub enum Packet {
    Message {
        text: Vec<Component>,
        action: bool,
    },
    Ban { duration: Option<usize> },
    Event { kind: Event },
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Role {
    Banned,
    User,
    Subscriber,
    Moderator,
    Owner,
}

#[derive(Debug)]
pub struct Context {
    packet: Packet,
    channel: String,
    user: Option<String>,
    role: Option<Role>,
    target: Option<String>,
    service: String,
}
