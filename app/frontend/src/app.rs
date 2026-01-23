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
                <div class="content has-text-centered">
                    <img
                        src="./ClickBane.png"
                        alt="Clickbane"
                        width="48"
                        height="48"
                        />
                    <a href="https://bulma.io">
                      <img
                        src="./BulmaIcon.png"
                        alt="Made with Bulma"
                        width="31"
                        height="48" />
                    </a>
                    <a href="https://yew.rs">
                    <img
                        src="https://avatars.githubusercontent.com/u/49116234?s=48&v=4"
                        alt="Powered by Yew"
                        width="48"
                        height="48"
                        /></a>
                    <a href="https://github.com/lunarust">
                      <img
                        src="../GitHub_Invertocat_Black_Clearspace.png"
                        alt="GitHub"
                        width="48"
                        height="48" /></a>
                </div>
            </footer>

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
