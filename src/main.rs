use dioxus::prelude::*;

mod components {
    pub mod alternate_maze_state;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    log::info!("app start");
    cx.render(rsx! {
        h1 {
            class: "m-8 text-4xl font-light",
            "交互着手数字集め迷路"
        }
        div {
            class: "flex justify-center p-2 mt-5",
            components::alternate_maze_state::alternate_maze_state{}
        }
    })
}
