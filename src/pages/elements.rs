use yew::prelude::*;

use crate::components::{code::Code, preview::Preview};

#[function_component(Notification)]
pub fn notification() -> Html {
    let code_1 = r#"html! {
    <ybc::Notification>
        {"This is Notification!"}
    </ybc::Notification>
}"#;
    let preview_1 = html! {
        <ybc::Notification>
            {"This is Notification!"}
        </ybc::Notification>
    };

    let code_2 = r#"html! {
    <>
    <ybc::Notification classes={classes!("is-primary")}>
        {"This is primary Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-link")}>
        {"This is link Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-info")}>
        {"This is info Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-success")}>
        {"This is success Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-warning")}>
        {"This is warning Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-danger")}>
        {"This is danger Notification!"}
    </ybc::Notification>
    </>
}"#;
    let preview_2 = html! {
        <>
        <ybc::Notification classes={classes!("is-primary")}>
            {"This is primary Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-link")}>
            {"This is link Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-info")}>
            {"This is info Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-success")}>
            {"This is success Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-warning")}>
            {"This is warning Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-danger")}>
            {"This is danger Notification!"}
        </ybc::Notification>
        </>
    };

    let code_3 = r#"html! {
    <>
    <ybc::Notification classes={classes!("is-primary", "is-light")}>
        {"This is light primary Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-link", "is-light")}>
        {"This is light link Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-info", "is-light")}>
        {"This is light info Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-success", "is-light")}>
        {"This is light success Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-warning", "is-light")}>
        {"This is light warning Notification!"}
    </ybc::Notification>
    <ybc::Notification classes={classes!("is-danger", "is-light")}>
        {"This is light danger Notification!"}
    </ybc::Notification>
    </>
}"#;
    let preview_3 = html! {
        <>
        <ybc::Notification classes={classes!("is-primary", "is-light")}>
            {"This is light primary Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-link", "is-light")}>
            {"This is light link Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-info", "is-light")}>
            {"This is light info Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-success", "is-light")}>
            {"This is light success Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-warning", "is-light")}>
            {"This is light warning Notification!"}
        </ybc::Notification>
        <ybc::Notification classes={classes!("is-danger", "is-light")}>
            {"This is light danger Notification!"}
        </ybc::Notification>
        </>
    };

    let code_4 = r#"let notification_active = use_state_eq(|| true);
let close_notification_cb = {
    let notification_active = notification_active.clone();
    Callback::from(move |_| {
        notification_active.set(false);
    })
    
};

html! {
    {
        if *notification_active {
            html! {
                <ybc::Notification>
                    <ybc::Delete tag={"button"} onclick={close_notification_cb} />
                    {"This is closable Notification!"}
                </ybc::Notification>
            }
        } else {
            html! {}
        }
    }
}"#;
    let notification_active = use_state_eq(|| true);
    let close_notification_cb = {
        let notification_active = notification_active.clone();
        Callback::from(move |_| {
            notification_active.set(false);
        })
    };
    let preview_4 = html! {
        {
            if *notification_active {
                html! {
                    <ybc::Notification>
                        <ybc::Delete tag={"button"} onclick={close_notification_cb} />
                        {"This is closable Notification!"}
                    </ybc::Notification>
                }
            } else {
                html! {}
            }
        }
    };

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Notification"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Notification colors"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Notification light colors"}</ybc::Title>
                <Preview html={preview_3} />
                <Code code={code_3}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Notification close"}</ybc::Title>
                <Preview html={preview_4} />
                <Code code={code_4}/>
            </ybc::Section>
        </ybc::Container>
    }
}

#[function_component(Block)]
pub fn block() -> Html {
    let code_1 = r#"html! {
    <ybc::Block>
        {"This is Block!"}
    </ybc::Block>
}"#;
    let preview_1 = html! {
        <ybc::Block>
            {"This is Block!"}
        </ybc::Block>
    };
    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Block"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
        </ybc::Container>
    }
}

