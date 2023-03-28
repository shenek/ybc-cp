
use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    let code_1 = r#"html! {
    <ybc::Menu>
        <ybc::MenuLabel text={"General"}></ybc::MenuLabel>
        <ybc::MenuList>
            <li><a>{"Dashboard"}</a></li>
            <li><a>{"Customers"}</a></li>
        </ybc::MenuList>
        <ybc::MenuLabel text={"Administration"}></ybc::MenuLabel>
        <ybc::MenuList>
            <li><a>{"Team Settings"}</a></li>
            <li>
                <a class={classes!("is-active")}>{"Manage Your Team"}</a>
                <ul>
                    <li><a>{"Members"}</a></li>
                    <li><a>{"Plugins"}</a></li>
                    <li><a>{"Add a member"}</a></li>
                </ul>
            </li>
            <li><a>{"Invitations"}</a></li>
            <li><a>{"Cloud Storage Environment Settings"}</a></li>
            <li><a>{"Authentication"}</a></li>
        </ybc::MenuList>
        <ybc::MenuLabel text={"Transactions"}></ybc::MenuLabel>
        <ybc::MenuList>
            <li><a>{"Payments"}</a></li>
            <li><a>{"Transfers"}</a></li>
            <li><a>{"Balance"}</a></li>
        </ybc::MenuList>
    </ybc::Menu>
}"#;
    let preview_1 = html! {
        <ybc::Menu>
            <ybc::MenuLabel text={"General"}></ybc::MenuLabel>
            <ybc::MenuList>
                <li><a>{"Dashboard"}</a></li>
                <li><a>{"Customers"}</a></li>
            </ybc::MenuList>
            <ybc::MenuLabel text={"Administration"}></ybc::MenuLabel>
            <ybc::MenuList>
                <li><a>{"Team Settings"}</a></li>
                <li>
                    <a class={classes!("is-active")}>{"Manage Your Team"}</a>
                    <ul>
                        <li><a>{"Members"}</a></li>
                        <li><a>{"Plugins"}</a></li>
                        <li><a>{"Add a member"}</a></li>
                    </ul>
                </li>
                <li><a>{"Invitations"}</a></li>
                <li><a>{"Cloud Storage Environment Settings"}</a></li>
                <li><a>{"Authentication"}</a></li>
            </ybc::MenuList>
            <ybc::MenuLabel text={"Transactions"}></ybc::MenuLabel>
            <ybc::MenuList>
                <li><a>{"Payments"}</a></li>
                <li><a>{"Transfers"}</a></li>
                <li><a>{"Balance"}</a></li>
            </ybc::MenuList>
        </ybc::Menu>
    };
    let descriptions = vec![
        description::Item {
            title: "Menu".to_owned(),
            id: "menu".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
    ];

    let api: Vec<description::Api> = vec![
        (
            "<ybc::Menu>",
            "https://docs.rs/ybc/latest/ybc/struct.MenuProps.html",
        )
            .into(),
        (
            "<ybc::MenuLabel>",
            "https://docs.rs/ybc/latest/ybc/struct.MenuLabelProps.html",
        )
            .into(),
        (
            "<ybc::MenuList>",
            "https://docs.rs/ybc/latest/ybc/struct.MenuListProps.html",
        )
            .into(),
    ];

    html! {
        <description::Description items={descriptions} {api} />
    }
}
