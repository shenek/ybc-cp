use crate::components::{code::Code, preview::Preview};
use yew::prelude::*;

#[function_component(Tag)]
pub fn tag() -> Html {
    let code_1 = r#"html! {
    <ybc::Tag>{"This is Tag"}</ybc::Tag>
}"#;
    let preview_1 = html! {
        <ybc::Tag>{"This is Tag"}</ybc::Tag>
    };
    let code_2 = r#"html! {
    <ybc::Tag classes={classes!("is-black")}>{"This is black Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-dark")}>{"This is dark Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-light")}>{"This is light Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-white")}>{"This is white Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-primary")}>{"This is primary Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-link")}>{"This is link Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-info")}>{"This is info Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-success")}>{"This is success Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-warning")}>{"This is warning Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-danger")}>{"This is danger Tag"}</ybc::Tag>
}"#;
    let preview_2 = html! {
        <>
        <ybc::Tag classes={classes!("is-black", "m-1")}>{"This is black Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-dark", "m-1")}>{"This is dark Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-light", "m-1")}>{"This is light Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-white", "m-1")}>{"This is white Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-primary", "m-1")}>{"This is primary Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-link", "m-1")}>{"This is link Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-info", "m-1")}>{"This is info Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-success", "m-1")}>{"This is success Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-warning", "m-1")}>{"This is warning Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-danger", "m-1")}>{"This is danger Tag"}</ybc::Tag>
        </>
    };

    let code_3 = r#"html! {
    <ybc::Tag classes={classes!("is-primary", "is-light")}>{"This is light primary Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-link", "is-light")}>{"This is light link Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-info", "is-light")}>{"This is light info Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-success", "is-light")}>{"This is light success Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-warning", "is-light")}>{"This is light warning Tag"}</ybc::Tag>
    <ybc::Tag classes={classes!("is-danger", "is-light")}>{"This is light danger Tag"}</ybc::Tag>
}"#;
    let preview_3 = html! {
        <>
        <ybc::Tag classes={classes!("is-primary", "is-light", "m-1")}>{"This is light primary Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-link", "is-light", "m-1")}>{"This is light link Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-info", "is-light", "m-1")}>{"This is light info Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-success", "is-light", "m-1")}>{"This is light success Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-warning", "is-light", "m-1")}>{"This is light warning Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-danger", "is-light", "m-1")}>{"This is light danger Tag"}</ybc::Tag>
        </>
    };

    let code_4 = r#"html! {
    <ybc::Tag size={Some(ybc::Size::Small)}>{"This is small Tag"}</ybc::Tag>
    <ybc::Tag size={Some(ybc::Size::Normal)}>{"This is normal Tag"}</ybc::Tag>
    <ybc::Tag size={Some(ybc::Size::Medium)}>{"This is medium Tag"}</ybc::Tag>
    <ybc::Tag size={Some(ybc::Size::Large)}>{"This is large Tag"}</ybc::Tag>
    <ybc::Tags classes={classes!("are-small")}>
        <ybc::Tag>{"This is first small Tag"}</ybc::Tag>
        <ybc::Tag>{"This is second small Tag"}</ybc::Tag>
        <ybc::Tag>{"This is third small Tag"}</ybc::Tag>
    </ybc::Tags>
    <ybc::Tags classes={classes!("are-normal")}>
        <ybc::Tag>{"This is first normal Tag"}</ybc::Tag>
        <ybc::Tag>{"This is second normal Tag"}</ybc::Tag>
        <ybc::Tag>{"This is third normal Tag"}</ybc::Tag>
    </ybc::Tags>
    <ybc::Tags classes={classes!("are-medium")}>
        <ybc::Tag>{"This is first medium Tag"}</ybc::Tag>
        <ybc::Tag>{"This is second mediumTag"}</ybc::Tag>
        <ybc::Tag>{"This is third medium Tag"}</ybc::Tag>
    </ybc::Tags>
    <ybc::Tags classes={classes!("are-large")}>
        <ybc::Tag>{"This is first large Tag"}</ybc::Tag>
        <ybc::Tag>{"This is second large Tag"}</ybc::Tag>
        <ybc::Tag>{"This is third large Tag"}</ybc::Tag>
    </ybc::Tags>
}"#;
    let preview_4 = html! {
        <>
        <ybc::Tag classes={classes!("m-1")} size={Some(ybc::Size::Small)}>{"This is small Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("m-1")} size={Some(ybc::Size::Normal)}>{"This is normal Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("m-1")} size={Some(ybc::Size::Medium)}>{"This is medium Tag"}</ybc::Tag>
        <ybc::Tag classes={classes!("m-1")} size={Some(ybc::Size::Large)}>{"This is large Tag"}</ybc::Tag>
        <ybc::Tags classes={classes!("are-small")}>
            <ybc::Tag>{"This is first small Tag"}</ybc::Tag>
            <ybc::Tag>{"This is second small Tag"}</ybc::Tag>
            <ybc::Tag>{"This is third small Tag"}</ybc::Tag>
        </ybc::Tags>
        <ybc::Tags classes={classes!("are-normal")}>
            <ybc::Tag>{"This is first normal Tag"}</ybc::Tag>
            <ybc::Tag>{"This is second normal Tag"}</ybc::Tag>
            <ybc::Tag>{"This is third normal Tag"}</ybc::Tag>
        </ybc::Tags>
        <ybc::Tags classes={classes!("are-medium")}>
            <ybc::Tag>{"This is first medium Tag"}</ybc::Tag>
            <ybc::Tag>{"This is second mediumTag"}</ybc::Tag>
            <ybc::Tag>{"This is third medium Tag"}</ybc::Tag>
        </ybc::Tags>
        <ybc::Tags classes={classes!("are-large")}>
            <ybc::Tag>{"This is first large Tag"}</ybc::Tag>
            <ybc::Tag>{"This is second large Tag"}</ybc::Tag>
            <ybc::Tag>{"This is third large Tag"}</ybc::Tag>
        </ybc::Tags>
        </>
    };

    let code_5 = r#"html! {
    <ybc::Tags>
        <ybc::Tag rounded={true} classes={classes!("m-1")} tag={"span"}>{"This is rounded Tag"}</ybc::Tag>
        <ybc::Tag delete={true} classes={classes!("m-1")} tag={"a"}></ybc::Tag>
    </ybc::Tags>
    <ybc::Tags has_addons={true}>
        <ybc::Tag classes={classes!("is-dark")} tag={"span"}>{"dark"}</ybc::Tag>
        <ybc::Tag classes={classes!("is-primary")} tag={"span"}>{"primary"}</ybc::Tag>
    </ybc::Tags>
    <ybc::Tags has_addons={true}>
        <ybc::Tag rounded={true} classes={classes!("is-dark")} tag={"span"}>{"This is rounded Tag"}</ybc::Tag>
        <ybc::Tag rounded={true} delete={true} tag={"a"}></ybc::Tag>
    </ybc::Tags>
}"#;
    let preview_5 = html! {
        <>
        <ybc::Tags>
            <ybc::Tag rounded={true} classes={classes!("m-1")} tag={"span"}>{"This is rounded Tag"}</ybc::Tag>
            <ybc::Tag delete={true} classes={classes!("m-1")} tag={"a"}></ybc::Tag>
        </ybc::Tags>
        <ybc::Tags has_addons={true}>
            <ybc::Tag classes={classes!("is-dark")} tag={"span"}>{"dark"}</ybc::Tag>
            <ybc::Tag classes={classes!("is-primary")} tag={"span"}>{"primary"}</ybc::Tag>
        </ybc::Tags>
        <ybc::Tags has_addons={true}>
            <ybc::Tag rounded={true} classes={classes!("is-dark")} tag={"span"}>{"This is rounded Tag"}</ybc::Tag>
            <ybc::Tag rounded={true} delete={true} tag={"a"}></ybc::Tag>
        </ybc::Tags>
        </>
    };

    let code_6 = r#"let show_checked = use_state_eq(|| false);
