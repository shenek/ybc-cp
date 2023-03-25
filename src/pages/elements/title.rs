use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(Title)]
pub fn title() -> Html {
    let code_1 = r#"html! {
    <ybc::Title>{"Title"}</ybc::Title>
    <ybc::Subtitle>{"SubTitle"}</ybc::Subtitle>
}"#;
    let preview_1 = html! {
        <>
        <ybc::Title>{"Title"}</ybc::Title>
        <ybc::Subtitle>{"SubTitle"}</ybc::Subtitle>
        </>
    };

    let code_2 = r#"html! {
    <ybc::Title is_spaced={true}>{"Title"}</ybc::Title>
    <ybc::Subtitle>{"SubTitle"}</ybc::Subtitle>
}"#;
    let preview_2 = html! {
        <>
        <ybc::Title is_spaced={true}>{"Title"}</ybc::Title>
        <ybc::Subtitle>{"SubTitle"}</ybc::Subtitle>
        </>
    };

    let code_3 = r#"html! {
    <ybc::Title size={ybc::HeaderSize::Is1}>{"Title 1"}</ybc::Title>
    <ybc::Title size={ybc::HeaderSize::Is2}>{"Title 2"}</ybc::Title>
    <ybc::Title size={ybc::HeaderSize::Is3}>{"Title 3 (default)"}</ybc::Title>
    <ybc::Title size={ybc::HeaderSize::Is4}>{"Title 4"}</ybc::Title>
    <ybc::Title size={ybc::HeaderSize::Is5}>{"Title 5"}</ybc::Title>
    <ybc::Title size={ybc::HeaderSize::Is6}>{"Title 6"}</ybc::Title>
    <ybc::Subtitle size={ybc::HeaderSize::Is1}>{"SubTitle 1"}</ybc::Subtitle>
    <ybc::Subtitle size={ybc::HeaderSize::Is2}>{"SubTitle 2"}</ybc::Subtitle>
    <ybc::Subtitle size={ybc::HeaderSize::Is3}>{"SubTitle 3"}</ybc::Subtitle>
    <ybc::Subtitle size={ybc::HeaderSize::Is4}>{"SubTitle 4"}</ybc::Subtitle>
    <ybc::Subtitle size={ybc::HeaderSize::Is5}>{"SubTitle 5 (default)"}</ybc::Subtitle>
    <ybc::Subtitle size={ybc::HeaderSize::Is6}>{"SubTitle 6"}</ybc::Subtitle>
}"#;
    let preview_3 = html! {
        <>
        <ybc::Title size={ybc::HeaderSize::Is1}>{"Title 1"}</ybc::Title>
        <ybc::Title size={ybc::HeaderSize::Is2}>{"Title 2"}</ybc::Title>
        <ybc::Title size={ybc::HeaderSize::Is3}>{"Title 3 (default)"}</ybc::Title>
        <ybc::Title size={ybc::HeaderSize::Is4}>{"Title 4"}</ybc::Title>
        <ybc::Title size={ybc::HeaderSize::Is5}>{"Title 5"}</ybc::Title>
        <ybc::Title size={ybc::HeaderSize::Is6}>{"Title 6"}</ybc::Title>
        <ybc::Subtitle size={ybc::HeaderSize::Is1}>{"SubTitle 1"}</ybc::Subtitle>
        <ybc::Subtitle size={ybc::HeaderSize::Is2}>{"SubTitle 2"}</ybc::Subtitle>
        <ybc::Subtitle size={ybc::HeaderSize::Is3}>{"SubTitle 3"}</ybc::Subtitle>
        <ybc::Subtitle size={ybc::HeaderSize::Is4}>{"SubTitle 4"}</ybc::Subtitle>
        <ybc::Subtitle size={ybc::HeaderSize::Is5}>{"SubTitle 5 (default)"}</ybc::Subtitle>
        <ybc::Subtitle size={ybc::HeaderSize::Is6}>{"SubTitle 6"}</ybc::Subtitle>
        </>
    };

    let descriptions = vec![
        description::Item {
            title: "Title and Subtitle".to_owned(),
            id: "title".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Title and Subtitle spaced".to_owned(),
            id: "title-spaced".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Title and Subtitle sizes".to_owned(),
            id: "title-sizes".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
        },
    ];

    let api: Vec<description::Api> = vec![
        (
            "<ybc::Title>",
            "https://docs.rs/ybc/latest/ybc/struct.TitleProps.html",
        )
            .into(),
        (
            "<ybc::Subtitle>",
            "https://docs.rs/ybc/latest/ybc/struct.SubtitleProps.html",
        )
            .into(),
    ];

    html! {
        <description::Description items={descriptions} {api} />
    }
}
