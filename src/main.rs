use serde::{Deserialize, Serialize};
use tlm_webui::{MessageSource, WebUIMessage, RequestType};
use yew::binary_format;
use yew::{prelude::*, format::bincode};
use yew::services::Task;
use yew::services::websocket::WebSocketTask;
use std::panic;
use yew::format::{Text, Bincode, Json};

enum Msg {
    AddOne,
    Import,
    Process,
    Hash,
    RequestFileVersions,
    Ignore,
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
}

impl Model {
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
        let t = WebUIMessage::Request(request);
        let message_source_json = MessageSource::WebUI(t).to_json();
        let tries: usize = 3;
        for _ in 0..tries {
            match self.web_socket_task.as_mut() {
                Some(web_socket_task) => {
                    web_socket_task.send(Ok(message_source_json));
                    //let request_serialised = Bincode(&request);
                    /* let serialised = bincode::serialize::<RequestType>(&self).unwrap_or_else(|err| {
                        println!("Failed to deserialise message: {}", err);
                        panic!();
                    }); */
                    /* let t = Bincode::from(request); */
                    //web_socket_task.send_binary(request_serialised);
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
        };
        match yew::services::websocket::WebSocketService::connect_text::<Text>("ws://localhost:8888", model.link.callback(|_| Msg::Ignore), model.link.callback(|_| Msg::Ignore)) {
            Ok(web_socket_task) => {
                model.web_socket_task = Some(web_socket_task);
                wait_until_web_socket_is_open(&mut model);
            },
            Err(err) => {
                println!("Failed to connect websocket, error: {}", err);
            },
        }
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            },
            Msg::Import => {
                self.send_message("import");
                self.value += 1;

                true
            },
            Msg::Process => {
                self.send_message("process");
                self.value += 1;

                true
            },
            Msg::Hash => {
                self.send_message("hash");
                self.value += 1;

                true
            },
            Msg::Ignore => {
                //Does nothing
                false
            },
            Msg::RequestFileVersions => {
                self.send_request(RequestType::AllFileVersions);
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
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a>{ "Dashboard" }</a></td>  //Progress view
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a>{ "Import" }</a></td>     //Details view of directories/paths with relevant controls in the control bar.
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a>{ "Main" }</a></td>       //Details view of all imported with relevant controls in the control bar.
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a>{ "Organise" }</a></td>   //Details view of all imported with relevant controls in the control bar. 
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a>{ "Process" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::AddOne)>{ self.value }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Import)>{ "Import" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Process)>{ "Process" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::Hash)>{ "Hash" }</a></td>
                            <td class={classes!("clickable", "navbar_element", "navbar_table")}><a class={classes!("navbar_button")} onclick=self.link.callback(|_| Msg::RequestFileVersions)>{ "RequestFileVersions" }</a></td>
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
                            </table>
                        </div>
                    </body>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}