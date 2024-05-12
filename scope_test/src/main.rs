use floem::{
    event::EventPropagation,
    reactive::{create_rw_signal, create_signal},
    views::{button, dyn_container, Decorators},
    IntoView,
};

fn main() {
    floem::launch(app_view);
}

fn app_view() -> impl IntoView {
    let (view, set_view) = create_signal(true);

    (
        dyn_container(move || {
            if view.get() {
                view1().into_any()
            } else {
                view2().into_any()
            }
        }),
        button(|| "switch").on_click(move |_| {
            set_view.update(|v| *v = !*v);
            EventPropagation::Stop
        }),
    )
        .style(|s| s.size_full().justify_center().items_center())
}

fn view1() -> impl IntoView {
    let signal = create_rw_signal(0i32);

    signal.on_cleanup(move || println!("Signal cleanup"));

    button(move || format!("View1 = {}", signal.get())).on_cleanup(|| println!("Button cleanup"))
}

fn view2() -> impl IntoView {
    "View2"
}
