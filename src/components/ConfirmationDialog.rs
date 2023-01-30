use leptos::{web_sys::HtmlInputElement, *};


#[component]
pub fn ConfirmationDialog(cx: Scope, message: String) -> Element {
    let (open, set_open) = create_signal(cx, true);

    let confirm = move |_| set_open.update(|_| ());
    let abort = move |_| set_open.update(|_| ());

    view! {
        cx,
        <div style="border: 1px; border-color: blue; border-style: solid; border-radius: 0.4rem;">
        <h2>"Informação"</h2>
        <p>{message}</p>
        <button style="background-color: royalblue; border-radius: 0.4rem; padding: 0 1rem 0 1rem;" on:click=confirm >"Sim"</button>
        <button style="background-color: royalblue; border-radius: 0.4rem; padding: 0 1rem 0 1rem;" on:click=abort   >"Não"</button>
        </div>
    }
}
