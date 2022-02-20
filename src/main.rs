use wasm_logger;
mod to_do_mvc;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<to_do_mvc::Model>();
}
