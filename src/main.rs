use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]

fn app() -> Html{
    // useState hook with a closure to initialize it with 0
    let state = use_state(|| Model {
        value: 0
    });

    let on_click = {
        // the original state variable will be used by yew so we need to clone it to take ownership
        let state = state.clone();

        Callback::from(move |_| {
            state.set(
                Model {
                    value: state.value + 1
                }
            )
        })
    };

    html! {
        <>
            <div>
                <button onclick={&on_click} >{"+1"}</button>
                <p>{ state.value }</p>
            </div>
            <div>
                <button onclick={&on_click} >{"+1"}</button>
                <p>{ state.value }</p>
            </div>
        </>
    }


}

fn main() {
    yew::start_app::<App>();

}