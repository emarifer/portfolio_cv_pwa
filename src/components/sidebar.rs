use yew::{classes, function_component, html, Callback, Html, Properties, UseStateHandle};
use yew_router::prelude::Link;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub activesidebar: UseStateHandle<bool>,
    pub activeroute: UseStateHandle<String>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let close_on_pressed = {
        let activesidebar = props.activesidebar.clone();

        Callback::from(move |_| {
            activesidebar.set(!*activesidebar);
        })
    };

    let overlay = if *props.activesidebar {
        html!( <div onclick={close_on_pressed.clone()} class="bg-black opacity-60 fixed top-16 right-0 bottom-0 left-0 z-30"></div>  )
    } else {
        html!()
    };

    let sidebar_style = "fixed top-16 bottom-0 left-0 z-40 w-2/3 p-4 bg-slate-700 \
                               flex flex-col transition-transform -translate-x-full duration-700";
    let sidebar_visible = "translate-x-0";
    let link_style = "text-sm active:text-sky-500 pb-1";

    html! {
        <>
          {overlay}
          <div class={classes!(sidebar_style, if *props.activesidebar { sidebar_visible } else { "" })}>
            <button onclick={close_on_pressed.clone()} class="absolute top-4 right-4 active:bg-slate-800">
              <svg class="w-6" viewBox="0 0 16 16">
                <g stroke="currentColor">
                  <path stroke-width="1" d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 \
                  8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z" />
                </g>
              </svg>
            </button>

            <div class="flex flex-col gap-4 mt-4">
              <div>
                  <Link<Route> to={Route::Home}>
                    <span
                      class={classes!(if &*props.activeroute == "/portfolio_cv_pwa/" {"active-route"} else {""}, link_style)}
                      onclick={close_on_pressed.clone()}>
                      <svg fill="currentColor" class="w-4 inline mr-2" viewBox="0 0 16 16">
                        <path d="M6.5 14.5v-3.505c0-.245.25-.495.5-.495h2c.25 0 .5.25.5.5v3.5a.5.5 0 0 0 \
                        .5.5h4a.5.5 0 0 0 .5-.5v-7a.5.5 0 0 0-.146-.354L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293L8.354 \
                        1.146a.5.5 0 0 0-.708 0l-6 6A.5.5 0 0 0 1.5 7.5v7a.5.5 0 0 0 .5.5h4a.5.5 0 0 0 .5-.5Z"/>
                      </svg>
                      {"Quién Soy"}
                    </span>
                  </Link<Route>>
              </div>

              <div>
                  <Link<Route> to={Route::MyCV}>
                    <span class={classes!(if &*props.activeroute == "/portfolio_cv_pwa/mycv" {"active-route"} else {""}, link_style)}
                      onclick={close_on_pressed}>
                      <svg fill="currentColor" class="w-4 inline mr-2" viewBox="0 0 16 16">
                        <path d="M9.293 0H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V4.707A1 1 0 0 0 \
                        13.707 4L10 .293A1 1 0 0 0 9.293 0zM9.5 3.5v-2l3 3h-2a1 1 0 0 1-1-1zM11 8a3 3 0 1 1-6 0 3 3 0 0 1 6 0zm2 \
                        5.755V14a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-.245S4 12 8 12s5 1.755 5 1.755z"/>
                      </svg>
                      {"Mi CV"}
                    </span>
                  </Link<Route>>
              </div>
            </div>

            <hr class="mt-8 border-sky-500" />

            <div class="h-full w-full mt-8 bg-slate-900 rounded-xl flex flex-col p-4 gap-6">
              <a href="https://www.laserrania.org/autor/enrique-marin-fernandez/" target="_blank" class="text-sm active:text-sky-500 flex gap-2">
                <svg fill="currentColor" class="w-4" viewBox="0 0 16 16">
                  <path d="M8.5 2.687c.654-.689 1.782-.886 3.112-.752 1.234.124 2.503.523 \
                  3.388.893v9.923c-.918-.35-2.107-.692-3.287-.81-1.094-.111-2.278-.039-3.213.492V2.687zM8 \
                  1.783C7.015.936 5.587.81 4.287.94c-1.514.153-3.042.672-3.994 1.105A.5.5 0 0 0 0 2.5v11a.5.5 0 0 0 \
                  .707.455c.882-.4 2.303-.881 3.68-1.02 1.409-.142 2.59.087 3.223.877a.5.5 0 0 0 .78 0c.633-.79 \
                  1.814-1.019 3.222-.877 1.378.139 2.8.62 3.681 1.02A.5.5 0 0 0 16 13.5v-11a.5.5 0 0 \
                  0-.293-.455c-.952-.433-2.48-.952-3.994-1.105C10.413.809 8.985.936 8 1.783z"/>
                </svg>
                {"Mis Libros"}
              </a>

              <a href="https://github.com/emarifer?tab=repositories" target="_blank" class="text-sm active:text-sky-500 flex gap-2">
                <svg fill="currentColor" class="w-4" viewBox="0 0 16 16">
                  <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 \
                  0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 \
                  1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 \
                  0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 \
                  2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 \
                  3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                </svg>
                {"Mi GitHub"}
              </a>
            </div>
          </div>
        </>
    }
}

/*
 * Ancho CSS de una etiqueta <span>. VER:
 * https://stackoverflow.com/questions/621401/css-width-of-a-span-tag#621409
 */
