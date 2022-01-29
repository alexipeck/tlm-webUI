use tlm_webui::WebUIFileVersion;

use {
    anyhow::Error,
    tlm_webui::{MessageSource, WebUIMessage, RequestType},
    yew::{prelude::*, services::{Task, console::ConsoleService, websocket::{WebSocketService, WebSocketTask, WebSocketStatus}}},
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

    Connect,
    Disconnected,
    Received(Result<String, Error>),
    EncodeAll,

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

struct Model {
    link: ComponentLink<Self>,
    test_value: i64,
    ws: Option<WebSocketTask>,
    //web_socket_service: WebSocketService,
    //console: ConsoleService,
    simple_console: String,
    rows: Vec<WebUIFileVersion>,
}

impl Model {
    fn add_to_console(&mut self, message: &str) {
        self.simple_console.push_str(&format!("[{}]\n", message))
    }

    fn add_file_version_to_context(&mut self, file_version: &WebUIFileVersion) {
        self.rows.push(file_version.clone());
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
            //web_socket_service: WebSocketService::default(),
            //console: ConsoleService::default(),
            simple_console: String::new(),
            rows: Vec::new(),
        };
        let cbout = model.link.callback(|data | Msg::Received(data));
                let cbnot = model.link.callback(|input| {
                    ConsoleService::log(&format!("Notification: {:?}", input));
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
        let cbout = self.link.callback(|data | Msg::Received(data));
        let cbnot = self.link.callback(|input| {
            ConsoleService::log(&format!("Notification: {:?}", input));
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
        match msg {
            Msg::Connect => {
                ConsoleService::log("Connect");
                self.add_to_console("Connect");
                let cbout = self.link.callback(|data | Msg::Received(data));
                let cbnot = self.link.callback(|input| {
                    ConsoleService::log(&format!("Notification: {:?}", input));
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
                //self.add_to_console(&message_string);
                if message_string.starts_with('{') {
                    let raw_message_source: Result<MessageSource, serde_json::Error> = serde_json::from_str(&message_string);
                    match raw_message_source {
                        Ok(message_source) => {
                            match message_source {
                                MessageSource::WebUI(WebUIMessage::FileVersions(file_versions)) => {
                                    for file_version in file_versions.iter() {
                                        //self.add_to_console(&file_version.file_name);
                                        self.add_file_version_to_context(file_version);
                                    }
                                },
                                _ => {
                                    //Not actually a WebUIMessage
                                    self.add_to_console(&message_string);
                                    return true;
                                },
                            }
                        },
                        Err(err) => {
                            //Not actually a WebUIMessage
                            self.add_to_console(&format!("Error converting json to MessageSource, Error: {}", err));
                            self.add_to_console(&message_string);
                            return true;
                        },
                    }
                } else {
                    self.add_to_console(&message_string);
                }
                self.test_value += 1;
				true    
			}
			Msg::Received(Err(err)) => {
                self.add_to_console(&format!("Error when reading data from server: {}", &err.to_string()));
                self.test_value += 1;
                true
			}
            Msg::AddOne => {
                self.test_value += 1;
                true
            },
            Msg::Import => {
                self.send_message("import");
                self.add_to_console("Import");
                self.test_value += 1;
                true
            },
            Msg::Process => {
                self.send_message("process");
                self.add_to_console("Process");
                self.test_value += 1;
                true
            },
            Msg::Hash => {
                self.send_message("hash");
                self.add_to_console("Hash");
                self.test_value += 1;
                true
            },
            Msg::EncodeAll => {
                self.send_message("encode_all");
                self.add_to_console("EncodeAll");
                self.test_value += 1;
                true
            },
            Msg::Ignore => {
                //Does nothing
                false
            },
            Msg::Request(request_type) => {
                self.send_request(request_type);
                self.add_to_console("Request");
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
        // This component has no properties so we will always return "false".
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
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::EncodeAll)>{ "Encode" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Request(RequestType::AllFileVersions))>{ "RequestFileVersions" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Connect)>{ if self.ws.is_some() { "" } else { "Connect" }}</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Test("test".to_string()))>{ "Test" }</a></td>
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
                                <tr class={classes!("clickable", "sidebar_element")}><a>{ "Import" }</a></tr>
                                <tr class={classes!("clickable", "sidebar_element")}><a>{ "Main" }</a></tr>
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
                                    <td><a> { format!("Connected: {} | ::Console::{}", self.ws.is_some(), self.simple_console.clone()) }</a></td>
                                </div>
                                {
                                    self.rows.clone().into_iter().map(|row| {
                                        html!{<div class={classes!("details_row")}>{ format!("{}", row) }</div>}
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