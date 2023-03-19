use crate::components::{code::Code, preview::Preview};
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

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Media Object"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Nested Media Object"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
        </ybc::Container>
    }
}
