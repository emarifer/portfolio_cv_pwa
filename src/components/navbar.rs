use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};
use yew_router::prelude::Link;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub activeroute: UseStateHandle<String>,
    pub activesidebar: UseStateHandle<bool>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let hamburger_on_pressed = {
        let activesidebar = props.activesidebar.clone();

        Callback::from(move |_| {
            activesidebar.set(!*activesidebar);
        })
    };

    html! {
        <nav class="w-full h-16 px-6 md:px-12 flex justify-between items-center bg-sky-400 fixed top-0 left-0 right-0 z-50">
          <Link<Route> to={Route::Home}>
            <div class="flex items-center gap-2">
              <img class="w-12 hover:scale-110 active:scale-110 duration-500 ease-in-out" src="img/logo.png" alt="Logo" />
              <span class="text-sm text-slate-700">{"Mi Landing Page"}</span>
            </div>
          </Link<Route>>

          <div class="hidden md:flex justify-between items-center gap-14">
            <div class="flex justify-center items-center gap-6">
              <span class={format!("hover:text-sky-700 {}", if &*props.activeroute == "/portfolio_cv_pwa/" { "active-route" } else { "" })}>
                <Link<Route> to={Route::Home}>{ "Quién Soy" }</Link<Route>>
              </span>

              <span class={format!("hover:text-sky-700 {}", if &*props.activeroute == "/portfolio_cv_pwa/mycv" { "active-route" } else { "" })}>
                <Link<Route> to={Route::MyCV}>{ "Mi CV" }</Link<Route>>
              </span>
            </div>

            <div class="flex justify-center items-center gap-6">
              <a href="https://www.laserrania.org/autor/enrique-marin-fernandez/" target="_blank" class="flex items-center gap-2">
                <img src="img/book-half.svg" alt="Books icon" class="w-8 hover:scale-110 duration-500 ease-in-out" />
                <span class="text-sm text-slate-700">{"Mis Libros"}</span>
              </a>

              <a href="https://github.com/emarifer?tab=repositories" target="_blank" class="flex items-center gap-2">
                <img src="img/github.svg" alt="GitHub icon" class="w-8 hover:scale-110 duration-500 ease-in-out" />
                <span class="text-sm text-slate-700">{"Mi GitHub"}</span>
              </a>
            </div>
          </div>

          <div class="block md:hidden" onclick={hamburger_on_pressed}>
            <img class="w-8 cursor-pointer active:bg-sky-600" src="img/list.svg" alt="Hamburger button" />
          </div>
        </nav>
    }
}

/*
 * Cómo hacer que el elemento sea invisible en tamaño móvil pero visible en tamaño laptop en
 * TailwindCSS. VER:
 * https://stackoverflow.com/questions/70647076/how-to-make-element-invisible-in-mobile-size-but-visible-in-laptop-size-in-tailw
 */
