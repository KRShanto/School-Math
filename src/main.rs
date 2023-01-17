pub mod components;
pub mod data;
pub mod find;

fn main() {
    yew::Renderer::<components::app::App>::new().render();
}
