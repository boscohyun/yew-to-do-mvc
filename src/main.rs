use wasm_logger;
mod to_do_mvc;
mod todo_mvc;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<todo_mvc::TodoMVC>();
}
