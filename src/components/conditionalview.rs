use leptos::prelude::*;

#[component]
pub fn Cond(
    #[prop(into)] cond: Signal<bool>,
    children: ChildrenFn
) -> impl IntoView {
    move || {
        if cond.get() {
            Some(view! { {children()} })
        } else {
            None
        }
    }
}