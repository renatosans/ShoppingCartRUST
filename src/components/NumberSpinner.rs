use leptos::{web_sys::HtmlInputElement, *};


#[component]
pub fn NumberSpinner(cx: Scope) -> Element {
    let (counter, set_counter) = create_signal(cx, 0);

    let increase = move |_| set_counter.update(|counter| *counter += 1);
    let decrease = move |_| set_counter.update(|counter| *counter -= 1);

    view! {
        cx,
        <div style="display: flex; flex-direction: row;">
          <div id="decrease">
            <button style="border: 0px; width: 25px; height: 25px; background-image: url('./public/icons/circle_minus.svg')" on:click=decrease />
          </div>
          <div id="item_count" style="width: 40px; height: 30px; line-height: 30px; text-align: center;">
            <span style="font-size: 20px;">{counter}</span>
          </div>
          <div id="increase">
            <button style="border: 0px; width: 25px; height: 25px; background-image: url('./public/icons/circle_plus.svg')" on:click=increase />
          </div>
        </div>
    }
}
