use std::rc::Rc;
use yew::prelude::*;

use super::{code::Code, preview::Preview};

#[derive(Debug, PartialEq)]
pub struct Api {
    name: String,
    url: String,
}

impl From<(&str, &str)> for Api {
    fn from(name_and_url: (&str, &str)) -> Self {
        let (name, url) = name_and_url;
        Self {
            name: name.to_owned(),
            url: url.to_owned(),
        }
    }
}

impl Api {
    fn render(&self) -> Html {
        html! {
            <li>
                <span class={classes!("icon-text")}>
                    <a href={self.url.to_owned()} target={"_blank"}>
                        <span>{&self.name}</span>
                        <ybc::Icon>
                            <i class="fas fa-up-right-from-square"></i>
                        </ybc::Icon>
                    </a>
                </span>
            </li>
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub title: String,
    pub id: String,
    pub code: String,
    pub html: Rc<Html>,
}

impl Item {
    pub fn render(&self) -> Html {
        html! {
            <ybc::Section>
                <ybc::Title>{&self.title}</ybc::Title>
                <Preview html={self.html.clone()} />
                <Code code={self.code.clone()}/>
            </ybc::Section>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub api: Vec<Api>,
    pub items: Vec<Item>,
}

#[function_component(Description)]
pub fn description(props: &Props) -> Html {
    html! {
        <ybc::Container>
            <ybc::Section classes={classes!("pb-0")}>
                <ybc::Title size={ybc::HeaderSize::Is2}>{"API"}</ybc::Title>
                <ybc::Content>
                <ul>
                { for props.api.iter().map(|e| e.render())}
                </ul>
                </ybc::Content>
            </ybc::Section>
            { for props.items.iter().map(|e| e.render())}
        </ybc::Container>
    }
}
