use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(ColumnsGap)]
pub fn columns_gap() -> Html {
    let code_1 = r#"html! {
    <ybc::Columns>
        <ybc::Column>{"Default gap"}</ybc::Column>
        <ybc::Column>{"Default gap"}</ybc::Column>
        <ybc::Column>{"Default gap"}</ybc::Column>
        <ybc::Column>{"Default gap"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_1 = html! {
        <ybc::Columns>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Default gap"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Default gap"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Default gap"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Default gap"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let code_2 = r#"html! {
    <ybc::Columns classes={classes!("is-gapless")}>
        <ybc::Column>{"First column"}</ybc::Column>
        <ybc::Column>{"Second column"}</ybc::Column>
        <ybc::Column>{"Third column"}</ybc::Column>
        <ybc::Column>{"Fourth column"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_2 = html! {
        <ybc::Columns classes={classes!("is-gapless")}>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"First column"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Second column"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Third column"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Fourth column"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let code_3 = r#"html! {
    <ybc::Columns classes={classes!("is-gapless", "is-mobile")} multiline={true}>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-half")}>{"is-half"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column classes={classes!("is-one-quarter")}>{"is-one-quarter"}</ybc::Column>
        <ybc::Column>{"Auto"}</ybc::Column>
    </ybc::Columns>
}"#;
    let preview_3 = html! {
        <ybc::Columns classes={classes!("is-gapless", "is-mobile")} multiline={true}>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-half")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-half"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-one-quarter")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"is-one-quarter"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Auto"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
    };

    let code_4 = r#"
let selected = use_state_eq(|| 3usize);
let choices = vec![
    ("is-0", 0.0),
    ("is-1", 0.25),
    ("is-2", 0.5),
    ("is-3", 0.75),
    ("is-4", 1.0),
    ("is-5", 1.25),
    ("is-6", 1.5),
    ("is-7", 1.75),
    ("is-8", 2.0),
];
html! {
    <ybc::Columns>
        <ybc::Column classes={classes!("is-half")}>
            {"Gap: "}
            <ybc::Tag classes={classes!("is-warning")}>{format!("{}rem", choices[*selected].1)}</ybc::Tag>
        </ybc::Column>
        <ybc::Column classes={classes!("is-half")}>
            { 
                for choices.iter().enumerate().map(|(idx, (class, _))| {
                    html!{
                        <ybc::Tag
                          classes={
                              if *selected == idx {
                                  classes!("is-warning", "is-light")
                              } else {
                                  classes!("is-warning")
                              }
                          }
                          onclick={
                            let selected = selected.clone();
                            Callback::from(move |_| selected.set(idx))
                          }
                        >{class}</ybc::Tag>
                    }
                
                })
            }
        </ybc::Column>
    </ybc::Columns>
    <ybc::Columns classes={classes!(choices[*selected].0, "is-variable")}>
        <ybc::Column classes={classes!("is-3")}>{"Side"}</ybc::Column>
        <ybc::Column classes={classes!("is-9")}>{"Main"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns classes={classes!(choices[*selected].0, "is-variable")}>
        <ybc::Column classes={classes!("is-3")}>{"Three columns"}</ybc::Column>
        <ybc::Column classes={classes!("is-3")}>{"Three columns"}</ybc::Column>
        <ybc::Column classes={classes!("is-3")}>{"Three columns"}</ybc::Column>
    </ybc::Columns>
    <ybc::Columns classes={classes!(choices[*selected].0, "is-variable")}>
        {  
            for (0..12).map(|_| html! {
                <ybc::Column classes={classes!("is-1")}>
                </ybc::Column>
            
            })
        }
    </ybc::Columns>
}"#;

    let selected = use_state_eq(|| 3usize);
    let choices = vec![
        ("is-0", 0.0),
        ("is-1", 0.25),
        ("is-2", 0.5),
        ("is-3", 0.75),
        ("is-4", 1.0),
        ("is-5", 1.25),
        ("is-6", 1.5),
        ("is-7", 1.75),
        ("is-8", 2.0),
    ];

    let preview_4 = html! {
        <>
        <ybc::Columns>
            <ybc::Column classes={classes!("is-half")}>
                {"Gap: "}
                <ybc::Tag classes={classes!("is-warning")}>{format!("{}rem", choices[*selected].1)}</ybc::Tag>
            </ybc::Column>
            <ybc::Column classes={classes!("is-half")}>
                {
                    for choices.iter().enumerate().map(|(idx, (class, _))| {
                        html!{
                            <ybc::Tag
                              classes={
                                  if *selected == idx {
                                      classes!("is-warning", "is-light")
                                  } else {
                                      classes!("is-warning")
                                  }
                              }
                              onclick={
                                let selected = selected.clone();
                                Callback::from(move |_| selected.set(idx))
                              }
                            >{class}</ybc::Tag>
                        }

                    })
                }
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns classes={classes!(choices[*selected].0, "is-variable")}>
            <ybc::Column classes={classes!("is-3")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Side"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-9")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Main"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns classes={classes!(choices[*selected].0, "is-variable")}>
            <ybc::Column classes={classes!("is-4")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Three columns"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-4")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Three columns"}</ybc::Notification>
            </ybc::Column>
            <ybc::Column classes={classes!("is-4")}>
                <ybc::Notification
                  classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                >{"Three columns"}</ybc::Notification>
            </ybc::Column>
        </ybc::Columns>
        <ybc::Columns classes={classes!(choices[*selected].0, "is-variable")}>
            {
                for (0..12).map(|_| html! {
                    <ybc::Column classes={classes!("is-1")}>
                        <ybc::Notification
                          classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                        >{"1"}</ybc::Notification>
                    </ybc::Column>

                })
            }
        </ybc::Columns>
        </>
    };

    let code_5 = r#"html! {
    <ybc::Columns classes={classes!("is-variable", "is-1-mobile", "is-0-tablet", "is-3-desktop", "is-8-widescreen", "is-2-fullhd")}>
        {
            for (0..6).map(|_| html! {
                <ybc::Column>{"Column"}</ybc::Column>
            })
        }
    </ybc::Columns>
}"#;
    let preview_5 = html! {
        <ybc::Columns classes={classes!("is-variable", "is-1-mobile", "is-0-tablet", "is-3-desktop", "is-8-widescreen", "is-2-fullhd")}>
            {
                for (0..6).map(|_| html! {
                    <ybc::Column>
                        <ybc::Notification
                          classes={classes!("is-primary", "has-text-centered", "has-text-weight-bold")}
                        >{"Column"}</ybc::Notification>
                    </ybc::Column>

                })
            }
        </ybc::Columns>
    };

    let descriptions = vec![
        description::Item {
            title: "Default gap".to_owned(),
            id: "columns-gap-default".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Gapless".to_owned(),
            id: "columns-gap-gapless".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Gapless multiline".to_owned(),
            id: "columns-gap-gapless-multiline".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
        },
        description::Item {
            title: "Variable gap".to_owned(),
            id: "columns-gap-variable".to_owned(),
            code: code_4.to_owned(),
            html: Rc::new(preview_4),
        },
        description::Item {
            title: "Breakpoint based column gaps".to_owned(),
            id: "columns-gap-breakpoint".to_owned(),
            code: code_5.to_owned(),
            html: Rc::new(preview_5),
        },
    ];

    let api: Vec<description::Api> = vec![
        (
            "<ybc::Column>",
            "https://docs.rs/ybc/latest/ybc/struct.ColumnProps.html",
        )
            .into(),
        (
            "<ybc::Columns>",
            "https://docs.rs/ybc/latest/ybc/struct.ColumnsProps.html",
        )
            .into(),
    ];
    html! {
        <description::Description items={descriptions} {api} />
    }
}
