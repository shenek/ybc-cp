use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let navbrand = html! {
        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
            <ybc::NavbarItem tag={ybc::NavbarItemTag::Div} classes={classes!("is-size-3")}>
                { "YBC - C&P" }
            </ybc::NavbarItem>
        </Link<Route>>
    };

    let navstart = html! {
        <>
        <ybc::NavbarDropdown navlink={html!{"Columns"}}>
            <Link<Route> classes={classes!("navbar-item")} to={Route::ColumnsBasics}>
                { "Basics" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::ColumnsSizes}>
                { "Sizes" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::ColumnsResponsiveness}>
                { "Responsiveness" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::ColumnsNesting}>
                { "Nesting" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::ColumnsGap}>
                { "Gap" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::ColumnsOptions}>
                { "Options" }
            </Link<Route>>
        </ybc::NavbarDropdown>
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
        <ybc::NavbarDropdown navlink={html!{"Form"}}>
            <Link<Route> classes={classes!("navbar-item")} to={Route::General}>
                { "General" }
            </Link<Route>>
        </ybc::NavbarDropdown>
        <ybc::NavbarDropdown navlink={html!{"Component"}}>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Menu}>
                { "Menu" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Message}>
                { "Message" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Pagination}>
                { "Pagination" }
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
