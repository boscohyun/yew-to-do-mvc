use yew::{function_component, html};

#[function_component(TodoComponent)]
pub fn todo_component() -> Html {
    html! {
      <div>{ "todo" }</div>
    }
}
