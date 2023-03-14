use yew::{function_component, html, use_state, Callback, Html};

use crate::components::cv_data::CVData;

#[function_component(MyCV)]
pub fn mycv() -> Html {
    let is_active = use_state(|| 1);

    let on_pressed_tab = {
        let is_active = is_active.clone();

        Callback::from(move |index: i32| is_active.set(index))
    };

    html! {
        <>
          <header class="flex flex-col justify-center mt-12 py-2 gap-10">
              <h1 class="text-sky-300 text-3xl text-center font-bold tracking-widest leading-10 mx-auto">{"Mi Curriculum Vitae"}</h1>
              <div class="rounded-full w-64 mx-auto shadow-md shadow-sky-800">
                <img class="inline-block rounded-full w-64" src="img/profile.jpeg" alt="Profile image" />
              </div>
          </header>

          <section class="mt-16">
              <h2 class="text-2xl font-bold text-slate-600 text-center">{"Conoce mi Experiencia"}</h2>

              <div class="my-8 mx-auto flex flex-col w-11/12 md:flex-row md:w-11/12 md:my-12">


                /* CONTENEDOR DE LAS PESTAÑAS */
                <div class="flex justify-between w-auto overflow-x-auto md:flex-col md:w-5/12 lg:w-auto">

                  <button onclick={let on_pressed_tab = on_pressed_tab.clone();
                    move |_| on_pressed_tab.emit(1)}
                    class={format!("tab {}", if *is_active == 1 { "border-sky-500 bg-slate-700" } else { "" })}>
                    { "Datos Personales" }
                  </button>
                  <button onclick={let on_pressed_tab = on_pressed_tab.clone();
                    move |_| on_pressed_tab.emit(2)}
                    class={format!("tab {}", if *is_active == 2 { "border-sky-500 bg-slate-700" } else { "" })}>
                    { "Datos Académicos" }
                  </button>
                  <button onclick={let on_pressed_tab = on_pressed_tab.clone();
                    move |_| on_pressed_tab.emit(3)}
                    class={format!("tab {}", if *is_active == 3 { "border-sky-500 bg-slate-700" } else { "" })}>
                    { "Vida Laboral" }
                  </button>
                  <button onclick={let on_pressed_tab = on_pressed_tab.clone();
                    move |_| on_pressed_tab.emit(4)}
                    class={format!("tab {}", if *is_active == 4 { "border-sky-500 bg-slate-700" } else { "" })}>
                    { "Autor de Libros" }
                  </button>
                  <button onclick={let on_pressed_tab = on_pressed_tab.clone();
                    move |_| on_pressed_tab.emit(5)}
                    class={format!("tab {}", if *is_active == 5 { "border-sky-500 bg-slate-700" } else { "" })}>
                    { "Aptitudes" }
                  </button>

                  <img class="w-6 bg-primary opacity-[0.85] sticky top-0 right-0 z-10 md:hidden" src="img/left-swipe.svg" alt="Swipe left" />
                </div>

                /* CONTENEDOR DE LOS DATOS DEL CV */
                <div class="my-5 md:px-10 md:my-0 md:max-w-[500px] lg:max-w-[800px]">
                  <CVData activetab={is_active} />
                </div>
              </div>
          </section>
        </>
    }
}

/*
 * texto de un botón en una línea. VER:
 * https://stackoverflow.com/questions/41248992/get-button-text-on-to-one-line
 */
