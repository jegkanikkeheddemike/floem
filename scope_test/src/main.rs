use floem::{
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
        button(|| "switch").on_click_stop(move |_| set_view.update(|v| *v = !*v)),
    )
        .style(|s| s.size_full().justify_center().items_center())
}

fn view1() -> impl IntoView {
    let signal_number = create_rw_signal(0i32);

    signal_number.on_drop(move || println!("Signal dropped at value {}", signal_number.get()));

    button(move || format!("View1 = {}", signal_number.get()))
        .on_click_stop(move |_| signal_number.update(|v| *v += 1))
}

fn view2() -> impl IntoView {
    "View2"
}
