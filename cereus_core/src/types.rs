
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
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

    pub fn is_text(&self) -> bool {
        match self {
            &Component::Text(_) => true,
            _ => false
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub quote_id: i64,
    pub response: Vec<Component>,
    pub channel: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandMeta {
    pub added_by: String,
    pub cooldown: i32,
    pub count: i32,
    pub enabled: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
    pub channel: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
    pub meta: CommandMeta,
    pub name: String,
    pub response: Vec<Component>,
    pub services: Vec<String>,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trust {
    pub channel: String,
    pub trusted: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Social {
    pub channel: String,
    pub service: String,
    pub url: String
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub packet: Packet,
    pub channel: String,
    pub user: Option<String>,
    pub role: Option<Role>,
    pub target: Option<String>,
    pub service: Option<String>,
    pub count: Option<u32>
}

impl Context {

    pub fn message(components: Vec<Component>) -> Self {
        Context {
            packet: Packet::Message { text: components, action: false },
            channel: String::new(),
            user: None,
            role: None,
            target: None,
            service: None,
            count: None
        }
    }

    pub fn target_message(target: Option<String>, components: Vec<Component>) -> Self {
        Context {
            packet: Packet::Message { text: components, action: false },
            channel: String::new(),
            user: None,
            role: None,
            target,
            service: None,
            count: None
        }
    }

    pub fn event(event: Event) -> Self {
        Context {
            packet: Packet::Event { kind: event },
            channel: String::new(),
            user: None,
            role: None,
            target: None,
            service: None,
            count: None
        }
    }

    pub fn from_packet(packet: Packet) -> Self {
        Context {
            packet,
            channel: String::new(),
            user: None,
            role: None,
            target: None,
            service: None,
            count: None
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

    pub fn merge(mut self, context: &Context) -> Context {
        self.channel = context.channel.clone();
        self.service = context.service.clone();
        self.user = context.user.clone();
        self
    }

    pub fn cut(mut self, index: usize) -> Context {
        if let Packet::Message { ref mut text, action: _ } = self.packet.clone() {
            let (_, remaining) = text.split_at(index);
            self.packet = Packet::Message { text: remaining.to_vec(), action: false };  // TODO: Pass action in
        }
        self
    }

    pub fn get_packet_content(&self) -> Vec<Component> {
        match &self.packet {
            &Packet::Message { ref text, action: _ } => text.to_vec(),
            _ => vec! []  // TODO: Maybe I should change this to an option?
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct QuoteAddResponse {
    pub created: bool,
    pub id: u32
}

#[macro_export]
macro_rules! url {
    ($url:expr) => {
        cereus_core::types::Component::URL($url.to_string())
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
           cereus_core::types::Component::Text(current)
        }
    }
}

#[macro_export]
macro_rules! emoji {
    ($emoji:expr) => {
        cereus_core::types::Component::Emoji($emoji.to_string())
    }
}

#[macro_export]
macro_rules! tag {
    ($tag:expr) => {
        cereus_core::types::Component::Tag($tag.to_string())
    }
}

// TODO: elimate this.
pub fn string_components_to_string(components: Vec<Component>) -> Vec<String> {
    let mut finished: Vec<String> = Vec::new();

    for component in components {
        finished.push(component.to_string());
    }
    return finished;
}

