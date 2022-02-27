use std::ops::Deref;

use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{
    function_component, html, use_effect, use_effect_with_deps, use_mut_ref, use_node_ref,
    use_state, use_state_eq, Callback, Html, Properties,
};

// InputTodo
#[derive(Properties, PartialEq, Clone)]
struct InputTodoProps {
    on_submit: Callback<String>,
}

#[function_component(InputTodo)]
fn input_todo(props: &InputTodoProps) -> Html {
    let input_node_ref = use_node_ref();

    let onkeydown = {
        let props = props.clone();
        let input_node_ref = input_node_ref.clone();

        move |e: KeyboardEvent| {
            let key = e.key();
            log::info!("key: {}", key);
            if key != "Enter" {
                return;
            }

            let input_option = input_node_ref.cast::<HtmlInputElement>();
            if input_option.is_none() {
                return;
            }

            let input = input_option.unwrap();
            let input_value = input.value();
            props.on_submit.emit(input_value);
        }
    };

    html! {
        <input
            ref={input_node_ref}
            type="text"
            placeholder="What need to be done?"
            {onkeydown} />
    }
}
// ~InputTodo

// Todo
#[derive(Properties, PartialEq, Clone)]
pub struct TodoProps {
    message: String,
    completed: bool,
    // onclick_checkbox
}

#[function_component(Todo)]
fn todo(props: &TodoProps) -> Html {
    html! {
      <div>
        <input type="checkbox" />
        <label>{ &props.message }</label>
      </div>
    }
}
// ~Todo

// TodoMVC
#[derive(Properties, PartialEq, Default)]
pub struct TodoMVCProps {
    #[prop_or(Vec::<TodoProps>::new())]
    todo_list: Vec<TodoProps>,
}

#[function_component(TodoMVC)]
pub fn todo_mvc(props: &TodoMVCProps) -> Html {
    let todo_list_state = use_state_eq(|| props.todo_list.clone());
    let on_input_todo_submitted = {
        let todo_list_state = todo_list_state.clone();

        Callback::from(move |todo_message: String| {
            let todo_list_state = todo_list_state.clone();
            let mut todo_list = todo_list_state.deref().clone();
            todo_list.push(TodoProps {
                message: todo_message,
                completed: false,
            });
            todo_list_state.set(todo_list);
        })
    };

    let has_todo = todo_list_state.len() > 0;
    let complete_all_button_html = if has_todo {
        html! {
          <button
            style="margin-right:2px;"
            // onclick={Callback::from(onclick_complete_all)}>
          >
            {"complete all"}
          </button>
        }
    } else {
        html! {}
    };

    html! {
      <div>
        <h1>{"todos"}</h1>
        <div>
                {complete_all_button_html}
                <InputTodo on_submit={ on_input_todo_submitted }/>
              </div>
              <ol style={ "list-style-type: none;" }>
              {
                  todo_list_state.iter().map(|todo_props| html! {
                      <li>
                        <input type="checkbox" />
                        <label>{ todo_props.message.to_string() }</label>
                      </li>
                  }).collect::<Html>()
                }
              </ol>
              <div>
                {"Double-click to edit a todo"}<br />
                {"Created by Hyun Seungmin"}<br />
                {"Part of TodoMVC"}<br />
              </div>
      </div>
    }
}
// ~TodoMVC
