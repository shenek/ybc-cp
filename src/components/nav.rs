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
            <Link<Route> classes={classes!("navbar-item")} to={Route::Content}>
                { "Content" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Delete}>
                { "Delete" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Icon}>
                { "Icon" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Image}>
                { "Image" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Notification}>
                { "Notification" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Progress}>
                { "Progress" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Table}>
                { "Table" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Tag}>
                { "Tag" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Title}>
                { "Title" }
            </Link<Route>>
        </ybc::NavbarDropdown>
        <ybc::NavbarDropdown navlink={html!{"Components"}}>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Message}>
                { "Message" }
            </Link<Route>>
        </ybc::NavbarDropdown>
        <ybc::NavbarDropdown navlink={html!{"Layouts"}}>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Container}>
                { "Container" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::MediaObject}>
                { "Media Object" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Footer}>
                { "Footer" }
            </Link<Route>>
        </ybc::NavbarDropdown>
        </>
    };

    html! {
        <ybc::Navbar
          navburger={true}
          {navbrand}
          classes={classes!("is-primary")}
          {navstart}
          fixed={Some(ybc::NavbarFixed::Top)}
        />
    }
}
