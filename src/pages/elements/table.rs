use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(Table)]
pub fn table() -> Html {
    let code_1 = r##"html! {
    <ybc::Table>
        <thead>
            <tr>
                <th>{"#"}</th>
                <th>{"A"}</th>
                <th>{"B"}</th>
            </tr>
        </thead>
        <tfoot>
            <tr>
                <th>{"SUM"}</th>
                <th>{"8"}</th>
                <th>{"10"}</th>
            </tr>
        </tfoot>
        <tbody>
            <tr>
                <td>{"1"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
            </tr>
            <tr>
                <td>{"2"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
            </tr>
        </tbody>
    </ybc::Table>
}"##;
    let preview_1 = html! {
        <ybc::Table>
            <thead>
                <tr>
                    <th>{"#"}</th>
                    <th>{"A"}</th>
                    <th>{"B"}</th>
                </tr>
            </thead>
            <tfoot>
                <tr>
                    <th>{"SUM"}</th>
                    <th>{"8"}</th>
                    <th>{"10"}</th>
                </tr>
            </tfoot>
            <tbody>
                <tr>
                    <td>{"1"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                </tr>
                <tr>
                    <td>{"2"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                </tr>
            </tbody>
        </ybc::Table>
    };

    let code_2 = r##"html! {
    <ybc::Table bordered={true}>
        <thead>
            <tr>
                <th>{"#"}</th>
                <th>{"A"}</th>
                <th>{"B"}</th>
            </tr>
        </thead>
        <tfoot>
            <tr>
                <th>{"SUM"}</th>
                <th>{"8"}</th>
                <th>{"10"}</th>
            </tr>
        </tfoot>
        <tbody>
            <tr>
                <td>{"1"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
            </tr>
            <tr>
                <td>{"2"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
            </tr>
        </tbody>
    </ybc::Table>
}"##;
    let preview_2 = html! {
        <ybc::Table bordered={true}>
            <thead>
                <tr>
                    <th>{"#"}</th>
                    <th>{"A"}</th>
                    <th>{"B"}</th>
                </tr>
            </thead>
            <tfoot>
                <tr>
                    <th>{"SUM"}</th>
                    <th>{"8"}</th>
                    <th>{"10"}</th>
                </tr>
            </tfoot>
            <tbody>
                <tr>
                    <td>{"1"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                </tr>
                <tr>
                    <td>{"2"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                </tr>
            </tbody>
        </ybc::Table>
    };

    let code_3 = r##"html! {
    <ybc::Table striped={true}>
        <thead>
            <tr>
                <th>{"#"}</th>
                <th>{"A"}</th>
                <th>{"B"}</th>
            </tr>
        </thead>
        <tfoot>
            <tr>
                <th>{"SUM"}</th>
                <th>{"8"}</th>
                <th>{"10"}</th>
            </tr>
        </tfoot>
        <tbody>
            <tr>
                <td>{"1"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
            </tr>
            <tr>
                <td>{"2"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
            </tr>
        </tbody>
    </ybc::Table>
}"##;
    let preview_3 = html! {
        <ybc::Table striped={true}>
            <thead>
                <tr>
                    <th>{"#"}</th>
                    <th>{"A"}</th>
                    <th>{"B"}</th>
                </tr>
            </thead>
            <tfoot>
                <tr>
                    <th>{"SUM"}</th>
                    <th>{"8"}</th>
                    <th>{"10"}</th>
                </tr>
            </tfoot>
            <tbody>
                <tr>
                    <td>{"1"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                </tr>
                <tr>
                    <td>{"2"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                </tr>
            </tbody>
        </ybc::Table>
    };

    let code_4 = r##"html! {
    <ybc::Table narrow={true}>
        <thead>
            <tr>
                <th>{"#"}</th>
                <th>{"A"}</th>
                <th>{"B"}</th>
            </tr>
        </thead>
        <tfoot>
            <tr>
                <th>{"SUM"}</th>
                <th>{"8"}</th>
                <th>{"10"}</th>
            </tr>
        </tfoot>
        <tbody>
            <tr>
                <td>{"1"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
            </tr>
            <tr>
                <td>{"2"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
            </tr>
        </tbody>
    </ybc::Table>
}"##;
    let preview_4 = html! {
        <ybc::Table narrow={true}>
            <thead>
                <tr>
                    <th>{"#"}</th>
                    <th>{"A"}</th>
                    <th>{"B"}</th>
                </tr>
            </thead>
            <tfoot>
                <tr>
                    <th>{"SUM"}</th>
                    <th>{"8"}</th>
                    <th>{"10"}</th>
                </tr>
            </tfoot>
            <tbody>
                <tr>
                    <td>{"1"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                </tr>
                <tr>
                    <td>{"2"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                </tr>
            </tbody>
        </ybc::Table>
    };

    let code_5 = r##"html! {
    <ybc::Table hoverable={true}>
        <thead>
            <tr>
                <th>{"#"}</th>
                <th>{"A"}</th>
                <th>{"B"}</th>
            </tr>
        </thead>
        <tfoot>
            <tr>
                <th>{"SUM"}</th>
                <th>{"8"}</th>
                <th>{"10"}</th>
            </tr>
        </tfoot>
        <tbody>
            <tr>
                <td>{"1"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
            </tr>
            <tr>
                <td>{"2"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
            </tr>
        </tbody>
    </ybc::Table>
}"##;
    let preview_5 = html! {
        <ybc::Table hoverable={true}>
            <thead>
                <tr>
                    <th>{"#"}</th>
                    <th>{"A"}</th>
                    <th>{"B"}</th>
                </tr>
            </thead>
            <tfoot>
                <tr>
                    <th>{"SUM"}</th>
                    <th>{"8"}</th>
                    <th>{"10"}</th>
                </tr>
            </tfoot>
            <tbody>
                <tr>
                    <td>{"1"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                </tr>
                <tr>
                    <td>{"2"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                </tr>
            </tbody>
        </ybc::Table>
    };

    let code_6 = r##"html! {
    <ybc::Table fullwidth={true}>
        <thead>
            <tr>
                <th>{"#"}</th>
                <th>{"A"}</th>
                <th>{"B"}</th>
            </tr>
        </thead>
        <tfoot>
            <tr>
                <th>{"SUM"}</th>
                <th>{"8"}</th>
                <th>{"10"}</th>
            </tr>
        </tfoot>
        <tbody>
            <tr>
                <td>{"1"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
            </tr>
            <tr>
                <td>{"2"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
            </tr>
        </tbody>
    </ybc::Table>
}"##;
    let preview_6 = html! {
        <ybc::Table fullwidth={true}>
            <thead>
                <tr>
                    <th>{"#"}</th>
                    <th>{"A"}</th>
                    <th>{"B"}</th>
                </tr>
            </thead>
            <tfoot>
                <tr>
                    <th>{"SUM"}</th>
                    <th>{"8"}</th>
                    <th>{"10"}</th>
                </tr>
            </tfoot>
            <tbody>
                <tr>
                    <td>{"1"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                </tr>
                <tr>
                    <td>{"2"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                </tr>
            </tbody>
        </ybc::Table>
    };

    let code_7 = r##"html! {
    <ybc::Table scrollable={true}>
        <tbody>
            <tr>
                <td>{"1"}</td>
                <td>{"2"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
                <td>{"7"}</td>
                <td>{"8"}</td>
                <td>{"9"}</td>
                <td>{"0"}</td>
                <td>{"1"}</td>
                <td>{"2"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
                <td>{"7"}</td>
                <td>{"8"}</td>
                <td>{"9"}</td>
                <td>{"0"}</td>
                <td>{"1"}</td>
                <td>{"2"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
                <td>{"7"}</td>
                <td>{"8"}</td>
                <td>{"9"}</td>
                <td>{"0"}</td>
                <td>{"1"}</td>
                <td>{"2"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
                <td>{"7"}</td>
                <td>{"8"}</td>
                <td>{"9"}</td>
                <td>{"0"}</td>
            </tr>
            <tr>
                <td>{"0"}</td>
                <td>{"1"}</td>
                <td>{"2"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
                <td>{"7"}</td>
                <td>{"8"}</td>
                <td>{"9"}</td>
                <td>{"0"}</td>
                <td>{"1"}</td>
                <td>{"2"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
                <td>{"7"}</td>
                <td>{"8"}</td>
                <td>{"9"}</td>
                <td>{"0"}</td>
                <td>{"1"}</td>
                <td>{"2"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
                <td>{"7"}</td>
                <td>{"8"}</td>
                <td>{"9"}</td>
                <td>{"0"}</td>
                <td>{"1"}</td>
                <td>{"2"}</td>
                <td>{"3"}</td>
                <td>{"4"}</td>
                <td>{"5"}</td>
                <td>{"6"}</td>
                <td>{"7"}</td>
                <td>{"8"}</td>
                <td>{"9"}</td>
            </tr>
        </tbody>
    </ybc::Table>
}"##;
    let preview_7 = html! {
        <ybc::Table scrollable={true}>
            <tbody>
                <tr>
                    <td>{"1"}</td>
                    <td>{"2"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                    <td>{"7"}</td>
                    <td>{"8"}</td>
                    <td>{"9"}</td>
                    <td>{"0"}</td>
                    <td>{"1"}</td>
                    <td>{"2"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                    <td>{"7"}</td>
                    <td>{"8"}</td>
                    <td>{"9"}</td>
                    <td>{"0"}</td>
                    <td>{"1"}</td>
                    <td>{"2"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                    <td>{"7"}</td>
                    <td>{"8"}</td>
                    <td>{"9"}</td>
                    <td>{"0"}</td>
                    <td>{"1"}</td>
                    <td>{"2"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                    <td>{"7"}</td>
                    <td>{"8"}</td>
                    <td>{"9"}</td>
                    <td>{"0"}</td>
                </tr>
                <tr>
                    <td>{"0"}</td>
                    <td>{"1"}</td>
                    <td>{"2"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                    <td>{"7"}</td>
                    <td>{"8"}</td>
                    <td>{"9"}</td>
                    <td>{"0"}</td>
                    <td>{"1"}</td>
                    <td>{"2"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                    <td>{"7"}</td>
                    <td>{"8"}</td>
                    <td>{"9"}</td>
                    <td>{"0"}</td>
                    <td>{"1"}</td>
                    <td>{"2"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                    <td>{"7"}</td>
                    <td>{"8"}</td>
                    <td>{"9"}</td>
                    <td>{"0"}</td>
                    <td>{"1"}</td>
                    <td>{"2"}</td>
                    <td>{"3"}</td>
                    <td>{"4"}</td>
                    <td>{"5"}</td>
                    <td>{"6"}</td>
                    <td>{"7"}</td>
                    <td>{"8"}</td>
                    <td>{"9"}</td>
                </tr>
            </tbody>
        </ybc::Table>
    };

    let descriptions = vec![
        description::Item {
            title: "Table".to_owned(),
            id: "table".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Table bordered".to_owned(),
            id: "table-bordered".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
        description::Item {
            title: "Table striped".to_owned(),
            id: "table-striped".to_owned(),
            code: code_3.to_owned(),
            html: Rc::new(preview_3),
        },
        description::Item {
            title: "Table narrow".to_owned(),
            id: "table-narrow".to_owned(),
            code: code_4.to_owned(),
            html: Rc::new(preview_4),
        },
        description::Item {
            title: "Table hoverable".to_owned(),
            id: "table-hoverable".to_owned(),
            code: code_5.to_owned(),
            html: Rc::new(preview_5),
        },
        description::Item {
            title: "Table fullwidth".to_owned(),
            id: "table-fullwidth".to_owned(),
            code: code_6.to_owned(),
            html: Rc::new(preview_6),
        },
        description::Item {
            title: "Table scrollable".to_owned(),
            id: "table-scrollable".to_owned(),
            code: code_7.to_owned(),
            html: Rc::new(preview_7),
        },
    ];

    html! {
        <description::Description items={descriptions} />
    }
}
