use crate::components::{code::Code, preview::Preview};
use yew::prelude::*;

#[function_component(Icon)]
pub fn icon() -> Html {
    let code_1 = r#"html! {
    <ybc::Icon>
        <i class="fas fa-home"></i>
    </ybc::Icon>
}"#;
    let preview_1 = html! {
        <ybc::Icon>
            <i class="fas fa-home"></i>
        </ybc::Icon>
    };

    let code_2 = r#"html! {
    <div class={classes!("icon-text")}>
        <ybc::Icon>
            <i class="fas fa-home"></i>
        </ybc::Icon>
        <span>{"Home"}</span>
    </div>
}"#;
    let preview_2 = html! {
        <div class={classes!("icon-text")}>
            <ybc::Icon>
                <i class="fas fa-home"></i>
            </ybc::Icon>
            <span>{"Home"}</span>
        </div>
    };

    let code_3 = r#"html! {
    <div class={classes!("icon-text")}>
        <ybc::Icon>
            <i class="fas fa-home"></i>
        </ybc::Icon>
        <span>{"Home"}</span>
        <ybc::Icon>
            <i class="fas fa-arrow-right"></i>
        </ybc::Icon>
        <span>{"Room"}</span>
        <ybc::Icon>
            <i class="fas fa-arrow-right"></i>
        </ybc::Icon>
        <span>{"Seat"}</span>
    </div>
}"#;
    let preview_3 = html! {
        <div class={classes!("icon-text")}>
            <ybc::Icon>
                <i class="fas fa-home"></i>
            </ybc::Icon>
            <span>{"Home"}</span>
            <ybc::Icon>
                <i class="fas fa-arrow-right"></i>
            </ybc::Icon>
            <span>{"Room"}</span>
            <ybc::Icon>
                <i class="fas fa-arrow-right"></i>
            </ybc::Icon>
            <span>{"Seat"}</span>
        </div>
    };

    let code_4 = r#"html! {
    <ybc::Icon classes={"has-text-info"}>
        <i class="fas fa-info-circle"></i>
    </ybc::Icon>
    <ybc::Icon classes={"has-text-success"}>
        <i class="fas fa-check-square"></i>
    </ybc::Icon>
    <ybc::Icon classes={"has-text-warning"}>
        <i class="fas fa-exclamation-triangle"></i>
    </ybc::Icon>
    <ybc::Icon classes={"has-text-danger"}>
        <i class="fas fa-ban"></i>
    </ybc::Icon>
}"#;
    let preview_4 = html! {
        <>
        <ybc::Icon classes={"has-text-info"}>
            <i class="fas fa-info-circle"></i>
        </ybc::Icon>
        <ybc::Icon classes={"has-text-success"}>
            <i class="fas fa-check-square"></i>
        </ybc::Icon>
        <ybc::Icon classes={"has-text-warning"}>
            <i class="fas fa-exclamation-triangle"></i>
        </ybc::Icon>
        <ybc::Icon classes={"has-text-danger"}>
            <i class="fas fa-ban"></i>
        </ybc::Icon>
        </>
    };

    let code_5 = r#"html! {
    <span class={classes!("icon-text", "has-text-info")}>
        <ybc::Icon>
            <i class="fas fa-info-circle"></i>
        </ybc::Icon>
        <span>{"Info"}</span>
    </span>
    <span class={classes!("icon-text", "has-text-success")}>
        <ybc::Icon>
            <i class="fas fa-check-square"></i>
        </ybc::Icon>
        <span>{"Success"}</span>
    </span>
    <span class={classes!("icon-text", "has-text-warning")}>
        <ybc::Icon>
            <i class="fas fa-exclamation-triangle"></i>
        </ybc::Icon>
        <span>{"Warning"}</span>
    </span>
    <span class={classes!("icon-text", "has-text-danger")}>
        <ybc::Icon>
            <i class="fas fa-ban"></i>
        </ybc::Icon>
        <span>{"Danger"}</span>
    </span>
}"#;
    let preview_5 = html! {
        <>
        <span class={classes!("icon-text", "has-text-info")}>
            <ybc::Icon>
                <i class="fas fa-info-circle"></i>
            </ybc::Icon>
            <span>{"Info"}</span>
        </span>
        <span class={classes!("icon-text", "has-text-success")}>
            <ybc::Icon>
                <i class="fas fa-check-square"></i>
            </ybc::Icon>
            <span>{"Success"}</span>
        </span>
        <span class={classes!("icon-text", "has-text-warning")}>
            <ybc::Icon>
                <i class="fas fa-exclamation-triangle"></i>
            </ybc::Icon>
            <span>{"Warning"}</span>
        </span>
        <span class={classes!("icon-text", "has-text-danger")}>
            <ybc::Icon>
                <i class="fas fa-ban"></i>
            </ybc::Icon>
            <span>{"Danger"}</span>
        </span>
        </>
    };

    let code_6 = r#"html! {
    <ybc::Block>
        <ybc::Icon classes={classes!("is-small", "has-background-warning")}>
            <i class={"fas fa-home"}></i>
        </ybc::Icon>
        <ybc::Icon classes={classes!("has-background-warning")}>
            <i class={"fas fa-home"}></i>
        </ybc::Icon>
        <ybc::Icon classes={classes!("is-medium", "has-background-warning")}>
            <i class={"fas fa-home"}></i>
        </ybc::Icon>
        <ybc::Icon classes={classes!("is-large", "has-background-warning")}>
            <i class={"fas fa-home"}></i>
        </ybc::Icon>
    </ybc::Block>
    <ybc::Block>
        <ybc::Icon classes={classes!("has-background-warning")}>
            <i class={"fas fa-lg fa-home"}></i>
        </ybc::Icon>
        <ybc::Icon classes={classes!("is-medium", "has-background-warning")}>
            <i class={"fas fa-home fa-lg"}></i>
        </ybc::Icon>
        <ybc::Icon classes={classes!("is-large", "has-background-warning")}>
            <i class={"fas fa-lg fa-home"}></i>
        </ybc::Icon>
    </ybc::Block>
    <ybc::Block>
        <ybc::Icon classes={classes!("is-medium", "has-background-warning")}>
            <i class={"fas fa-home fa-2x"}></i>
        </ybc::Icon>
        <ybc::Icon classes={classes!("is-large", "has-background-warning")}>
            <i class={"fas fa-2x fa-home"}></i>
        </ybc::Icon>
    </ybc::Block>
}"#;
    let preview_6 = html! {
        <>
        <ybc::Block>
            <ybc::Icon classes={classes!("is-small", "has-background-warning")}>
                <i class={"fas fa-home"}></i>
            </ybc::Icon>
            <ybc::Icon classes={classes!("has-background-warning")}>
                <i class={"fas fa-home"}></i>
            </ybc::Icon>
            <ybc::Icon classes={classes!("is-medium", "has-background-warning")}>
                <i class={"fas fa-home"}></i>
            </ybc::Icon>
            <ybc::Icon classes={classes!("is-large", "has-background-warning")}>
                <i class={"fas fa-home"}></i>
            </ybc::Icon>
        </ybc::Block>
        <ybc::Block>
            <ybc::Icon classes={classes!("has-background-warning")}>
                <i class={"fas fa-lg fa-home"}></i>
            </ybc::Icon>
            <ybc::Icon classes={classes!("is-medium", "has-background-warning")}>
                <i class={"fas fa-home fa-lg"}></i>
            </ybc::Icon>
            <ybc::Icon classes={classes!("is-large", "has-background-warning")}>
                <i class={"fas fa-lg fa-home"}></i>
            </ybc::Icon>
        </ybc::Block>
        <ybc::Block>
            <ybc::Icon classes={classes!("is-medium", "has-background-warning")}>
                <i class={"fas fa-home fa-2x"}></i>
            </ybc::Icon>
            <ybc::Icon classes={classes!("is-large", "has-background-warning")}>
                <i class={"fas fa-2x fa-home"}></i>
            </ybc::Icon>
        </ybc::Block>
        </>
    };

    let code_7 = r#"let rotating = use_state_eq(|| false);
