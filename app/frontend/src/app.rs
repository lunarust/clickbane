use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;
use common::*;

use crate::components::nav::Nav;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::home::Home;

use crate::pages::reports::Reports;
use crate::pages::jobs::Jobs;
use crate::pages::customers::Customers;
use crate::pages::configuration_js::Confjs;
use crate::pages::configuration::Conf;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/Jobs")]
    Jobs,
    #[at("/Reports")]
    Reports,
    #[at("/Customers")]
    Customers,
    #[at("/Conf")]
    Conf,
    #[at("/ConfJS")]
    Confjs,
    #[not_found]
    #[at("/404")]
    NotFound,
}
#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>

            <footer class="footer">
                <div class="content has-text-right">

                    <span class="footer_icon"><a href="https://yew.rs">
                    <img
                        src="./resources/yewstack.png"
                        alt="Powered by Yew"
                        width="24"
                        height="24"
                        /></a></span>

                    <span class="footer_icon"><a href="https://rust-lang.org/">
                    <img
                        src="./resources/rust-logo-512x512-blk.png"
                        alt="Powered by Yew"
                        width="24"
                        height="24"
                        /></a></span>

                    <span class="footer_icon"><a href="https://github.com/lunarust/clickbane">
                      <img
                        src="./resources/GitHub_Invertocat_Black_Clearspace.png"
                        alt="GitHub"
                        width="24"
                        height="24" /></a></span>

                    <span class="footer_icon"><a href="https://bulma.io">
                      <img
                        src="./resources/BulmaIcon.png"
                        alt="Made with Bulma"
                        width="15"
                        height="24" />
                    </a></span>

                    <span class="footer_icon">
                      <img
                        src="./resources/logo.svg"
                        alt="Kappa"
                        width="30"
                        height="28" /></span>
                </div>
            </footer>

            <div id="logo">
            <span class="footer_logo">
              <img
                src="./resources/ClickBane.png"
                alt="ClickBane"
                width="75"
                height="68" /></span>
            </div>

        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}


#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            //<Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>

            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">
                      <img
                        src="https://bulma.io/assets/images/made-with-bulma--dark.png"
                        alt="Made with Bulma"
                        width="128"
                        height="24" />
                    </a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>

        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::Jobs => {
            html! { <Jobs customer={Customer::default()} reload=0 /> }
        }
        Route::Reports => {
            html! { <Reports /> }
        }
        Route::Customers => {
            html! { <Customers /> }
        }
        Route::Confjs => {
            html! { <Confjs /> }
        }
        Route::Conf => {
            html! { <Conf /> }
        }
    }
}
