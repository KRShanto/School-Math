pub mod app;
pub mod data;
pub mod find;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
