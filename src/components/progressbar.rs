use leptos::prelude::*;
#[component]
pub fn ProgressBar(progress: ReadSignal<i32>, #[prop(default = 100)]max:u16) -> impl IntoView {
    
    
    view! {
        <progress
            max=max
            value=progress
        />
    }
}