use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{ConsoleService};
use stdweb::web::event::KeyDownEvent;
use stdweb::web::event::IKeyboardEvent;
use stdweb::web::event::IEvent;

mod content;
use content::{Content, CursorPos, GetString, prettify_code};

pub struct Model {
    console: ConsoleService,
    link: ComponentLink<Model>,
    text: String,
    cursor: CursorPos,
    cycle: Vec<CursorPos>,
    cycle_id: usize,
    content: Content,
    auto_update: bool,
    window_width: usize
}

pub enum Msg {
    GotInput(String),
    KeyEvt(KeyDownEvent),
    Ignore,
    ClearVirtualWhitespace,
    Format,
    ToggleAutoUpdate,
    UpdateWidth(usize)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        //let typed = "fn test(&self,  other:\n  \n&mut usize){let x=(self+1)*other;\n return1<y}";
        let typed = "fn test(other:&mut usize){let array=[1123456, 531432124, 43241432, 4312432, 9432, 432,4328,432];let x=(self+1)*other;return 1<y}";
        let visible = "fn test(other: &mut usize) {\n    let x = (self + 1) * other;\n    return 1 < y\n}";
        let content = Content::from_strings(&typed, &typed);
        
        let input = "fn   test(){println!(\"x\");}";
        let exp = "stdin:\n\nfn test() {\n    println!(\"x\");\n}\n";
        
        let mut console = ConsoleService::new();
        
        //console.log(&prettify_code(input.to_string()));
        //console.log(&prettify_code(input.to_string()));
        //console.log(&prettify_code(input.to_string()));

        Model {
            console,
            link,
            text: content.get_string(), //"fn test() {\n    println!(\"hello\")\n}".to_string(),
            cursor: content.cursor_pos(), //CursorPos { line: 0, col: 9, between: true },
            cycle: vec!(
                CursorPos { line: 0, col: 9, between: true },
                CursorPos { line: 0, col: 9, between: false },
                CursorPos { line: 0, col: 8, between: false },
                CursorPos { line: 0, col: 0, between: false },
                CursorPos { line: 1, col: 0, between: false },
                CursorPos { line: 1, col: 4, between: false },
                CursorPos { line: 1, col: 6, between: false },
                CursorPos { line: 1, col: 21, between: false },
            ),
            cycle_id: 0,
            content,
            auto_update: false,
            window_width: 100
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::KeyEvt(e) => {
                e.stop_propagation();
                e.prevent_default();
                match e.key().as_ref() {
                    "ArrowLeft" => {
                        // self.cycle_id = (self.cycle_id + self.cycle.len() - 1) % self.cycle.len();
                        // self.cursor = self.cycle[self.cycle_id].clone();
                        self.content.cursor_left();
                        self.cursor = self.content.cursor_pos();
                    },
                    "ArrowRight" => {
                        // self.cycle_id = (self.cycle_id + 1) % self.cycle.len();
                        // self.cursor = self.cycle[self.cycle_id].clone();
                        self.content.cursor_right();
                        self.cursor = self.content.cursor_pos();
                    },
                    "Backspace" => {
                        self.content.backspace();
                        if self.auto_update {
                            let res = self.content.update_virtual_whitespace(self.window_width);
                            self.console.log(&res);
                        }
                        self.cursor = self.content.cursor_pos();
                        self.text = self.content.get_string();
                        //self.console.log(&format!("{:?}", self.view_model.to_model_pos(false)));
                        //let req = self.view_model.backspace();
                        //self.ws.as_mut().unwrap().send(Json(&req));
                    },
                    "Enter" => {
                        self.content.insert('\n');
                        if self.auto_update {
                            let res = self.content.update_virtual_whitespace(self.window_width);
                            self.console.log(&res);
                        }
                        self.cursor = self.content.cursor_pos();
                        self.text = self.content.get_string();
                    },
                    x if x.len() == 1 => {
                        self.content.insert(x.chars().next().unwrap());
                        if self.auto_update {
                            let res = self.content.update_virtual_whitespace(self.window_width);
                            self.console.log(&res);
                        }
                        self.cursor = self.content.cursor_pos();
                        self.text = self.content.get_string();
                        //self.console.log(&format!("{:?}", self.view_model.to_model_pos(true)));
                        //let req = self.view_model.insert(x.to_string());
                        //self.ws.as_mut().unwrap().send(Json(&req));
                    },
                    _ => ()
                }
                self.console.log(&format!("{:?}", e.key()));
                // FIXME: implement
                
            },
            Msg::GotInput(s) => {
                //self.input = s;
            },
            Msg::ClearVirtualWhitespace => {
                self.content.clear_virtual_whitespace();
                self.cursor = self.content.cursor_pos();
                self.text = self.content.get_string();
            },
            Msg::Format => {
                let res = self.content.update_virtual_whitespace(self.window_width);
                self.console.log(&res);
                self.cursor = self.content.cursor_pos();
                self.text = self.content.get_string();
            },
            Msg::Ignore => {
                return false;
            },
            Msg::ToggleAutoUpdate => {
                self.auto_update = !self.auto_update;
            },
            Msg::UpdateWidth(n) => {
                self.window_width = n;
                let res = self.content.update_virtual_whitespace(self.window_width);
                self.console.log(&res);
                self.cursor = self.content.cursor_pos();
                self.text = self.content.get_string();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let col = self.cursor.col as f32 + if self.cursor.between { 0.5 } else { 0.0 };
        let x = (col * 10.0);
        let y = self.cursor.line as f32 * 19.0;
        let s = format!("background-color: grey; position: absolute; width: 2px; height: 19px; top: {}px; left: {}px;", y, x-1.0);
        html! {
            <div  >
                <nav class="menu",>
                    <button onclick=|_| Msg::ClearVirtualWhitespace,>{ "Clear virtual whitespace" }</button>
                    <button onclick=|_| Msg::Format,>{ "Update virtual whitespace" }</button>
                    <button onclick=|_| Msg::ToggleAutoUpdate,>{ if self.auto_update {"Auto update ON"} else {"Auto update OFF"} }</button>
                    <input oninput=|e| Msg::UpdateWidth(e.value.parse().unwrap()), type="range", min="40", max="150", value="100", class="slider", style="width:500px", />
                </nav>
                /*<textarea rows=5, style="width: 100%", 
                    oninput=|e| WsAction::SendData(e.value).into(),
                    placeholder="placeholder",>
                </textarea>
                <p></p>
                <textarea rows=10, placeholder="output goes here", readonly=true, style="width: 100%", >
                    { &self.output }
                </textarea>
                */
                <div style="width:80%; border: 1px solid black; padding: 10px;", onkeydown=|e| Msg::KeyEvt(e), tabindex="0", >
                    <div style="font-family: monospace; position: relative; font-size: 12pt;", >
                        <pre>{ self.text.clone() }</pre>
                        <div id="cursor", style=s, ></div>
                        <pre>{ format!("{}|", " ".repeat(self.window_width)) }</pre>
                    </div>
                </div>
                <span style="font-family: monospace; position: relative; font-size: 12pt;", > { self.window_width } </span>
                //{ format!("{:?}", self.view_model.pos) }
            </div>
        }
    }
}