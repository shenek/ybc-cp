use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(Progress)]
pub fn block() -> Html {
    let code_1 = r#"html! {
    <ybc::Progress max={100_f32} value={15_f32} />
}"#;
    let preview_1 = html! {
        <ybc::Progress max={100_f32} value={15_f32} />
    };

    let code_2 = r#"html! {
    <ybc::Progress max={100_f32} value={15_f32} classes={"is-primary"} />
    <ybc::Progress max={100_f32} value={30_f32} classes={"is-link"} />
    <ybc::Progress max={100_f32} value={45_f32} classes={"is-info"} />
    <ybc::Progress max={100_f32} value={60_f32} classes={"is-success"} />
    <ybc::Progress max={100_f32} value={75_f32} classes={"is-warning"} />
    <ybc::Progress max={100_f32} value={90_f32} classes={"is-danger"} />
}"#;
    let preview_2 = html! {
        <>
            <ybc::Progress max={100_f32} value={15_f32} classes={"is-primary"} />
            <ybc::Progress max={100_f32} value={30_f32} classes={"is-link"} />
            <ybc::Progress max={100_f32} value={45_f32} classes={"is-info"} />
            <ybc::Progress max={100_f32} value={60_f32} classes={"is-success"} />
            <ybc::Progress max={100_f32} value={75_f32} classes={"is-warning"} />
            <ybc::Progress max={100_f32} value={90_f32} classes={"is-danger"} />
        </>
    };

    let code_3 = r#"html! {
    <ybc::Progress max={100_f32} value={20_f32} classes={"is-small"} />
    <ybc::Progress max={100_f32} value={40_f32} classes={"is-normal"} />
    <ybc::Progress max={100_f32} value={60_f32} classes={"is-medium"} />
    <ybc::Progress max={100_f32} value={80_f32} classes={"is-large"} />
}"#;
    let preview_3 = html! {
        <>
            <ybc::Progress max={100_f32} value={20_f32} classes={"is-small"} />
            <ybc::Progress max={100_f32} value={40_f32} classes={"is-normal"} />
            <ybc::Progress max={100_f32} value={60_f32} classes={"is-medium"} />
            <ybc::Progress max={100_f32} value={80_f32} classes={"is-large"} />
        </>
    };

    let code_4 = r#"html! {
    // TODO wait for PR
    <ybc::Progress max={100_f32} />
}"#;
    // TODO wait for PR
    let preview_4 = html! {
        <ybc::Progress max={100_f32} />
    };

    let code_5 = r#"
let value = use_state(|| 20_f32);
let inc_cb = {
    let value = value.clone();
    Callback::from(move |_| value.set(*value + 10_f32))
};
let dec_cb = {
    let value = value.clone();
    Callback::from(move |_| value.set(*value - 10_f32))
};

html! {
    <>
        <ybc::Progress max={100_f32} value={*value} classes={"is-small"} />
        <ybc::Buttons classes={classes!("has-addons")}>
            <ybc::Button onclick={dec_cb}>{"-"}</ybc::Button>
            <ybc::Button onclick={inc_cb}>{"+"}</ybc::Button>
        </ybc::Buttons>
    </>
}"#;

    let value = use_state(|| 20_f32);
    let inc_cb = {
        let value = value.clone();
        Callback::from(move |_| value.set(*value + 10_f32))
    };
    let dec_cb = {
        let value = value.clone();
        Callback::from(move |_| value.set(*value - 10_f32))
    };
    let preview_5 = html! {
        <>
            <ybc::Progress max={100_f32} value={*value} classes={classes!("is-small")} />
            <ybc::Buttons classes={classes!("has-addons")}>
                <ybc::Button onclick={dec_cb}>{"-"}</ybc::Button>
                <ybc::Button onclick={inc_cb}>{"+"}</ybc::Button>
            </ybc::Buttons>
        </>
    };

    let descriptions = vec![
        description::Item {
            title: "Progress".to_owned(),
            id: "progress".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Progress colors".to_owned(),
            id: "progress-colors".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Progress sizes".to_owned(),
            id: "progress-sizes".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
        },
        description::Item {
            title: "Progress indeterminate".to_owned(),
            id: "progress-indeterminate".to_owned(),
            code: code_4.to_owned(),
            html: Rc::new(preview_4),
        },
        description::Item {
            title: "Progress usage".to_owned(),
            id: "progress-usage".to_owned(),
            code: code_5.to_owned(),
            html: Rc::new(preview_5),
        },
    ];

    let api: Vec<description::Api> = vec![(
        "<ybc::Progress>",
        "https://docs.rs/ybc/latest/ybc/struct.ProgressProps.html",
    )
        .into()];

    html! {
        <description::Description items={descriptions} {api} />
    }
}
