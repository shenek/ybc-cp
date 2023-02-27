use crate::components::{code::Code, preview::Preview};
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

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Progress"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Progress colors"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Progress sizes"}</ybc::Title>
                <Preview html={preview_3} />
                <Code code={code_3}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Progress indeterminate"}</ybc::Title>
                <Preview html={preview_4} />
                <Code code={code_4}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Progress usage"}</ybc::Title>
                <Preview html={preview_5} />
                <Code code={code_5}/>
            </ybc::Section>
        </ybc::Container>
    }
}
