use {
    std::fmt,
    serde::{Serialize, Deserialize},
};

#[derive(Serialize, Deserialize)]
pub enum WorkerMessage {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WebUIFileVersion {
    pub generic_uid: i32,
    pub id: i32,
    pub file_name: String,
}

impl fmt::Display for WebUIFileVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "generic_uid: {}, id: {}, file_name: {}", self.generic_uid, self.id, self.file_name)
    }
}

#[derive(Serialize, Deserialize)]
pub enum MessageSource {
    Worker(WorkerMessage),
    WebUI(WebUIMessage),
}

impl MessageSource {
    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => {
                json
            },
            Err(err) => {
                println!("Failed converting MessageSource to json string, error output: {}", err);
                panic!();
            },
        }
    }

    pub fn expect_webui_message(json: String) -> Option<WebUIMessage> {
        let raw_message_source: Result<Self, serde_json::Error> = serde_json::from_str(&json);
        match raw_message_source {
            Ok(message_source) => {
                match message_source {
                    MessageSource::WebUI(webui_message) => {
                        return Some(webui_message);
                    },
                    _ => return None,
                }
            },
            Err(err) => {
                println!("Failed converting json string to MessageSource, error output: {}", err);
            },
        }
        None
    }

    pub fn from_webui_message(webui_message: WebUIMessage) -> Self {
        Self::WebUI(webui_message)
    }
}

#[derive(Serialize, Deserialize)]
pub enum RequestType {
    AllFileVersions,
}

#[derive(Serialize, Deserialize)]
pub enum WebUIMessage {
    //WebUI -> Server
    Request(RequestType),
    //EncodeGeneric(i32, i32, AddEncodeMode, EncodeProfile),
    
    //Server -> WebUI
    Encode(i32, i32),
    FileVersion(i32, i32, String),
    FileVersions(Vec<WebUIFileVersion>),
}

impl WebUIMessage {}











/* #[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct WebUIFileVersion {
    pub generic_uid: u32,
    pub file_version_id: u32,
    pub file_name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum RequestType {
    AllFileVersions,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum WebUIMessage {
    //EncodeGeneric(i32, i32, AddEncodeMode, EncodeProfile),
    FileVersion(i32, i32, String),
    FileVersions(Vec<WebUIFileVersion>),
    Request(RequestType),
} */

/* impl WebUIMessage {
    ///Convert WorkerMessage to a tungstenite message for sending over websockets
    pub fn to_message(&self) -> Message {
        let serialised = bincode::serialize(self).unwrap_or_else(|err| {
            println!("Failed to serialise WorkerMessage: {}", err);
            panic!();
        });
        Message::binary(serialised)
    }

    pub fn from_message(message: Message) -> Self {
        bincode::deserialize::<Self>(&message.into_data()).unwrap_or_else(|err| {
            println!("Failed to deserialise message: {}", err);
            panic!();
        })
    }
} */