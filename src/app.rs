use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| {* set_count.write() += 1}
            //class:red = move || count.get() % 2 == 0 //change color on odd
            class=(["button-40", "rounded"], move || count.get() % 2 == 1)
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
        <button
            on:click=move |_| {* set_count.write() += 10}
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", move || count.get().to_string())
            >
            "Click me: "
            {count} 
        </button>
    }
}