let toggle_cb = {
    let rotating = rotating.clone();
    Callback::from(move |_| {
        rotating.set(!*rotating);
    })
};
html! {
    <ybc::Icon onclick={toggle_cb}>
        <i
          class={
              if *rotating {
                  "fa fa-pulse fa-basketball"
              } else {
                  "fa fa-basketball"
              }
          }
        ></i>
    </ybc::Icon>
}"#;
    let rotating = use_state_eq(|| false);
    let toggle_cb = {
        let rotating = rotating.clone();
        Callback::from(move |_| {
            rotating.set(!*rotating);
        })
    };
    let preview_7 = html! {
        <ybc::Icon onclick={toggle_cb}>
            <i
              class={
                  if *rotating {
                      "fa fa-pulse fa-basketball"
                  } else {
                      "fa fa-basketball"
                  }
              }
            ></i>
        </ybc::Icon>
    };

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Icon"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Icon text"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Icon texts"}</ybc::Title>
                <Preview html={preview_3} />
                <Code code={code_3}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Icon colors"}</ybc::Title>
                <Preview html={preview_4} />
                <Code code={code_4}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Icon text colors"}</ybc::Title>
                <Preview html={preview_5} />
                <Code code={code_5}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Icon sizes"}</ybc::Title>
                <Preview html={preview_6} />
                <Code code={code_6}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Icon onclick"}</ybc::Title>
                <Preview html={preview_7} />
                <Code code={code_7}/>
            </ybc::Section>
        </ybc::Container>
    }
}
