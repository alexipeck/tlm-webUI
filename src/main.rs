use std::collections::HashSet;

use tlm_webui::{WebUIFileVersion, WebUIShow};

use {
    anyhow::Error,
    tlm_webui::{MessageSource, WebUIMessage, RequestType},
    yew::{prelude::*, services::{Task, websocket::{WebSocketService, WebSocketTask, WebSocketStatus}}},
    std::panic,
    yew::format::Text,
};

enum Msg {
    AddOne,
    Import,
    Process,
    Hash,
    Request(RequestType),
    Test(String),

    Reconnect,
    Disconnected,
    Received(Result<String, Error>),
    EncodeAll,
    Encode(i32, i32),

    Ignore,
}

fn wait_until_web_socket_is_open(structure: &mut Model) {
    loop {
        match structure.ws.as_mut() {
            Some(web_socket_task) => {
                while !web_socket_task.is_active() {
                    //Do nothing but take time
                }
                break;
            }
            None => {
                match yew::services::websocket::WebSocketService::connect_text::<Text>("ws://localhost:8888", structure.link.callback(|_| Msg::Ignore), structure.link.callback(|_| Msg::Ignore)) {
                    Ok(web_socket_task) => {
                        structure.ws = Some(web_socket_task);
                        continue;
                    },
                    Err(err) => {
                        println!("Failed to connect websocket, error: {}", err);
                    }
                }
            }
        }
    }
}

pub enum Tab {
    Shows,
    FileVersions,
}

pub struct DataContext{
    file_versions: HashSet<WebUIFileVersion>,
    //profiles: HashSet<EncodeProfile>,
    shows: HashSet<WebUIShow>,
}

impl DataContext {
    pub fn default() -> Self {
        Self {
            file_versions: HashSet::new(),
            shows: HashSet::new(),
        }
    }
}

struct Model {
    link: ComponentLink<Self>,
    test_value: i64,
    ws: Option<WebSocketTask>,
    data: DataContext,
    current_tab: Tab,
}

impl Model {
    fn add_file_version_to_context(&mut self, file_version: &WebUIFileVersion) {
        self.data.file_versions.insert(file_version.clone());
    }

    fn add_shows_to_context(&mut self, show: &WebUIShow) {
        self.data.shows.insert(show.clone());
    }

    fn send_message(&mut self, message: &str) {
        let tries: usize = 3;
        for _ in 0..tries {
            match self.ws.as_mut() {
                Some(web_socket_task) => {
                    web_socket_task.send(Ok(String::from(message)));
                    break;
                },
                None => {
                    wait_until_web_socket_is_open(self);
                },
            }
        }
    }

    fn send_command(&mut self, message: WebUIMessage) {
        let tries: usize = 3;
        for _ in 0..tries {
            match self.ws.as_mut() {
                Some(web_socket_task) => {
                    web_socket_task.send(Ok(MessageSource::from_webui_message(message).to_json()));
                    break;
                },
                None => {
                    wait_until_web_socket_is_open(self);
                },
            }
        }
    }

    fn send_string(&mut self, send_text: String) {
        let tries: usize = 3;
        for _ in 0..tries {
            match self.ws.as_mut() {
                Some(web_socket_task) => {
                    web_socket_task.send(Ok(send_text));
                    break;
                },
                None => {
                    wait_until_web_socket_is_open(self);
                },
            }
        }
    }

