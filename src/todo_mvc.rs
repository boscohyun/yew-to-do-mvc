use std::rc::Rc;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{
    function_component, html, use_node_ref, use_reducer, use_state, Callback, Html, Properties,
    Reducible,
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
            if let Some(input) = input_option {
                let input_value = input.value();
                input.set_value("");
                props.on_submit.emit(input_value);
            }
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

// TodoItem
#[derive(Properties, PartialEq, Clone)]
pub struct TodoItemProps {
    index: usize,
    message: String,
    completed: bool,
    onclick_checkbox: Callback<usize>,
    onclick_remove: Callback<usize>,
}

#[function_component(TodoItem)]
fn todo_item(props: &TodoItemProps) -> Html {
    let remove_button_enabled = use_state(|| false);
    let checked = props.completed.clone();
    let message = props.message.clone();

    let onclick_checkbox = {
        let index = props.index.clone();
        let onclick_checkbox = props.onclick_checkbox.clone();

        move |_| onclick_checkbox.emit(index)
    };

    let onclick_remove = {
        let index = props.index.clone();
        let onclick_remove = props.onclick_remove.clone();

        move |_| onclick_remove.emit(index)
    };

    let onmouseenter = {
        let remove_button_enabled = remove_button_enabled.clone();
        move |_| remove_button_enabled.set(true)
    };

    let onmouseleave = {
        let remove_button_enabled = remove_button_enabled.clone();
        move |_| remove_button_enabled.set(false)
    };

    let button_style = if *remove_button_enabled {
        String::from("")
    } else {
        String::from("display: none")
    };

    html! {
      <div {onmouseenter} {onmouseleave}>
        <input type="checkbox" onclick={onclick_checkbox} {checked} />
        <label>{ message }</label>
        <button style={ button_style } onclick={onclick_remove}>{ "x" }</button>
      </div>
    }
}
// ~TodoItem

// TodoMVC
#[derive(Properties, PartialEq, Default)]
pub struct TodoMVCProps {
    #[prop_or(Vec::<TodoItemProps>::new())]
    todo_list: Vec<TodoItemProps>,
}

#[derive(Clone)]
enum TodoMVCTab {
    All,
    Active,
    Completed,
}

struct TodoMVCState {
    tab: TodoMVCTab,
    todo_list: Vec<TodoItemProps>,
}

impl Default for TodoMVCState {
    fn default() -> TodoMVCState {
        Self {
            tab: TodoMVCTab::All,
            todo_list: Vec::<TodoItemProps>::new(),
        }
    }
}

enum TodoMVCAction {
    ChangeTab(TodoMVCTab),
    RegisterTodo(String),
    UnregisterTodo(usize),
    UnregisterCompletedTodoAll,
    ToggleCompleted(usize),
    ToggleCompletedAll,
}

impl Reducible for TodoMVCState {
    type Action = TodoMVCAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TodoMVCAction::ChangeTab(tab) => Self {
                tab,
                todo_list: self.todo_list.clone(),
            }
            .into(),
            TodoMVCAction::RegisterTodo(message) => {
                let mut todo_list = self.todo_list.clone();
                let index = todo_list.len();
                todo_list.push(TodoItemProps {
                    index,
                    message,
                    completed: false,
                    onclick_checkbox: Callback::default(),
                    onclick_remove: Callback::default(),
                });
                Self {
                    tab: self.tab.clone(),
                    todo_list,
                }
                .into()
            }
            TodoMVCAction::UnregisterTodo(index) => {
                let mut todo_list = self.todo_list.clone();
                if index >= todo_list.len() {
                    return self;
                }

                todo_list.remove(index);
                Self {
                    tab: self.tab.clone(),
                    todo_list,
                }
                .into()
            }
            TodoMVCAction::UnregisterCompletedTodoAll => {
                // let todo_list = self
                //     .todo_list
                //     .iter()
                //     .filter(|todo_item| !todo_item.completed)
                //     .collect::<Vec<TodoItemProps>>();

                Self {
                    tab: self.tab.clone(),
                    todo_list: vec![], //todo_list,
                }
                .into()
            }
            TodoMVCAction::ToggleCompleted(index) => {
                let mut todo_list = self.todo_list.clone();
                if index >= todo_list.len() {
                    return self;
                }

                let todo_item = &mut todo_list[index];
                todo_item.completed = !todo_item.completed;
                Self {
                    tab: self.tab.clone(),
                    todo_list,
                }
                .into()
            }
            TodoMVCAction::ToggleCompletedAll => {
                log::info!("onclick complete all");
                let todo_list = self.todo_list.clone();
                let already_completed_all: bool = todo_list.iter().all(|todo| todo.completed);
                let todo_list = todo_list
                    .iter()
                    .map(|todo_item| TodoItemProps {
                        message: todo_item.message.clone(),
                        completed: !already_completed_all,
                        onclick_checkbox: todo_item.onclick_checkbox.clone(),
                        onclick_remove: todo_item.onclick_remove.clone(),
                        ..(*todo_item)
                    })
                    .collect::<Vec<TodoItemProps>>();
                Self {
                    tab: self.tab.clone(),
                    todo_list,
                }
                .into()
            }
        }
    }
}

