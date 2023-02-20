use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let navbrand = html! {
        <ybc::NavbarItem tag={ybc::NavbarItemTag::Div} classes={classes!("is-size-3")}>{ "YBC - C&P" }</ybc::NavbarItem>
    };

    let navstart = html! {
        <>
        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
            { "Home" }
        </Link<Route>>
        <ybc::NavbarDropdown navlink={html!{"Elements"}}>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Block}>
                { "Block" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Notification}>
                { "Notification" }
            </Link<Route>>
        </ybc::NavbarDropdown>
        <ybc::NavbarDropdown navlink={html!{"Layouts"}}>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Container}>
                { "Container" }
            </Link<Route>>
        </ybc::NavbarDropdown>
        </>
    };

    html! {
        <ybc::Navbar navburger={true} {navbrand} classes={classes!("is-primary")} {navstart} />
    }
}
