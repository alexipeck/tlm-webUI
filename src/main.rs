use anyhow::Error;
use tlm_webui::{MessageSource, WebUIMessage, RequestType};
use yew::{services::console::ConsoleService, prelude::*};
use yew::services::{Task, WebSocketService};
use yew::services::websocket::{WebSocketTask, WebSocketStatus};
use std::panic;
use yew::format::Text;

enum Msg {
    AddOne,
    Import,
    Process,
    Hash,
    Request(RequestType),
    Ignore,
    Connect,
    Disconnected,
    Received(Result<String, Error>),
    H,
}

fn wait_until_web_socket_is_open(structure: &mut Model) {
    loop {
        match structure.web_socket_task.as_mut() {
            Some(web_socket_task) => {
                while !web_socket_task.is_active() {
                    //Do nothing but take time
                }
                break;
            }
            None => {
                match yew::services::websocket::WebSocketService::connect_text::<Text>("ws://localhost:8888", structure.link.callback(|_| Msg::Ignore), structure.link.callback(|_| Msg::Ignore)) {
                    Ok(web_socket_task) => {
                        structure.web_socket_task = Some(web_socket_task);
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
    value: i64,
    web_socket_task: Option<WebSocketTask>,
    web_socket_service: WebSocketService,
    console: ConsoleService,
    simple_console: String,
}

impl Model {
    fn add_to_console(&mut self, message: &str) {
        self.simple_console.push_str(&format!("{}\n", message))
    }

    fn send_message(&mut self, message: &str) {
        let tries: usize = 3;
        for _ in 0..tries {
            match self.web_socket_task.as_mut() {
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

    fn send_request(&mut self, request: RequestType) {
        let t = MessageSource::WebUI(WebUIMessage::Request(request));
        let message_source_json = t.to_json();
        let tries: usize = 3;
        for _ in 0..tries {
            match self.web_socket_task.as_mut() {
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
            link,
            value: 0,
            web_socket_task: None,
            web_socket_service: WebSocketService::default(),
            console: ConsoleService::default(),
            simple_console: String::new(),
        };
        match WebSocketService::connect_text::<Text>("ws://localhost:8888", model.link.callback(|_| Msg::Ignore), model.link.callback(|_| Msg::Ignore)) {
            Ok(web_socket_task) => {
                model.web_socket_task = Some(web_socket_task);
                wait_until_web_socket_is_open(&mut model);
            },
            Err(err) => {
                model.add_to_console(&format!("Failed to connect websocket, error: {}", err))
            },
        }
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                self.add_to_console("Added one.");
                true
            },
            Msg::Import => {
                self.send_message("import");
                self.value += 1;
                self.add_to_console("Trigger Import.");
                true
            },
            Msg::Process => {
                self.send_message("process");
                self.value += 1;
                self.add_to_console("Trigger Process");
                true
            },
            Msg::Hash => {
                self.send_message("hash");
                self.value += 1;
                self.add_to_console("Trigger Hash.");
                true
            },
            Msg::Ignore => {
                //Does nothing
                false
            },
            Msg::Request(request_type) => {
                self.send_request(request_type);
                self.add_to_console("Requesting stuff");
                true
            },
            Msg::Connect => {
                //self.console.log("F Test");
                yew::services::ConsoleService::log("Connect");
                //let t = self.link.
                //let cbout = self.link.callback(function)|Json(data)| Msg::Received(data);
				//let cbnot = self.link.send_back(|input| {
				//	yew::services::ConsoleService::log(&format!("Notification: {:?}", input));
				//	match input {
				//		WebSocketStatus::Closed | WebSocketStatus::Error => {
				//			Msg::Disconnected
				//		}
				//		_ => Msg::Ignore,
				//	}
				//});
				//if self.web_socket_task.is_none() {
				//	let task = WebSocketService::connect("ws://127.0.0.1:8888", cbout, cbnot.into());
                //    match task {
                //        Ok(websocket_task) => {
                //            self.web_socket_task = Some(websocket_task);
                //        },
                //        Err(err) => {
                //            println!("{}", err);
                //            panic!();
                //        },
                //    }
				//}
                false
            },
            Msg::Disconnected => {
                //Does nothing
                false
            },
            Msg::Received(Ok(message_string)) => {
                if message_string.starts_with('{') {
                    let json = message_string;
                    let message_source = MessageSource::from_json(json);
                    match message_source {
                        MessageSource::WebUI(webui_message) => {
                            if let WebUIMessage::FileVersions(file_versions) = webui_message {
                                self.value += file_versions.len() as i64;
                                    return true;
                            }
                        },
                        _ => {
                            return false;
                        }
                    }
                }
                self.add_to_console("Received data.");
				true    
			}
			Msg::Received(Err(message_string)) => {
				/* self.server_data.push_str(&format!("Error when reading data from server: {}\n", &message_string.to_string()));
				true */
                self.add_to_console(&format!("Error when reading data from server: {}\n", &message_string.to_string()));
                true
			}
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
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::AddOne)>{ self.value }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Import)>{ "Import" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Process)>{ "Process" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Hash)>{ "Hash" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Request(RequestType::AllFileVersions))>{ "RequestFileVersions" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Connect)>{ "Test" }</a></td>
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
                                <div class={classes!("details_row")}><tr>
                                    <th class={classes!("row_portion")}><a>{ "Path" }</a></th>
                                    <th class={classes!("row_portion")}><a>{ "" }</a></th>
                                    <th class={classes!("row_portion")}><a>{ "" }</a></th>
                                    <th class={classes!("row_portion")}><a>{ "" }</a></th>
                                </tr></div>
                                
                                <div class={classes!("details_row")}><tr>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                </tr></div>
                                <div class={classes!("details_row")}><tr>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                </tr></div>
                                <div class={classes!("details_row")}>
                                    <td><a></a>{ &self.simple_console }</td>
                                </div>
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