use yew::prelude::*;

#[function_component]
fn InfoTiles() -> Html {
    html! {
        <>
            <ybc::Tile ctx={ybc::TileCtx::Parent}>
                <ybc::Tile ctx={ybc::TileCtx::Child} classes={classes!("box")}>
                    <ybc::Title tag={"p"}>{ "What is Yew?" }</ybc::Title>
                    <ybc::Subtitle tag={"p"}>{ "A framework for creating reliable and efficient web applications." }</ybc::Subtitle>
                    <ybc::Image size={ybc::ImageSize::Is1by1}><a href={"https://yew.rs/"}><img src={"https://yew.rs/img/logo.svg"}/></a></ybc::Image>
                </ybc::Tile>
            </ybc::Tile>

            <ybc::Tile ctx={ybc::TileCtx::Parent}>
                <ybc::Tile ctx={ybc::TileCtx::Child} classes={classes!("box")}>
                    <ybc::Title tag={"p"}>{ "What is Bulma?" }</ybc::Title>
                    <ybc::Subtitle tag={"p"}>{ "Bulma: the modern CSS framework that just works." }</ybc::Subtitle>
                    <ybc::Image size={ybc::ImageSize::Is1by1}><a href={"https://bulma.io/"}><img src={"https://raw.githubusercontent.com/jgthms/bulma/master/docs/assets/Bulma%20Icon.svg"}/></a></ybc::Image>
                </ybc::Tile>
            </ybc::Tile>

            <ybc::Tile ctx={ybc::TileCtx::Parent}>
                <ybc::Tile ctx={ybc::TileCtx::Child} classes={classes!("box")}>
                    <ybc::Title tag={"p"}>{ "What is YBC?" }</ybc::Title>
                    <ybc::Subtitle tag={"p"}>{ "A Yew component library based on the Bulma CSS framework." }</ybc::Subtitle>
                    <ybc::Content>
                        {r#"
                            YBC encapsulates all of the structure, style and functionality of the Bulma CSS framework as a set of Yew components.
                            YBC also ships with support for the Yew Router, adding Bulma-styled components which wrap the Yew Router components for clean integration.
                        "#}
                    </ybc::Content>
                    <ybc::Content>
                        {r#"
                            As a guiding principle, YBC does not attempt to encapsulate every single Bulma style as a Rust type, let alone the many valid style combinations.
                            That would be far too complex, and probably limiting to the user in many ways. Instead, YBC handles structure, required classes, functionality,
                            sane defaults and every component can be customized with any additional classes for an exact look and feel.
                        "#}
                    </ybc::Content>
                </ybc::Tile>
            </ybc::Tile>
        </>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let title_body = html! {
        <ybc::Container>
            <ybc::Title size={Some(ybc::HeaderSize::Is1)} tag={"h1"}>{"YBC - Copy & Paste"}</ybc::Title>
            <ybc::Subtitle tag={"h2"}>{"Ready to use examples"}</ybc::Subtitle>
        </ybc::Container>
    };

    html! {
        <ybc::Tile ctx={ybc::TileCtx::Ancestor} classes={classes!("container", "is-vertical")}>
            <ybc::Tile ctx={ybc::TileCtx::Child}>
                <ybc::Hero body_classes={classes!("pb-0", "has-text-centered")} body={title_body} />
            </ybc::Tile>

            <ybc::Tile ctx={ybc::TileCtx::Parent} classes={classes!("container")}>
                <InfoTiles />
            </ybc::Tile>
        </ybc::Tile>
    }
}
