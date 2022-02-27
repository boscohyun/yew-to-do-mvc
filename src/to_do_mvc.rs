use wasm_bindgen::JsCast;
use web_sys::{
    window, Document, EventTarget, HtmlInputElement, InputEvent, KeyboardEvent, MouseEvent,
};
use yew::{html, Callback, Component, Context, Html};

pub enum Msg {
    InputTodoValue(String),
    KeyDown(String),
    OnClickCompleteAll,
}

pub struct Todo {
    message: String,
    completed: bool,
}

// pub struct TodoList {
//     list: Vec<Todo>,
// }
//
// #[derive(Copy, Clone)]
// struct TodoListWrapper<'a> {
//     todo_list_ref: &'a TodoList,
// }

pub struct Model {
    input: String,
    list: Vec<Todo>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::new(),
            list: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputTodoValue(value) => {
                log::info!("input: {}", value);
                if value == self.input {
                    return false;
                }

                self.input = value;
                return true;
            }
            Msg::KeyDown(key) => {
                log::info!("key: {}", key);
                if key != "Enter" {
                    return false;
                }

                if self.input.is_empty() {
                    return false;
                }

                self.list.push(Todo {
                    message: self.input.to_string(),
                    completed: false,
                });
                self.input = String::new();

                /*
                 * ref #1: https://stackoverflow.com/a/61635949
                 *
                 * use wasm_bindgen::JsCast;
                 *
                 * let window = web_sys::window().unwrap();
                 * let document = window.document().unwrap();
                 * let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
                 * let cookie = html_document.cookie().unwrap();
                 *
                 * ref #2: https://yew.rs/docs/0.18.0/concepts/wasm-bindgen/web-sys#the-node-in-noderef
                 */
                if let Some(input_todo_element) = window()
                    .and_then(|some| some.document())
                    .and_then(|some| Document::get_element_by_id(&some, "input_todo_element"))
                    .and_then(|some| some.dyn_into::<HtmlInputElement>().ok())
                {
                    input_todo_element.set_value("");
                }

                return true;
            }
            Msg::OnClickCompleteAll => {
                let completed_all: bool = self.list.iter().all(|todo| todo.completed);
                if completed_all {
                    // for mut todo in self.list {
                    //     todo.completed = false;
                    // }
                    self.list = self
                        .list
                        .iter()
                        .map(|todo| Todo {
                            message: todo.message.to_string(),
                            completed: false,
                        })
                        .collect::<Vec<Todo>>();
                } else {
                    // for mut todo in self.list {
                    //     todo.completed = true;
                    // }
                    self.list = self
                        .list
                        .iter()
                        .map(|todo| Todo {
                            message: todo.message.to_string(),
                            completed: true,
                        })
                        .collect::<Vec<Todo>>();
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let ctx_link = ctx.link();

        let onclick_complete_all: Callback<MouseEvent> =
            ctx_link.callback(|_| Msg::OnClickCompleteAll);

        let oninput_todo = ctx_link.batch_callback(|e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::InputTodoValue(input.value()))
        });

        let onkeydown_todo = ctx_link.callback(|e: KeyboardEvent| {
            // e.prevent_default();
            let key = e.key();
            Msg::KeyDown(key)
        });

        let header_style = r#"
          color: rgba(175, 47, 47, 0.15);
          font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
          font-size: 50px;
          font-weight: 100;
          margin: 0px;
        "#;

        let has_todo = self.list.len() > 0;
        let complete_all_button_html = if has_todo {
            html! {
              <button
                style="margin-right:2px;"
                onclick={Callback::from(onclick_complete_all)}>
                {"complete all"}
              </button>
            }
        } else {
            html! {}
        };
        let list_to_html = if has_todo {
            let list = self
                .list
                .iter()
                .map(|todo| {
                    html! {
                      <div>
                        <input type="checkbox" />
                        <label>{ todo.message.to_string() }</label>
                      </div>
                    }
                })
                .collect::<Html>();
            html! {
              <ol>{list}</ol>
            }
        } else {
            html! {}
        };

        html! {
            <>
              <h1 style={header_style}>{"todos"}</h1>
              <div>
                {complete_all_button_html}
                <input
                  id="input_todo_element"
                  type="text"
                  placeholder="What need to be done?"
                  oninput={Callback::from(oninput_todo)}
                  onkeydown={Callback::from(onkeydown_todo)} />
              </div>
              {list_to_html}
              <div>
                {"Double-click to edit a todo"}<br />
                {"Created by Hyun Seungmin"}<br />
                {"Part of TodoMVC"}<br />
              </div>
            </>
        }
    }
}