#[function_component(TodoMVC)]
pub fn todo_mvc(props: &TodoMVCProps) -> Html {
    let props = props.clone();
    let reducer = use_reducer(|| TodoMVCState {
        tab: TodoMVCTab::All,
        todo_list: props.todo_list.clone(),
    });
    let change_tab_all = {
        let reducer = reducer.clone();
        Callback::from(move |_| reducer.dispatch(TodoMVCAction::ChangeTab(TodoMVCTab::All)))
    };
    let change_tab_active = {
        let reducer = reducer.clone();
        Callback::from(move |_| reducer.dispatch(TodoMVCAction::ChangeTab(TodoMVCTab::Active)))
    };
    let change_tab_completed = {
        let reducer = reducer.clone();
        Callback::from(move |_| reducer.dispatch(TodoMVCAction::ChangeTab(TodoMVCTab::Completed)))
    };
    let register_todo = {
        let reducer = reducer.clone();
        Callback::from(move |message| reducer.dispatch(TodoMVCAction::RegisterTodo(message)))
    };
    let unregister_todo = {
        let reducer = reducer.clone();
        Callback::from(move |index| reducer.dispatch(TodoMVCAction::UnregisterTodo(index)))
    };
    let unregister_completed_todo_all = {
        let reducer = reducer.clone();
        Callback::from(move |_| reducer.dispatch(TodoMVCAction::UnregisterCompletedTodoAll))
    };
    let toggle_completed = {
        let reducer = reducer.clone();
        Callback::from(move |index| reducer.dispatch(TodoMVCAction::ToggleCompleted(index)))
    };
    let toggle_completed_all = {
        let reducer = reducer.clone();
        Callback::from(move |_| reducer.dispatch(TodoMVCAction::ToggleCompletedAll))
    };

    let has_todo_item = reducer.todo_list.len() > 0;
    let complete_all_button_html = if has_todo_item {
        html! {
          <button
            style="margin-right:2px;"
            onclick={toggle_completed_all}>
            {"complete all"}
          </button>
        }
    } else {
        html! {}
    };

    let filtered_todo_list = reducer
        .todo_list
        .iter()
        .filter(|todo_item| match reducer.tab {
            TodoMVCTab::All => true,
            TodoMVCTab::Active => !todo_item.completed,
            TodoMVCTab::Completed => todo_item.completed,
        });

    html! {
      <div>
        <h1>{"todos"}</h1>
        <div>
          {complete_all_button_html}
          <InputTodo on_submit={ register_todo }/>
        </div>
        {
            filtered_todo_list.map(|todo_item| {
              let toggle_completed = toggle_completed.clone();
              let unregister_todo = unregister_todo.clone();
              html! {
                <TodoItem
                  key={ todo_item.index }
                  onclick_checkbox={ toggle_completed }
                  onclick_remove={ unregister_todo }
                  ..todo_item.clone() />
              }
            }).collect::<Html>()
        }
        <div>
          <label>{ format!("{} items left", reducer.todo_list.iter().filter(|todo_item| !todo_item.completed).count()) }</label>
          <button onclick={ change_tab_all }>{ "All" }</button>
          <button onclick={ change_tab_active }>{ "Active" }</button>
          <button onclick={ change_tab_completed }>{ "Completed" }</button>
          <button onclick={ unregister_completed_todo_all }>{ "Clear completed" }</button>
        </div>
        <div>
          {"Double-click to edit a todo"}<br />
          {"Created by Hyun Seungmin"}<br />
          {"Part of TodoMVC"}<br />
        </div>
      </div>
    }
}
// ~TodoMVC
