use yew::{function_component, html, Html};

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <>
          <h2 class="text-3xl mt-8 px-12 font-bold tracking-widest text-center md:mt-14">{"Mis Últimos Proyectos"}</h2>
          <div class="flex flex-col items-center justify-center mx-auto w-11/12 lg:flex-row lg:my-4">
            <article class="border hover:border-sky-600 rounded-lg bg-slate-800 m-2 my-4">
              <div class="m-2">
                <img class="opacity-80 hover:opacity-100" src="img/todoapp-tauri-yew.jpg" alt="Todoapp Tauri Yew" />
                <h4 class="font-bold my-2">{ "Todoapp + Tauri + Yew" }</h4>
                <div class="flex justify-between items-center">
                  <div class="flex flex-nowrap items-center">
                    <span class="bg-slate-200 rounded-lg px-2 mr-2 text-slate-700 text-xs">
                      { "RUST" }
                    </span>
                    <span class="bg-slate-200 rounded-lg px-2 mr-2 text-slate-700 text-xs">
                      { "TAURI" }
                    </span>
                    <span class="bg-slate-200 rounded-lg px-2 mr-1 text-slate-700 text-xs">
                      { "TAILWINDCSS" }
                    </span>
                  </div>

                  <a href="https://github.com/emarifer/todoapp-tauri-yew" target="_blank">
                    <img class="w-6 pr-2" src="img/white-eye.svg" alt="White Eye" />
                  </a>
                </div>
              </div>
            </article>

            <article class="border hover:border-sky-600 rounded-lg bg-slate-800 m-2 my-4">
              <div class="m-2">
                <img class="opacity-80 hover:opacity-100" src="img/my-gps-app.jpg" alt="Mi GPS App" />
                <h4 class="font-bold my-2">{ "My GPS App" }</h4>
                <div class="flex justify-between items-center">
                  <div class="flex flex-nowrap items-center">
                    <span class="bg-slate-200 rounded-lg px-2 mr-2 text-slate-700 text-xs">
                      { "FLUTTER" }
                    </span>
                    <span class="bg-slate-200 rounded-lg px-2 mr-2 text-slate-700 text-xs">
                      { "MOBILE APP" }
                    </span>
                    <span class="bg-slate-200 rounded-lg px-2 mr-1 text-slate-700 text-xs">
                      { "DART" }
                    </span>
                  </div>

                  <a href="https://github.com/emarifer/my-gps-app" target="_blank">
                    <img class="w-6 pr-2" src="img/white-eye.svg" alt="White Eye" />
                  </a>
                </div>
              </div>
            </article>

            <article class="border hover:border-sky-600 rounded-lg bg-slate-800 m-2 my-4">
              <div class="m-2">
                <img class="opacity-80 hover:opacity-100" src="img/my-pwa.jpg" alt="My PWA" />
                <h4 class="font-bold my-2">{ "My PWA" }</h4>
                <div class="flex justify-between items-center">
                  <div class="flex flex-nowrap items-center">
                    <span class="bg-slate-200 rounded-lg px-2 mr-2 text-slate-700 text-xs">
                      { "PWA" }
                    </span>
                    <span class="bg-slate-200 rounded-lg px-2 mr-2 text-slate-700 text-xs">
                      { "VUEJS" }
                    </span>
                    <span class="bg-slate-200 rounded-lg px-2 mr-1 text-slate-700 text-xs">
                      { "NOTES IN DB" }
                    </span>
                  </div>

                  <a href="https://github.com/emarifer/my-pwa" target="_blank">
                    <img class="w-6 pr-2" src="img/white-eye.svg" alt="White Eye" />
                  </a>
                </div>
              </div>
            </article>
          </div>
        </>
    }
}
