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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WebUIShow {
    pub show_uid: i32,
    pub show_title: String,
    pub episode_count: usize,
}

impl fmt::Display for WebUIShow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "show_uid: {}, show_title: {}, episode_count: {}", self.show_uid, self.show_title, self.episode_count)
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
    AllProfiles,
    AllGenerics,
    AllShows,
    FileVersions(usize, usize), //specified range, this will be used for indexing the server's storage structure
    Generics(usize, usize), //specified range, this will be used for indexing the server's storage structure
}

#[derive(Serialize, Deserialize)]
pub enum WebUIMessage {
    //WebUI -> Server
    Request(RequestType),
    //EncodeGeneric(i32, i32, AddEncodeMode, EncodeProfile),
    
    //Server -> WebUI
    Shows(Vec<WebUIShow>),
    Encode(i32, i32),
    FileVersion(i32, i32, String),
    FileVersions(Vec<WebUIFileVersion>),
    Generic(i32, i32, String),
    //Generics(Vec<WebUIGeneric>),
}

impl WebUIMessage {}