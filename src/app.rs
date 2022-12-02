use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct NewTaskArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let task_content = use_ref(|| NodeRef::default());

    let name = use_state(|| String::new());

    let task_msg = use_state(|| String::new());
    {
        let task_msg = task_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke(
                        "create_task",
                        to_value(&NewTaskArgs { name: &*name }).unwrap(),
                    )
                    .await;
                    log(&new_msg.as_string().unwrap());
                    task_msg.set(new_msg.as_string().unwrap());
                });

                || {}
            },
            name2,
        );
    }

    let create_task = {
        let name = name.clone();
        let task_content = task_content.clone();
        Callback::from(move |_| {
            name.set(task_content.cast::<web_sys::HtmlInputElement>().unwrap().value());
        })
    };

    html! {
        <main class="container">

            <div class="row">
                <input id="task-input" ref={&*task_content} placeholder="Enter a new task..." />
                <button type="button" onclick={create_task}>{"Create Task"}</button>
            </div>

            <p><b>{ &*task_msg }</b></p>
        </main>
    }
}
