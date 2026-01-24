use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Nav() -> Html {
    let navbar_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let active_class = if !*navbar_active { "is-active" } else { "" };

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-3">{ "ClickBane" }</h1>

                <button class={classes!("navbar-burger", "burger", active_class)}
                    aria-label="menu" aria-expanded="false"
                    onclick={toggle_navbar}
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </button>
            </div>
            <div class={classes!("navbar-menu", active_class)}>
                <div class="navbar-start">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                        { " Home " }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Jobs}>
                        { " Scheduled Reports " }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Reports}>
                        { " All Reports " }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Customers}>
                        { " Customers " }
                    </Link<Route>>

                    <div class="navbar-item has-dropdown is-hoverable">
                        <div class="navbar-link">
                            { "⚙" }
                        </div>
                        <div class="navbar-dropdown">
                            <Link<Route> classes={classes!("navbar-item")} to={Route::Conf}>
                                { "Settings" }
                            </Link<Route>>
                            <Link<Route> classes={classes!("navbar-item")} to={Route::Confjs}>
                                { "JasperServer Settings" }
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
