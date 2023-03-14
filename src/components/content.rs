use wasm_bindgen::prelude::wasm_bindgen;
use yew::{
    function_component, html, use_effect_with_deps, use_state, AnimationEvent, Callback, Html,
    Properties, UseStateHandle,
};
use yew_router::{
    prelude::{use_route, Switch},
    Routable,
};

use crate::router::{switch, Route};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    pub activeroute: UseStateHandle<String>,
}

#[function_component(Content)]
pub fn content(props: &ContentProps) -> Html {
    let route = use_route::<Route>().unwrap();

    let display_location = use_state(|| route.clone());
    let transition_stage = use_state(|| "fadeIn".to_string());

    let page_change = {
        let display_location = display_location.clone();
        let transition_stage = transition_stage.clone();
        let route = route.clone();
        Callback::from(move |_e: AnimationEvent| {
            if *transition_stage == "fadeOut".to_string() {
                transition_stage.set("fadeIn".to_string());
                display_location.set(route.clone());
            }
        })
    };

    {
        let route = route.clone();
        let route2 = route.clone();
        let activeroute = props.activeroute.clone();
        let transition_stage = transition_stage.clone();
        let display_location = display_location.clone();

        use_effect_with_deps(
            move |_| {
                // log(&route.to_path()); // imprime en consola la ruta actual
                activeroute.set(route.to_path());

                if route != *display_location {
                    transition_stage.set("fadeOut".to_string())
                }

                || {}
            },
            route2,
        )
    }

    html! {
        <div class={format!("pt-16 {}", *transition_stage)} onanimationend={page_change}>
          <Switch<Route> render={move |_| switch(&*display_location)} />
          // En un caso general se podría aplicar:
          // <Switch<Route> render={switch} />
          // o usando un referencia en la función switch() definida en router.rs:
          // <Switch<Route> render={|r: Route| switch(&r)} />
          // VER la documentación de Yew:
          // https://yew.rs/docs/concepts/router
        </div>
    }
}
