use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::event::IKeyboardEvent;
use stdweb::web::event::IEvent;

mod content;
use content::{Content, GetString};

const text_size: usize = 12;

pub struct Model {
    console: ConsoleService,
    text: String,
    //cursor: CursorPos,
    cursor2: ((usize, usize), (usize, usize)),
    cursor_small: (usize, usize),
    content: Content,
    auto_update: bool,
    window_width: usize,
    char_dimensions: (f32, f32)
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

        let mut console = ConsoleService::new();

        use stdweb::web::IElement;
        use stdweb::web::INode;
        use stdweb::unstable::TryFrom;
        use stdweb::web::IHtmlElement;
        let mut elmt = stdweb::web::document().create_element("span").unwrap();
        let text = stdweb::web::document().create_text_node("x");
        elmt.append_child(&text);
        elmt.set_attribute("style", &format!("font-family: monospace; position: absolute; top: -1000px; left: -1000px; font-size: {}pt;", text_size));
        stdweb::web::document().body().unwrap().append_child(&elmt);
        let rect = stdweb::web::HtmlElement::try_from(elmt).unwrap().get_bounding_client_rect();
        console.log(&format!("{}, {}", rect.get_width(), rect.get_height()));


        Model {
            console: console,
            text: content.get_string(),
            //cursor: content.cursor_pos(),
            cursor2: ((0, 11), (2, 4)),
            cursor_small: (0, 0),
            content,
            auto_update: false,
            window_width: 100,
            char_dimensions: (rect.get_width() as f32, rect.get_height() as f32)
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
        let (w, h) = self.char_dimensions;

        let x = (self.cursor2.0).1 as f32 * w;
        let y = (self.cursor2.0).0 as f32 * h;
        let s = format!("background-color: #7799bb; position: absolute; width: 2px; height: {}px; top: {}px; left: {}px; display: {};", h, y, x as i32 - 1, if self.cursor2.0 == self.cursor2.1 { "block" } else { "None"});
        let s_small = format!("background-color: #7799bb; position: absolute; width: 2px; height: {}px; top: {}px; left: {}px; display: {};", h, h*self.cursor_small.0 as f32, w * self.cursor_small.1 as f32 - 1.0, if (self.cursor2.0).0 != (self.cursor2.1).0 { "block" } else { "None"});
        
        // cursor
        let width_first_line = w * if (self.cursor2.0).0 == (self.cursor2.1).0 { 
            (self.cursor2.1).1 - (self.cursor2.0).1 
        } else {
            self.window_width - (self.cursor2.0).1
        } as f32;
        let first_line_style = format!("top: {}px; left: {}px; width: {}px; height: {}px;", h*(self.cursor2.0).0 as f32, w*(self.cursor2.0).1 as f32, width_first_line, h);
        let num_mid_lines = ((self.cursor2.1).0 - (self.cursor2.0).0).checked_sub(1).unwrap_or(0);
        let mid_lines_style = format!("top: {}px; left: 0px; width: {}px; height: {}px;", h*((self.cursor2.0).0 + 1) as f32, w * self.window_width as f32, h*num_mid_lines as f32);
        let last_line_width = if (self.cursor2.0).0 == (self.cursor2.1).0 { 
            0
        }else{
            (self.cursor2.1).1
        };
        let last_line_style = format!("top: {}px; left: 0px; width: {}px; height: {}px;", h*(self.cursor2.1).0 as f32, w * last_line_width as f32, h);
        let div_style = format!("font-family: monospace; position: relative; font-size: {}pt;", text_size);

        html! {
            <div  >
                <nav class="menu",>
                    <button onclick=|_| Msg::ClearVirtualWhitespace,>{ "Clear virtual whitespace" }</button>
                    <button onclick=|_| Msg::Format,>{ "Update virtual whitespace" }</button>
                    <button onclick=|_| Msg::ToggleAutoUpdate,>{ if self.auto_update {"Auto update ON"} else {"Auto update OFF"} }</button>
                    <input oninput=|e| Msg::UpdateWidth(e.value.parse().unwrap()), type="range", min="40", max="150", value="100", class="slider", style="width:500px", />
                </nav>
                <div style="width:80%; border: 1px solid grey; padding: 10px;", onkeydown=|e| Msg::KeyEvt(e), tabindex="0", >
                    <div style=div_style, >
                        <pre>{ self.text.clone() }</pre>
                        <div id="cursor", style=s, ></div>
                        <div class="area", style=first_line_style, ></div>
                        <div class="area", style=mid_lines_style, ></div>
                        <div class="area", style=last_line_style, ></div>
                        <div id="cursor_small", style=s_small, ></div>
                        <pre>{ format!("{}|", " ".repeat(self.window_width)) }</pre>
                    </div>
                </div>
            </div>
        }
    }
}