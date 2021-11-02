use yew::prelude::*;
//use websocket::ClientBuilder;
//use websocket::message::Message;
use yew::services::websocket::{self, WebSocketTask};
//use yew::callback::Callback;
//use yew::services::ConsoleService;
use console_error_panic_hook;
use std::panic;
use yew::format::Text;
use anyhow::Error;

//use websocket::url::Url;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
//use futures_util::{future, pin_mut, StreamExt};

enum Msg {
    AddOne,
    Import,
    Blank,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    web_socket_task: Option<WebSocketTask>,
}

impl Model {
    fn make_web_socket(&mut self) {
        let ws_result = yew::services::websocket::WebSocketService::connect_text::<Text>("ws://localhost:8888", self.link.callback(|_| Msg::Blank), self.link.callback(|_| Msg::Blank));
        match ws_result {
            Ok(_) => {
                self.web_socket_task = Some(ws_result.unwrap());
            },
            Err(E) => {
                
            }
        }
    }

    fn send_message(&mut self, message: &str) {
        let tries: usize = 3;
        for _ in 0..tries {
            match self.web_socket_task.as_mut() {
                Some(t) => {
                    t.send(Ok(String::from(message)));
                    break;
                },
                None => {
                    self.make_web_socket();
                }
            }
        }
        
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        Self {
            link,
            value: 0,
            web_socket_task: None,
        }
    } 

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::Import => {
                self.send_message("import");
                self.value += 1;

                true
            }
            Msg::Blank => {
                //Does nothing
                false
            }
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
                                    <td class={classes!("row_portion")}><a>{ "Some test element and another little bit longer" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element and another little bit longer" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element and another little bit longer" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element and another little bit longer" }</a></td>
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
                                    //<td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                </tr></div>
                                <div class={classes!("details_row")}><tr>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    //<td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                </tr></div>
                                <div class={classes!("details_row")}><tr>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    //<td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                </tr></div>
                                <div class={classes!("details_row")}><tr>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    //<td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                </tr></div>
                                <div class={classes!("details_row")}><tr>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    //<td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                </tr></div>
                                <div class={classes!("details_row")}><tr>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    //<td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                </tr></div>
                                <div class={classes!("details_row")}><tr>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    <td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
                                    //<td class={classes!("row_portion")}><a>{ "Some test element" }</a></td>
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