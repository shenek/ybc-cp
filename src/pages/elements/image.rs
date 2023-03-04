use crate::components::{code::Code, preview::Preview};
use yew::prelude::*;

#[function_component(Image)]
pub fn image() -> Html {
    let code_1 = r#"html! {
    <ybc::Image size={ybc::ImageSize::Is128x128}>
        <img src={"/images/placeholders/128x128.png"} />
    </ybc::Image>
}"#;
    let preview_1 = html! {
        <ybc::Image size={ybc::ImageSize::Is128x128}>
            <img src={"/images/placeholders/128x128.png"} />
        </ybc::Image>
    };

    let code_2 = r#"html! {
    <ybc::Image size={ybc::ImageSize::Is16x16}>
        <img src={"/images/placeholders/16x16.png"} />
    </ybc::Image>
    <ybc::Image size={ybc::ImageSize::Is24x24}>
        <img src={"/images/placeholders/24x24.png"} />
    </ybc::Image>
    <ybc::Image size={ybc::ImageSize::Is32x32}>
        <img src={"/images/placeholders/32x32.png"} />
    </ybc::Image>
    <ybc::Image size={ybc::ImageSize::Is48x48}>
        <img src={"/images/placeholders/48x48.png"} />
    </ybc::Image>
    <ybc::Image size={ybc::ImageSize::Is64x64}>
        <img src={"/images/placeholders/64x64.png"} />
    </ybc::Image>
    <ybc::Image size={ybc::ImageSize::Is96x96}>
        <img src={"/images/placeholders/96x96.png"} />
    </ybc::Image>
    <ybc::Image size={ybc::ImageSize::Is128x128}>
        <img src={"/images/placeholders/128x128.png"} />
    </ybc::Image>
}"#;
    let preview_2 = html! {
        <>
        <ybc::Image size={ybc::ImageSize::Is16x16}>
            <img src={"/images/placeholders/16x16.png"} />
        </ybc::Image>
        <ybc::Image size={ybc::ImageSize::Is24x24}>
            <img src={"/images/placeholders/24x24.png"} />
        </ybc::Image>
        <ybc::Image size={ybc::ImageSize::Is32x32}>
            <img src={"/images/placeholders/32x32.png"} />
        </ybc::Image>
        <ybc::Image size={ybc::ImageSize::Is48x48}>
            <img src={"/images/placeholders/48x48.png"} />
        </ybc::Image>
        <ybc::Image size={ybc::ImageSize::Is64x64}>
            <img src={"/images/placeholders/64x64.png"} />
        </ybc::Image>
        <ybc::Image size={ybc::ImageSize::Is96x96}>
            <img src={"/images/placeholders/96x96.png"} />
        </ybc::Image>
        <ybc::Image size={ybc::ImageSize::Is128x128}>
            <img src={"/images/placeholders/128x128.png"} />
        </ybc::Image>
        </>
    };

    let code_3 = r#"html! {
    <ybc::Image size={ybc::ImageSize::Is128x128}>
        <img
          classes={classes!("is-rounded")}
          src={"/images/placeholders/128x128.png"}
        />
    </ybc::Image>
}"#;
    let preview_3 = html! {
        <ybc::Image size={ybc::ImageSize::Is128x128}>
            <img
              class={classes!("is-rounded")}
              src={"/images/placeholders/128x128.png"}
            />
        </ybc::Image>
    };

    let code_4 = r#"html! {
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"1 by 1"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is1by1}>
                    <img src={"/images/placeholders/480x480.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"5 by 4"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is5by4}>
                    <img src={"/images/placeholders/600x480.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"4 by 3"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is4by3}>
                    <img src={"/images/placeholders/640x480.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"3 by 2"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is3by2}>
                    <img src={"/images/placeholders/480x320.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"5 by 3"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is5by3}>
                    <img src={"/images/placeholders/800x480.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"16 by 9"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is16by9}>
                    <img src={"/images/placeholders/640x360.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"2 by 1"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is2by1}>
                    <img src={"/images/placeholders/640x320.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"3 by 1"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is3by1}>
                    <img src={"/images/placeholders/720x240.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"4 by 5"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is4by5}>
                    <img src={"/images/placeholders/480x600.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"3 by 4"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is3by4}>
                    <img src={"/images/placeholders/480x640.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"2 by 3"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is2by3}>
                    <img src={"/images/placeholders/320x480.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"3 by 5"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is3by5}>
                    <img src={"/images/placeholders/480x800.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"9 by 16"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is9by16}>
                    <img src={"/images/placeholders/360x640.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"1 by 2"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is1by2}>
                    <img src={"/images/placeholders/320x640.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
    <ybc::Columns vcentered={true}>
            <ybc::Column>
                <ybc::Title classes={"has-text-centered"}>{"1 by 3"}</ybc::Title>
            </ybc::Column>
            <ybc::Column>
                <ybc::Image size={ybc::ImageSize::Is1by3}>
                    <img src={"/images/placeholders/240x720.png"} />
                </ybc::Image>
            </ybc::Column>
    </ybc::Columns>
}"#;
    let preview_4 = html! {
        <>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"1 by 1"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is1by1}>
                        <img src={"/images/placeholders/480x480.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"5 by 4"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is5by4}>
                        <img src={"/images/placeholders/600x480.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"4 by 3"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is4by3}>
                        <img src={"/images/placeholders/640x480.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"3 by 2"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is3by2}>
                        <img src={"/images/placeholders/480x320.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"5 by 3"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is5by3}>
                        <img src={"/images/placeholders/800x480.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"16 by 9"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is16by9}>
                        <img src={"/images/placeholders/640x360.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"2 by 1"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is2by1}>
                        <img src={"/images/placeholders/640x320.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"3 by 1"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is3by1}>
                        <img src={"/images/placeholders/720x240.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"4 by 5"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is4by5}>
                        <img src={"/images/placeholders/480x600.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"3 by 4"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is3by4}>
                        <img src={"/images/placeholders/480x640.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"2 by 3"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is2by3}>
                        <img src={"/images/placeholders/320x480.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"3 by 5"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is3by5}>
                        <img src={"/images/placeholders/480x800.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"9 by 16"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is9by16}>
                        <img src={"/images/placeholders/360x640.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"1 by 2"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is1by2}>
                        <img src={"/images/placeholders/320x640.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        <ybc::Columns vcentered={true}>
                <ybc::Column>
                    <ybc::Title classes={"has-text-centered"}>{"1 by 3"}</ybc::Title>
                </ybc::Column>
                <ybc::Column>
                    <ybc::Image size={ybc::ImageSize::Is1by3}>
                        <img src={"/images/placeholders/240x720.png"} />
                    </ybc::Image>
                </ybc::Column>
        </ybc::Columns>
        </>
    };

    html! {
        <ybc::Container>
            <ybc::Section>
                <ybc::Title>{"Image"}</ybc::Title>
                <Preview html={preview_1} />
                <Code code={code_1}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Image fixed sizes"}</ybc::Title>
                <Preview html={preview_2} />
                <Code code={code_2}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Image rounded"}</ybc::Title>
                <Preview html={preview_3} />
                <Code code={code_3}/>
            </ybc::Section>
            <ybc::Section>
                <ybc::Title>{"Image ratio"}</ybc::Title>
                <Preview html={preview_4} />
                <Code code={code_4}/>
            </ybc::Section>
        </ybc::Container>
    }
}
