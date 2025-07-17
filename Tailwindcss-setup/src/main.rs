use yew::prelude::*;

struct Model {
    value: i64,
}

enum Action {
    Increment,
    Decrement,
}

#[function_component(App)]

fn app() -> Html {
    // useState hook with a closure to initialize it with 0
    let state = use_state(|| Model { value: 0 });

    let on_click = {
        // the original state variable will be used by yew so we need to clone it to take ownership
        let state = state.clone();

        Callback::from(move |action: Action| {

            let delta = match action {
                Action::Increment => 1,
                Action::Decrement => -1,
            };

            state.set(Model {
                value: state.value + delta,
            })
        })
    };

    html! {
        <div class="flex items-center -space-x-4 justify-center p-10 text-white">
            <button class="bg-gray-900 w-24 h-24 rounded-full z-10" onclick={&on_click.reform(|_| Action::Increment)} >{"Increment"}</button>
            <div class="bg-gray-400 w-16 h-16 rounded-full flex items-center justify-center">
                { state.value }
            </div>
            <button class="bg-gray-900 w-24 h-24 rounded-full z-10" onclick={&on_click.reform(|_| Action::Decrement)} >{"Decrement"}</button>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