let onclick = {
    let show_checked = show_checked.clone();
    Callback::from(move |_|{
        show_checked.set(!*show_checked)
    })  
};
html! {
    <ybc::Tags has_addons={true}>
        <ybc::Tag rounded={true} classes={classes!("is-info")}>{
            if *show_checked {
                "Checked"
             
            } else {
                "Un checked"
            }
        }</ybc::Tag>
        <ybc::Tag 
          rounded={true}
          classes={classes!("is-dark")}
          delete={true}
          tag={"a"}
          {onclick}
        />
    </ybc::Tags>
}"#;
    let show_checked = use_state_eq(|| false);
    let onclick = {
        let show_checked = show_checked.clone();
        Callback::from(move |_| show_checked.set(!*show_checked))
    };
    let preview_6 = html! {
        <ybc::Tags has_addons={true}>
            <ybc::Tag rounded={true} classes={classes!("is-info")}>{
                if *show_checked {
                    "Checked"

                } else {
                    "Un checked"
                }
            }</ybc::Tag>
            <ybc::Tag
              rounded={true}
              classes={classes!("is-dark")}
              delete={true}
              tag={"a"}
              {onclick}
            />
        </ybc::Tags>
    };

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Tag"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Tag colors"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Tag light colors"}</ybc::Title>
                <Preview html={preview_3} />
                <Code code={code_3}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Tag sizes"}</ybc::Title>
                <Preview html={preview_4} />
                <Code code={code_4}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Tag modifiers, combinations and addons"}</ybc::Title>
                <Preview html={preview_5} />
                <Code code={code_5}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Tag hooks"}</ybc::Title>
                <Preview html={preview_6} />
                <Code code={code_6}/>
            </ybc::Section>
        </ybc::Container>
    }
}
