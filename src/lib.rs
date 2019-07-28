use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::event::IKeyboardEvent;
use stdweb::web::event::IEvent;

mod content;
use content::{Content, GetString};

pub struct Model {
    console: ConsoleService,
    text: String,
    //cursor: CursorPos,
    cursor2: ((usize, usize), (usize, usize)),
    cursor_small: (usize, usize),
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

impl Model {
    fn update_cursor(&mut self) {
        let (cursor2, cursor_small) = self.content.cursor_pos_2();
        self.cursor2 = cursor2;
        self.cursor_small = cursor_small;
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        //let typed = "fn test(&self,  other:\n  \n&mut usize){let x=(self+1)*other;\n return1<y}";
        //let typed = "fn test(other:&mut usize){let array=[1123456, 531432124, 43241432, 4312432, 9432, 432,4328,432];let x=(self+1)*other;return 1<y}";
        let _visible = "fn test(other: &mut usize) {\n    let x = (self + 1) * other;\n    return 1 < y\n}";
        let typed = "fn test() {\n\n    let x = 1;\n}";
        let content = Content::from_strings(&typed, &typed);
        Model {
            console: ConsoleService::new(),
            text: content.get_string(),
            //cursor: content.cursor_pos(),
            cursor2: ((0, 11), (2, 4)),
            cursor_small: (0, 0),
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
                        self.update_cursor();
                    },
                    "ArrowRight" => {
                        self.content.cursor_right();
                        self.update_cursor();
                    },
                    "ArrowDown" => {
                        self.content.cursor_down();
                        self.update_cursor();
                    },
                    "ArrowUp" => {
                        self.content.cursor_up();
                        self.update_cursor();
                    },
                    "End" => {
                        self.content.cursor_end();
                        self.update_cursor();
                    },
                    "Home" => {
                        self.content.cursor_home();
                        self.update_cursor();
                    },
                    "Backspace" => {
                        self.content.backspace();
                        if self.auto_update {
                            let res = self.content.update_virtual_whitespace(self.window_width);
                            self.console.log(&res);
                        }
                        self.update_cursor();
                        self.text = self.content.get_string();
                    },
                    "Delete" => {
                        self.content.delete();
                        if self.auto_update {
                            let res = self.content.update_virtual_whitespace(self.window_width);
                            self.console.log(&res);
                        }
                        self.update_cursor();
                        self.text = self.content.get_string();
                    },
                    "Enter" => {
                        self.content.insert('\n');
                        if self.auto_update {
                            let res = self.content.update_virtual_whitespace(self.window_width);
                            self.console.log(&res);
                        }
                        self.update_cursor();
                        self.text = self.content.get_string();
                    },
                    x if x.len() == 1 => {
                        self.content.insert(x.chars().next().unwrap());
                        if self.auto_update {
                            let res = self.content.update_virtual_whitespace(self.window_width);
                            self.console.log(&res);
                        }
                        self.update_cursor();
                        self.text = self.content.get_string();
                    },
                    _ => ()
                }
                self.console.log(&format!("{:?}", e.key()));
                // FIXME: implement
                
            },
            Msg::ClearVirtualWhitespace => {
                self.content.clear_virtual_whitespace();
                self.update_cursor();
                self.text = self.content.get_string();
            },
            Msg::Format => {
                let res = self.content.update_virtual_whitespace(self.window_width);
                self.console.log(&res);
                self.update_cursor();
                self.text = self.content.get_string();
            },
            Msg::ToggleAutoUpdate => {
                self.auto_update = !self.auto_update;
            },
            Msg::UpdateWidth(n) => {
                self.window_width = n;
                let res = self.content.update_virtual_whitespace(self.window_width);
                self.console.log(&res);
                self.update_cursor();
                self.text = self.content.get_string();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let col = (self.cursor2.0).1 as f32;
        let x = col * 10.0;
        let y = (self.cursor2.0).0 as f32 * 19.0;
        let s = format!("background-color: #7799bb; position: absolute; width: 2px; height: 19px; top: {}px; left: {}px; display: {};", y, x-1.0, if self.cursor2.0 == self.cursor2.1 { "block" } else { "None"});
        let s_small = format!("background-color: #7799bb; position: absolute; width: 2px; height: 19px; top: {}px; left: {}px; display: {};", 19*self.cursor_small.0, (10*self.cursor_small.1)as isize - 1, if (self.cursor2.0).0 != (self.cursor2.1).0 { "block" } else { "None"});
        
        // cursor
        let width_first_line = 10 * if (self.cursor2.0).0 == (self.cursor2.1).0 { 
            (self.cursor2.1).1 - (self.cursor2.0).1 
        } else {
            self.window_width - (self.cursor2.0).1
        };
        let first_line_style = format!("top: {}px; left: {}px; width: {}px;", 19*(self.cursor2.0).0, 10*(self.cursor2.0).1, width_first_line);
        let num_mid_lines = ((self.cursor2.1).0 - (self.cursor2.0).0).checked_sub(1).unwrap_or(0);
        let mid_lines_style = format!("top: {}px; left: 0px; width: {}px; height: {}px;", 19*((self.cursor2.0).0 + 1), 10 * self.window_width, 19*num_mid_lines);
        let last_line_width = if (self.cursor2.0).0 == (self.cursor2.1).0 { 
            0
        }else{
            (self.cursor2.1).1
        };
        let last_line_style = format!("top: {}px; left: 0px; width: {}px;", 19*(self.cursor2.1).0, 10 * last_line_width);
        
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
                        <div class="area", style=first_line_style, ></div>
                        <div class="area", style=mid_lines_style, ></div>
                        <div class="area", style=last_line_style, ></div>
                        <div id="cursor_small", style=s_small, ></div>
                        <pre>{ format!("{}|", " ".repeat(self.window_width)) }</pre>
                    </div>
                </div>
                <span style="font-family: monospace; position: relative; font-size: 12pt;", > { self.window_width } </span>
            </div>
        }
    }
}