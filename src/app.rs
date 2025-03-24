use leptos::prelude::*;
use crate::components::progressbar::ProgressBar;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    view! {
        <button
            on:click=move |_| {* set_count.write() += 1}
            //class:red = move || count.get() % 2 == 0 //change color on odd
            class=(["red",], move || count.get() % 2 == 1)
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {double_count}
        </p>
        
        
        <ProgressBar progress=count/>
    }
}