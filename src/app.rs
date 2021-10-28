extern crate failure;
use yew::prelude::*;
use failure::Error;
use yew::{classes, html};
use yew::format::Json;
use yew::services::console::ConsoleService;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
//use crate::components::button::Button;

pub enum Msg {
    AddOne,
    RemoveOne,
    //Connect,                         // connect to websocket server
	Disconnected,                    // disconnected from server
	Ignore,                          // ignore this message
	TextInput(String),               // text was input in the input box
	SendText,                        // send our text to server
	Received(Result<String, Error>), // data received from server
}

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
    counter: usize,
    //console: ConsoleService,
	ws: Option<WebSocketTask>,
	//wss: WebSocketService,
	text: String,                    // text in our input box
	server_data: String,             // data received from the server
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            counter: 0,
            //console: ConsoleService::new(),//default
			ws: None,
			//wss: WebSocketService::connect("ws://localhost:8888"),
			text: String::new(),
			server_data: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.counter += 1;
            }
            Msg::RemoveOne => {
                self.counter -= if self.counter == 0 { 0 } else { 1 };
            }
            /* Msg::Connect => {
				self.console.log("Connecting");
				let cbout = self.link.send_back(|Json(data)| Msg::Received(data));
				let cbnot = self.link.send_back(|input| {
					ConsoleService::new().log(&format!("Notification: {:?}", input));
					match input {
						WebSocketStatus::Closed | WebSocketStatus::Error => {
							Msg::Disconnected
						}
						_ => Msg::Ignore,
					}
				});
				if self.ws.is_none() {
					let task = self.wss.connect("ws://localhost:8888", cbout, cbnot.into());
					self.ws = Some(task);
				}
				return true;
			} */
			Msg::Disconnected => {
				self.ws = None;
				return true;
			}
			Msg::Ignore => {
				return false;
			}
			Msg::TextInput(e) => {
				self.text = e; // note input box value
				return true;
			}
			Msg::SendText => {
				match self.ws {
					Some(ref mut task) => {
						task.send(Json(&self.text));
						self.text = "".to_string();
						return true; // clear input box
					}
					None => {
						return false;
					}
				}
			}
			Msg::Received(Ok(s)) => {
				self.server_data.push_str(&format!("{}\n", &s));
				return true;
			}
			Msg::Received(Err(s)) => {
				self.server_data.push_str(&format!("Error when reading data from server: {}\n", &s.to_string()));
				return true;
			}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
                            //<Button onsignal=self.link.callback(|_| Msg::RemoveOne) title="-1" />
                            //<Button onsignal=self.link.callback(|_| Msg::AddOne) title="+1" />  
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
                            </table>
                        </div>
                    </body>
                </div>
            </>
        }
    }
}
