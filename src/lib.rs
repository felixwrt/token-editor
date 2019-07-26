use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::event::IKeyboardEvent;
use stdweb::web::event::IEvent;

mod content;
use content::{Content, CursorPos, GetString};

pub struct Model {
    console: ConsoleService,
    text: String,
    cursor: CursorPos,
    content: Content,
    auto_update: bool,
    window_width: usize
}

pub enum Msg {
    KeyEvt(KeyDownEvent),
    ClearVirtualWhitespace,
    Format,
    ToggleAutoUpdate,
    UpdateWidth(usize)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        //let typed = "fn test(&self,  other:\n  \n&mut usize){let x=(self+1)*other;\n return1<y}";
        let typed = "fn test(other:&mut usize){let array=[1123456, 531432124, 43241432, 4312432, 9432, 432,4328,432];let x=(self+1)*other;return 1<y}";
        let _visible = "fn test(other: &mut usize) {\n    let x = (self + 1) * other;\n    return 1 < y\n}";
        let typed = "hello";
        let content = Content::from_strings(&typed, &typed);
        Model {
            console: ConsoleService::new(),
            text: content.get_string(),
            cursor: content.cursor_pos(),
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
                        self.content.cursor_left();
                        self.cursor = self.content.cursor_pos();
                    },
                    "ArrowRight" => {
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
                    },
                    _ => ()
                }
                self.console.log(&format!("{:?}", e.key()));
                // FIXME: implement
                
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
        let x = col * 10.0;
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
                <div style="width:80%; border: 1px solid black; padding: 10px;", onkeydown=|e| Msg::KeyEvt(e), tabindex="0", >
                    <div style="font-family: monospace; position: relative; font-size: 12pt;", >
                        <pre>{ self.text.clone() }</pre>
                        <div id="cursor", style=s, ></div>
                        <pre>{ format!("{}|", " ".repeat(self.window_width)) }</pre>
                    </div>
                </div>
                <span style="font-family: monospace; position: relative; font-size: 12pt;", > { self.window_width } </span>
            </div>
        }
    }
}