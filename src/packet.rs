
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum Component {
    Text(String),
    Emoji(String),
    Tag(String),
    URL(String),
}

impl Component {
    /// Convert the component into a user-able string.
    pub fn to_string(&self) -> String {
        // TODO: Make this do more than just return whatever the crap we have
        match self {
            &Component::Text(ref text) => text.to_string(),
            &Component::Emoji(ref emoji) => emoji.to_string(),
            &Component::Tag(ref tag) => tag.to_string(),
            &Component::URL(ref url) => url.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    Start { new: bool },
    Follow { success: bool },
    Subscribe { streak: usize },
    Host { success: bool },
    Join { success: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Packet {
    Message { text: Vec<Component>, action: bool },
    Ban { duration: Option<usize> },
    Event { kind: Event },
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Banned,
    User,
    Subscriber,
    Moderator,
    Owner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub packet: Packet,
    pub channel: String,
    pub user: Option<String>,
    pub role: Option<Role>,
    pub target: Option<String>,
    pub service: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandContext {
}

pub fn string_components_to_string(components: Vec<Component>) -> Vec<String> {
    let mut finished: Vec<String> = Vec::new();

    for component in components {
        if let Component::Text(text) = component {
            finished.push(text);
        }
    }
    return finished;
}

#[macro_export]
macro_rules! url {
    ($url:expr) => {
        Component::URL($url.to_string())
    }
}

#[macro_export]
macro_rules! text {
    ($text:expr) => {
        text!($text, "")
    };
    ($text:expr, $($replacer:expr),*) => {
        {
            let mut current = $text.to_string();
            $(current = current.replacen("{}", $replacer, 1);)*
            Component::Text(current)
        }
    }
}

#[macro_export]
macro_rules! emoji {
    ($emoji:expr) => {
        Component::Emoji($emoji.to_string())
    }
}

#[macro_export]
macro_rules! tag {
    ($tag:expr) => {
        Component::Tag($tag.to_string())
    }
}
