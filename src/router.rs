use yew::prelude::{html, Html};
use yew_router::prelude::Routable;

use crate::components::pages::{home::Home, mycv::MyCV, not_found::NotFound};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/portfolio_cv_pwa/")]
    Home,
    #[at("/portfolio_cv_pwa/mycv")]
    MyCV,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html!(<Home />),
        Route::MyCV => html!(<MyCV />),
        Route::NotFound => html!(<NotFound />),
    }
}
