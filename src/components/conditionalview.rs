use leptos::prelude::*;

#[component]
pub fn Cond(
    #[prop(into)] cond: Signal<bool>,
    children: Children
) -> impl IntoView {
    if cond.get() {
        Some(view! { {children()} })
    } else {
        None
    }
}