use std::rc::Rc;
use yew::prelude::*;

use super::{code::Code, preview::Preview};

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
    pub items: Vec<Item>,
}


#[function_component(Description)]
pub fn description(props: &Props) -> Html {
    html! {
        <ybc::Container>
            { for props.items.iter().map(|e| e.render())}
        </ybc::Container>
    }
}
