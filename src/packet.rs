
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    Start { new: bool },
    Follow { success: bool },
    Subscribe { streak: usize },
    Host { success: bool },
    Join { success: bool },
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub packet: Packet,
    pub channel: String,
    pub user: Option<String>,
    pub role: Option<Role>,
    pub target: Option<String>,
    pub service: String
}

impl Context {

    pub fn message(components: Vec<Component>) -> Self {
        Context {
            packet: Packet::Message { text: components, action: false },
            channel: String::new(),
            user: None,
            role: None,
            target: None,
            service: String::new()
        }
    }

    pub fn event(event: Event) -> Self {
        Context {
            packet: Packet::Event { kind: event },
            channel: String::new(),
            user: None,
            role: None,
            target: None,
            service: String::new()
        }
    }

    pub fn from_packet(packet: Packet) -> Self {
        Context {
            packet,
            channel: String::new(),
            user: None,
            role: None,
            target: None,
            service: String::new()
        }
    }

    pub fn sub(&mut self, pattern: regex::Regex, repl: &str) -> Option<String> {
        if let Packet::Message { ref mut text, action: _ } = self.packet.clone() {

            for (i, chunk) in text.clone().iter().enumerate() {
                match chunk {
                    Component::Text(t) => {
                        let filled = pattern.replace(t, repl);
                        text[i] = Component::Text(filled.to_string());
                    },
                    _ => continue
                }
            }
        }
        None
    }
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
        $crate::packet::Component::URL($url.to_string())
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
            $crate::packet::Component::Text(current)
        }
    }
}

#[macro_export]
macro_rules! emoji {
    ($emoji:expr) => {
        $crate::packet::Component::Emoji($emoji.to_string())
    }
}

#[macro_export]
macro_rules! tag {
    ($tag:expr) => {
        $crate::packet::Component::Tag($tag.to_string())
    }
}
