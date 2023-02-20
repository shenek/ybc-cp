use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::{
    components::nav::Nav,
    components::code::highlight_style,
    pages::{
        home::Home,
        page_not_found::PageNotFound,
        layout::Container,
        elements::{Notification, Block},
    }
};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/container/")]
    Container,
    #[at("/block/")]
    Block,
    #[at("/notification/")]
    Notification,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <ybc::Footer>
            <ybc::Content classes={classes!("has-text-centered")} tag={"div"}>
                { "Powered by " }
                <a href="https://yew.rs">{ "Yew" }</a>
                { " using " }
                <a href="https://bulma.io">{ "Bulma" }</a>
                { " and " }
                <a href="https://github.com/thedodd/ybc">{ "YBC" }</a>
                { "." }
            </ybc::Content>
        </ybc::Footer>
        
    }
    
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <Nav />
            <main>
                <Switch<Route> render={switch} />
                <Footer />
            </main>
        </BrowserRouter>
        {highlight_style()}
        </>
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
        <>
        <Router history={history}>
            <Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>
            <Footer />
        </Router>
        {highlight_style()}
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Container => {
            html! { <Container /> }
        }
        Route::Block => {
            html! { <Block /> }
        }
        Route::Notification => {
            html! { <Notification /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
