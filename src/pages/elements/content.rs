use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;
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

    let descriptions = vec![
        description::Item {
            title: "Content".to_owned(),
            id: "content".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Content ordered lists".to_owned(),
            id: "content-ordered-lists".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Content sizes".to_owned(),
            id: "content-sizes".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
        },
    ];

    let api: Vec<description::Api> = vec![(
        "<ybc::Content>",
        "https://docs.rs/ybc/latest/ybc/struct.ContentProps.html",
    )
        .into()];

    html! {
        <description::Description items={descriptions} {api} />
    }
}