    fn send_request(&mut self, request: RequestType) {
        let t = MessageSource::WebUI(WebUIMessage::Request(request));
        let message_source_json = t.to_json();
        let tries: usize = 3;
        for _ in 0..tries {
            match self.ws.as_mut() {
                Some(web_socket_task) => {
                    web_socket_task.send(Ok(message_source_json));
                    break;
                },
                None => {
                    wait_until_web_socket_is_open(self);
                },
            }
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let mut model = Self {
            ws: None,
            link,
            test_value: 0,
            data: DataContext::default(),
            current_tab: Tab::FileVersions,
        };
        let cbout = model.link.callback(|data | Msg::Received(data));
        let cbnot = model.link.callback(|input| {
            match input {
                WebSocketStatus::Closed | WebSocketStatus::Error => {
                    Msg::Disconnected
                }
                _ => Msg::Ignore,
            }
        });
        model.ws = Some(WebSocketService::connect_text("ws://localhost:8888", cbout, cbnot).unwrap());
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        /* if self.ws.is_none() {
            let cbout = self.link.callback(|data | Msg::Received(data));
            let cbnot = self.link.callback(|input| {
                match input {
                    WebSocketStatus::Closed | WebSocketStatus::Error => {
                        Msg::Disconnected
                    }
                    _ => Msg::Ignore,
                }
            });
            let task = WebSocketService::connect_text("ws://localhost:8888", cbout, cbnot);
            self.ws = Some(task.unwrap());
            self.test_value += 1;
        } */
        
        match self.current_tab {
            Tab::FileVersions => {
                self.link.callback(|_: ()| Msg::Request(RequestType::AllFileVersions));
            },
            Tab::Shows => {},
        }

        
        match msg {
            Msg::Reconnect => {
                let cbout = self.link.callback(|data | Msg::Received(data));
                let cbnot = self.link.callback(|input| {
                    match input {
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Msg::Disconnected
                        }
                        _ => Msg::Ignore,
                    }
                });
                if self.ws.is_none() {
                    let task = WebSocketService::connect_text("ws://localhost:8888", cbout, cbnot);
                    self.ws = Some(task.unwrap());
                }
                self.test_value += 1;
                true
            },
            Msg::Disconnected => {
                self.ws = None;
                self.test_value += 1;
                true
            },
            Msg::Received(Ok(message_string)) => {
                if message_string.starts_with('{') {
                    let raw_message_source: Result<MessageSource, serde_json::Error> = serde_json::from_str(&message_string);
                    match raw_message_source {
                        Ok(message_source) => {
                            match message_source {
                                MessageSource::WebUI(WebUIMessage::FileVersions(file_versions)) => {
                                    for file_version in file_versions.iter() {
                                        self.add_file_version_to_context(file_version);
                                    }
                                },
                                MessageSource::WebUI(WebUIMessage::Shows(shows)) => {
                                    for show in shows.iter() {
                                        self.add_shows_to_context(show);
                                    }
                                },
                                _ => {
                                    //Not actually a WebUIMessage
                                    return false;
                                },
                            }
                        },
                        Err(_) => {
                            //Not actually a WebUIMessage
                            return false;
                        },
                    }
                } else {
                }
                self.test_value += 1;
				true    
			}
			Msg::Received(Err(err)) => {
                self.test_value += 1;
                true
			}
            Msg::AddOne => {
                self.test_value += 1;
                true
            },
            Msg::Import => {
                self.send_message("import");
                self.test_value += 1;
                true
            },
            Msg::Process => {
                self.send_message("process");
                self.test_value += 1;
                true
            },
            Msg::Hash => {
                self.send_message("hash");
                self.test_value += 1;
                true
            },
            Msg::EncodeAll => {
                self.send_message("encode_all");
                self.test_value += 1;
                true
            },
            Msg::Encode(generic_uid, id) => {
                self.send_command(WebUIMessage::Encode(generic_uid, id));
                self.test_value += 1;
                true
            },
            Msg::Ignore => {
                //Does nothing
                false
            },
            Msg::Request(request_type) => {
                self.send_request(request_type);
                self.test_value += 1;
                true
            },
            Msg::Test(string) => {
                self.send_string(string);
                true
            },
            _ => {
                false
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <nav>
                    //<img src="TLM Icon.png" width="500" height="500"/>
                    <table>
                        <tr>
                        //Progress view
                        //Details view of directories/paths with relevant controls in the control bar.
                        //Details view of all imported with relevant controls in the control bar.
                        //Details view of all imported with relevant controls in the control bar.
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::AddOne)>{ self.test_value }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Import)>{ "Import" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Process)>{ "Process" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Hash)>{ "Hash" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Request(RequestType::AllFileVersions))>{ "RequestFileVersions" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Test("test".to_string()))>{ "Test" }</a></td>
                            {
                                if self.ws.is_none() {
                                    html!{
                                        <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Reconnect)>{ "Reconnect" }</a></td>
                                    }
                                } else {
                                    html!{}
                                }
                            }
                        </tr>
                    </table>
                </nav>
                <div class={classes!("main")}>
                    <body>
                        //Navbar
                        <div class={classes!("sidebar")}>
                            //sidebar elements will have a child if they have more than one page
                            <table class={classes!("sidebar")}>
                                <tr class={classes!("clickable", "sidebar_element")}><a>{ "Dashboard" }</a></tr>
                                <tr class={classes!("clickable", "sidebar_element")}><a>{ "Profiles" }</a></tr>
                                <tr class={classes!("clickable", "sidebar_element")}><a>{ "List" }</a></tr>
                                <tr class={classes!("clickable", "sidebar_element", "sidebar_element_child")}><a>{ "Organise" }</a></tr>
                                <tr class={classes!("clickable", "sidebar_element", "sidebar_element_child")}><a>{ "Process" }</a></tr>
                            </table>
                        </div>

                        //Filter/Static control
                        <div class={classes!("filter_control")}>
                            
                        </div>

                        //Details View
                        <div class={classes!("main")}>
                            <table class={classes!("details_table")}>
                                <div class={classes!("details_row")}>
                                    <td><a> { format!("Connected: {}", self.ws.is_some()) }</a></td>
                                </div>
                                {
                                    self.data.file_versions.clone().into_iter().map(|row| {
                                        let generic_uid = row.generic_uid.clone();
                                        let id = row.id.clone();
                                        let file_name = row.file_name.clone();
                                        html!{
                                            <div class={classes!("details_row")}>
                                                <th class={classes!("row_portion")}><a>{ format!("{}", file_name) }</a></th>
                                                <th class={classes!("row_portion")}><a>{ format!("{}", generic_uid) }</a></th>
                                                <th class={classes!("row_portion")}><a>{ format!("{}", id) }</a></th>
                                                <th class={classes!("row_portion")}><button onclick=self.link.callback( move |_| Msg::Encode(generic_uid, id))>{ "Encode" }</button></th>
                                            </div>
                                        }
                                        //html!{<div class={classes!("details_row")}>{ format!("{}", row) }</div>}
                                    }).collect::<Html>()
                        
                                    //self.rows.iter().to_string()
                                    /* for t in self.rows.iter() {

                                    } */
                                    //self.rows.iter().collect::<Html>()
                                }
                                
                            </table>
                        </div>
                    </body>
                </div>
            </>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
    //Shorthand for the above
    //yew::start_app::<Model>();
}