use leptos::prelude::*;

/// Conditional view for Signal
#[component]
pub fn CondSig(
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

/// Conditional view for Closure
#[component]
pub fn CondSure(
    cond: impl Fn() -> bool + Send + Sync + 'static,
    children: ChildrenFn
) -> impl IntoView {
    move || {
        if cond {
            Some(view! { {children()} })
        } else {
            None
        }
    }
}