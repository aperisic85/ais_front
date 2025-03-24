use leptos::prelude::*;
mod app;
use app::App;
fn main() {
    leptos::mount::mount_to_body(|| view! { <App></App> });
}