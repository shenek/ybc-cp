use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: Html,
}

#[function_component(Preview)]
pub fn preview(props: &Props) -> Html {
    html! {
        <>
        <hr />
        <ybc::Block classes={classes!("p-2", "mb-3")}>{ props.html.clone() }</ybc::Block>
        <hr />
        </>
    }
}