#[function_component(Content)]
pub fn content() -> Html {
    let code_1 = r#"html! {
    <ybc::Content>
        <h1>{"This is Content!"}</h1>
        <p>{"First paragraph text"}</p>
        <h2>{"This is smaller content!"}</h2>
        <p>{"Second paragraph"}</p>
        <ul>
            <li>{"Item1"}</li>
            <li>{"Item2"}</li>
            <li>{"Item3"}</li>
            <li>{"Item4"}</li>
        </ul>
        <h3>{"This is even smaller content!"}</h3>
        <ol>
            <li>{"Item1"}</li>
            <li>{"Item2"}</li>
            <li>{"Item3"}</li>
            <li>{"Item4"}</li>
        </ol>
        <h3>{"This is even even smaller content!"}</h3>
        <dl>
            <dt>{"Title 1"}</dt>
            <dd>{"Item1"}</dd>
            <dt>{"Title 2"}</dt>
            <dd>{"Item2"}</dd>
            <dt>{"Title 3"}</dt>
            <dd>{"Item3"}</dd>
        </dl>
        <h4>{"This is even even even smaller content!"}</h4>
        <blockquote>
            {"Some longer text in quotes"}
        </blockquote>
        <h5>{"This is even even even even smaller content!"}</h5>
        <table>
            <thead>
                <tr>
                    <th>{"ONE"}</th>
                    <th>{"TWO"}</th>
                    <th>{"THREE"}</th>
                    <th>{"FOUR"}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{"1"}</td>
                    <td>{"2"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                </tr>
                <tr>
                    <td>{"11"}</td>
                    <td>{"22"}</td>
                    <td>{"33"}</td>
                    <td>{"44"}</td>
                </tr>
            </tbody>
        </table>
}"#;
    let preview_1 = html! {
        <ybc::Content>
            <h1>{"This is Content!"}</h1>
            <p>{"First paragraph text"}</p>
            <h2>{"This is smaller content!"}</h2>
            <p>{"Second paragraph"}</p>
            <ul>
                <li>{"Item1"}</li>
                <li>{"Item2"}</li>
                <li>{"Item3"}</li>
                <li>{"Item4"}</li>
            </ul>
            <h3>{"This is even smaller content!"}</h3>
            <ol>
                <li>{"Item1"}</li>
                <li>{"Item2"}</li>
                <li>{"Item3"}</li>
                <li>{"Item4"}</li>
            </ol>
            <h3>{"This is even even smaller content!"}</h3>
            <dl>
                <dt>{"Title 1"}</dt>
                <dd>{"Item1"}</dd>
                <dt>{"Title 2"}</dt>
                <dd>{"Item2"}</dd>
                <dt>{"Title 3"}</dt>
                <dd>{"Item3"}</dd>
            </dl>
            <h4>{"This is even even even smaller content!"}</h4>
            <blockquote>
                {"Some longer text in quotes"}
            </blockquote>
            <h5>{"This is even even even even smaller content!"}</h5>
            <table>
                <thead>
                    <tr>
                        <th>{"ONE"}</th>
                        <th>{"TWO"}</th>
                        <th>{"THREE"}</th>
                        <th>{"FOUR"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{"1"}</td>
                        <td>{"2"}</td>
                        <td>{"3"}</td>
                        <td>{"4"}</td>
                    </tr>
                    <tr>
                        <td>{"11"}</td>
                        <td>{"22"}</td>
                        <td>{"33"}</td>
                        <td>{"44"}</td>
                    </tr>
                </tbody>
            </table>
        </ybc::Content>
    };

    let code_2 = r#"html! {
    <ybc::Content>
        <ol class="is-lower-alpha">
            <li>{"L alpha 1"}</li>
            <li>{"L alpha 2"}</li>
            <li>{"L alpha 3"}</li>
        </ol>
        <ol class="is-lower-roman">
            <li>{"L roman 1"}</li>
            <li>{"L roman 2"}</li>
            <li>{"L roman 3"}</li>
        </ol>
        <ol class="is-upper-alpha">
            <li>{"U alpha 1"}</li>
            <li>{"U alpha 2"}</li>
            <li>{"U alpha 3"}</li>
        </ol>
        <ol class="is-upper-roman">
            <li>{"U roman 1"}</li>
            <li>{"U roman 2"}</li>
            <li>{"U roman 3"}</li>
        </ol>
    </ybc::Content>
}"#;
    let preview_2 = html! {
        <ybc::Content>
            <ol class="is-lower-alpha">
                <li>{"L alpha 1"}</li>
                <li>{"L alpha 2"}</li>
                <li>{"L alpha 3"}</li>
            </ol>
            <ol class="is-lower-roman">
                <li>{"L roman 1"}</li>
                <li>{"L roman 2"}</li>
                <li>{"L roman 3"}</li>
            </ol>
            <ol class="is-upper-alpha">
                <li>{"U alpha 1"}</li>
                <li>{"U alpha 2"}</li>
                <li>{"U alpha 3"}</li>
            </ol>
            <ol class="is-upper-roman">
                <li>{"U roman 1"}</li>
                <li>{"U roman 2"}</li>
                <li>{"U roman 3"}</li>
            </ol>
        </ybc::Content>
    };

    let code_3 = r#"html! {
    <>
    <ybc::Content classes={classes!("is-small")}>
        <h1>{"Is Small"}</h1>
        <p>{"Small text"}</p>
    </ybc::Content>
    <ybc::Content classes={classes!("is-normal")}>
        <h1>{"Is Normal"}</h1>
        <p>{"Normal text"}</p>
    </ybc::Content>
    <ybc::Content classes={classes!("is-medium")}>
        <h1>{"Is Medium"}</h1>
        <p>{"Medium text"}</p>
    </ybc::Content>
    <ybc::Content classes={classes!("is-large")}>
        <h1>{"Is Large"}</h1>
        <p>{"Large text"}</p>
    </ybc::Content>
    </>
}"#;
    let preview_3 = html! {
        <>
        <ybc::Content classes={classes!("is-small")}>
            <h1>{"Is Small"}</h1>
            <p>{"Small text"}</p>
        </ybc::Content>
        <ybc::Content classes={classes!("is-normal")}>
            <h1>{"Is Normal"}</h1>
            <p>{"Normal text"}</p>
        </ybc::Content>
        <ybc::Content classes={classes!("is-medium")}>
            <h1>{"Is Medium"}</h1>
            <p>{"Medium text"}</p>
        </ybc::Content>
        <ybc::Content classes={classes!("is-large")}>
            <h1>{"Is Large"}</h1>
            <p>{"Large text"}</p>
        </ybc::Content>
        </>
    };
    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Content"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Content ordered lists"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Content sizes"}</ybc::Title>
                <Preview html={preview_3} />
                <Code code={code_3}/>
            </ybc::Section>
        </ybc::Container>
    }
}
