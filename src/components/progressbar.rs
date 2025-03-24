use leptos::prelude::*;
#[component]
pub fn ProgressBar(progress: ReadSignal<i32>) -> impl IntoView {
    
    
    view! {
        <progress
            max="100"
            value=progress
        />
    }
}