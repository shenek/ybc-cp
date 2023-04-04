use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::{
    components::code::highlight_style,
    components::nav::Nav,
    pages::{
        components::{Menu, Message, Pagination},
        elements::{
            Block, Content, Delete, Icon, Image, Notification, Progress, Table, Tag, Title,
        },
        form::General,
        home::Home,
        layout::{Container, Footer, MediaObject},
        columns::{ColumnsBasics, ColumnsSizes, ColumnsResponsiveness},
        page_not_found::PageNotFound,
    },
};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/columns/basics/")]
    ColumnsBasics,
    #[at("/columns/sizes/")]
    ColumnsSizes,
    #[at("/columns/responsiveness/")]
    ColumnsResponsiveness,
    #[at("/layout/container/")]
    Container,
    #[at("/layout/media_object/")]
    MediaObject,
    #[at("/layout/footer/")]
    Footer,
    #[at("/elements/block/")]
    Block,
    #[at("/elements/content/")]
    Content,
    #[at("/elements/delete/")]
    Delete,
    #[at("/elements/icon/")]
    Icon,
    #[at("/elements/image/")]
    Image,
    #[at("/elements/notification/")]
    Notification,
    #[at("/elements/progress/")]
    Progress,
    #[at("/elements/table/")]
    Table,
    #[at("/elements/tag/")]
    Tag,
    #[at("/elements/title/")]
    Title,
    #[at("/components/menu/")]
    Menu,
    #[at("/components/message/")]
    Message,
    #[at("/components/pagination/")]
    Pagination,
    #[at("/form/general/")]
    General,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(YbcCpFooter)]
pub fn ybc_cp_footer() -> Html {
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
                <YbcCpFooter />
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
        Route::MediaObject => {
            html! { <MediaObject /> }
        }
        Route::Footer => {
            html! { <Footer /> }
        }
        Route::Block => {
            html! { <Block /> }
        }
        Route::Content => {
            html! { <Content /> }
        }
        Route::Delete => {
            html! { <Delete /> }
        }
        Route::Icon => {
            html! { <Icon /> }
        }
        Route::Image => {
            html! { <Image /> }
        }
        Route::Table => {
            html! { <Table /> }
        }
        Route::Tag => {
            html! { <Tag /> }
        }
        Route::Title => {
            html! { <Title /> }
        }
        Route::Notification => {
            html! { <Notification /> }
        }
        Route::Progress => {
            html! { <Progress /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::Menu => {
            html! { <Menu /> }
        }
        Route::Message => {
            html! { <Message /> }
        }
        Route::Pagination => {
            html! { <Pagination /> }
        }
        Route::General => {
            html! { <General /> }
        }
        Route::ColumnsBasics => {
            html! { <ColumnsBasics /> }
        }
        Route::ColumnsSizes => {
            html! { <ColumnsSizes /> }
        }
        Route::ColumnsResponsiveness => {
            html! { <ColumnsResponsiveness /> }
        }
    }
}
