use std::rc::Rc;

use crate::components::description;
use yew::prelude::*;

#[function_component(MediaObject)]
pub fn media_object() -> Html {
    let code_1 = r#"html! {
    <ybc::Media>
        <ybc::MediaLeft>
            <ybc::Image size={ybc::ImageSize::Is64x64}>
                <img src={"/images/placeholders/64x64.png"} />
            </ybc::Image>
        </ybc::MediaLeft>
        <ybc::MediaContent>
            <ybc::Content>
                <p>
                    <strong>{"Some user"}</strong> <small>{"@someuser"}</small> <small>{"42m"}</small>
                    <br />
                    {"This is media content"}
                </p>
            </ybc::Content>
        </ybc::MediaContent>
        <ybc::MediaRight>
            <ybc::Delete />
        </ybc::MediaRight>
    </ybc::Media>
}"#;
    let preview_1 = html! {
        <ybc::Media>
            <ybc::MediaLeft>
                <ybc::Image size={ybc::ImageSize::Is64x64}>
                    <img src={"/images/placeholders/64x64.png"} />
                </ybc::Image>
            </ybc::MediaLeft>
            <ybc::MediaContent>
                <ybc::Content>
                    <p>
                        <strong>{"Some user"}</strong> <small>{"@someuser"}</small> <small>{"42m"}</small>
                        <br />
                        {"This is media content"}
                    </p>
                </ybc::Content>
            </ybc::MediaContent>
            <ybc::MediaRight>
                <ybc::Delete />
            </ybc::MediaRight>
        </ybc::Media>
    };

    let code_2 = r#"html! {
    <ybc::Media>
        <ybc::MediaLeft>
            <ybc::Image size={ybc::ImageSize::Is64x64}>
                <img src={"/images/placeholders/64x64.png"} />
            </ybc::Image>
        </ybc::MediaLeft>
        <ybc::MediaContent>
            <ybc::Content>
                <p>
                    <strong>{"Some user"}</strong>
                    <br />
                    {"First message in first level"}
                </p>
            </ybc::Content>
            <ybc::Media>
                <ybc::MediaLeft>
                    <ybc::Image size={ybc::ImageSize::Is64x64}>
                        <img src={"/images/placeholders/64x64.png"} />
                    </ybc::Image>
                </ybc::MediaLeft>
                <ybc::MediaContent>
                    <ybc::Content>
                        <p>
                            <strong>{"Other user"}</strong>
                            <br />
                            {"First message in second level"}
                        </p>
                    </ybc::Content>
                    <ybc::Media>
                        <ybc::MediaLeft>
                            <ybc::Image size={ybc::ImageSize::Is64x64}>
                                <img src={"/images/placeholders/64x64.png"} />
                            </ybc::Image>
                        </ybc::MediaLeft>
                        <ybc::MediaContent>
                            <ybc::Content>
                                <p>
                                    <strong>{"Some user"}</strong>
                                    <br />
                                    {"First message in third level"}
                                </p>
                            </ybc::Content>
                        </ybc::MediaContent>
                    </ybc::Media>
                </ybc::MediaContent>
            </ybc::Media>
            <ybc::Media>
                <ybc::MediaLeft>
                    <ybc::Image size={ybc::ImageSize::Is64x64}>
                        <img src={"/images/placeholders/64x64.png"} />
                    </ybc::Image>
                </ybc::MediaLeft>
                <ybc::MediaContent>
                    <ybc::Content>
                        <p>
                            <strong>{"Other user"}</strong>
                            <br />
                            {"Second message in second level"}
                        </p>
                    </ybc::Content>
                </ybc::MediaContent>
            </ybc::Media>
        </ybc::MediaContent>
    </ybc::Media>
    <ybc::Media>
        <ybc::MediaLeft>
            <ybc::Image size={ybc::ImageSize::Is64x64}>
                <img src={"/images/placeholders/64x64.png"} />
            </ybc::Image>
        </ybc::MediaLeft>
        <ybc::MediaContent>
            <ybc::Content>
                <p>
                    <strong>{"Some user"}</strong>
                    <br />
                    {"Second message in first level"}
                </p>
            </ybc::Content>
        </ybc::MediaContent>
    </ybc::Media>
}"#;
    let preview_2 = html! {
        <>
        <ybc::Media>
            <ybc::MediaLeft>
                <ybc::Image size={ybc::ImageSize::Is64x64}>
                    <img src={"/images/placeholders/64x64.png"} />
                </ybc::Image>
            </ybc::MediaLeft>
            <ybc::MediaContent>
                <ybc::Content>
                    <p>
                        <strong>{"Some user"}</strong>
                        <br />
                        {"First message in first level"}
                    </p>
                </ybc::Content>
                <ybc::Media>
                    <ybc::MediaLeft>
                        <ybc::Image size={ybc::ImageSize::Is64x64}>
                            <img src={"/images/placeholders/64x64.png"} />
                        </ybc::Image>
                    </ybc::MediaLeft>
                    <ybc::MediaContent>
                        <ybc::Content>
                            <p>
                                <strong>{"Other user"}</strong>
                                <br />
                                {"First message in second level"}
                            </p>
                        </ybc::Content>
                        <ybc::Media>
                            <ybc::MediaLeft>
                                <ybc::Image size={ybc::ImageSize::Is64x64}>
                                    <img src={"/images/placeholders/64x64.png"} />
                                </ybc::Image>
                            </ybc::MediaLeft>
                            <ybc::MediaContent>
                                <ybc::Content>
                                    <p>
                                        <strong>{"Some user"}</strong>
                                        <br />
                                        {"First message in third level"}
                                    </p>
                                </ybc::Content>
                            </ybc::MediaContent>
                        </ybc::Media>
                    </ybc::MediaContent>
                </ybc::Media>
                <ybc::Media>
                    <ybc::MediaLeft>
                        <ybc::Image size={ybc::ImageSize::Is64x64}>
                            <img src={"/images/placeholders/64x64.png"} />
                        </ybc::Image>
                    </ybc::MediaLeft>
                    <ybc::MediaContent>
                        <ybc::Content>
                            <p>
                                <strong>{"Other user"}</strong>
                                <br />
                                {"Second message in second level"}
                            </p>
                        </ybc::Content>
                    </ybc::MediaContent>
                </ybc::Media>
            </ybc::MediaContent>
        </ybc::Media>
        <ybc::Media>
            <ybc::MediaLeft>
                <ybc::Image size={ybc::ImageSize::Is64x64}>
                    <img src={"/images/placeholders/64x64.png"} />
                </ybc::Image>
            </ybc::MediaLeft>
            <ybc::MediaContent>
                <ybc::Content>
                    <p>
                        <strong>{"Some user"}</strong>
                        <br />
                        {"Second message in first level"}
                    </p>
                </ybc::Content>
            </ybc::MediaContent>
        </ybc::Media>
        </>
    };

    let descriptions = vec![
        description::Item {
            title: "Media Object".to_owned(),
            id: "media-object".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
        description::Item {
            title: "Nested Media Object".to_owned(),
            id: "nested-media-object".to_owned(),
            code: code_2.to_owned(),
            html: Rc::new(preview_2),
        },
    ];

    html! {
        <description::Description items={descriptions} />
    }
}
