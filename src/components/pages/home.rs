use yew::{function_component, html, Callback, Html};
use yew_router::prelude::use_navigator;

use crate::components::projects::Projects;
use crate::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let on_click_go_cv_button = {
        let navigator = navigator.clone();

        Callback::from(move |_| navigator.push(&Route::MyCV))
    };

    html! {
        <>
          <section>
            <div class="my-2 flex flex-col justify-center md:flex-row md:items-center">
              <div class="p-12 md:w-[500px]">
                <h1 class="text-sky-300 text-2xl font-bold tracking-widest leading-10">{"Hola, Soy"}</h1>

                <p class="py-2 text-3xl font-bold text-slate-300">
                  {"Enrique Marín"}
                </p>
                <p class="text-3xl font-bold text-slate-600">
                  {"Desarrollo Aplicaciones WEB."}
                </p>
                <p class="my-5 text-lg">
                  {"Soy desarrollador de software que se especializa en diseñar y desarrollar
                  aplicaciones WEB excepcionales y otras tecnologías (Android, Flutter, Desktop apps…)."}
                </p>

                <button onclick={on_click_go_cv_button}
                  class="bg-gradient-to-tl from-cyan-400 via-sky-600 to-fuchsia-500 rounded-lg flex text-lg p-1 duration-300 hover:scale-90">
                  <span class="bg-neutral-900 py-4 px-6 rounded-md duration-300 hover:bg-transparent">
                    {"Conoce Mi CV"}
                    <span class="ml-6 text-2xl font-black text-amber-700">{"▷"}</span>
                  </span>
                </button>
              </div>

              <div class="rounded-full w-64 mx-auto shadow-md shadow-sky-800 md:w-80 md:rounded-2xl">
                <img class="inline-block rounded-full w-64 md:w-80 md:rounded-2xl" src="img/profile.jpeg" alt="Profile image" />
              </div>
            </div>

            <div class="px-12 my-12 mx-auto md:px-20 md:max-w-screen-md lg:max-w-[920px]">
              <hr class="mb-3 border-sky-600" />
                <h2 class="text-sky-600 text-xl px-4 font-bold tracking-widest leading-10">{"¡¡¡Saludos!!!"}</h2>
                <p class="text-sky-600 text-base px-4">
                  {"Te presento mi landing page para que sepas algo sobre mí. Cualquier duda que se te ofrezca \
                  te la resolveré si te pones en contacto conmigo através de email o teléfono. ¡Te espero!"}
                </p>
              <hr class="mt-3 border-sky-600" />
            </div>
          </section>

          <section>
            <Projects />
          </section>
        </>
    }
}

/*
 * DISEÑO DEL BOTÓN. VER:
 * https://www.youtube.com/watch?v=NWw-sNSlnvo&list=PLJEe15qBQuSZTvoeZ67Md9CPK8I13xk_Z
 */
