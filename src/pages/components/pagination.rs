use std::rc::Rc;

use yew::prelude::*;

use crate::{components::description, Route};

#[function_component(Pagination)]
pub fn pagination() -> Html {
    let code_1 = r#"let onclick = Callback::from(move |_| {});
html! {
    <ybc::Pagination
      next={html!{
        <ybc::PaginationItemRouter<Route>
          item_type={ybc::PaginationItemType::Next}
          route={Route::Pagination}
        >{"Next"}</ybc::PaginationItemRouter<Route>>
      }}
      previous={html!{
        <ybc::PaginationItemRouter<Route>
          item_type={ybc::PaginationItemType::Previous}
          route={Route::Pagination}
        >{"Previous"}</ybc::PaginationItemRouter<Route>>
      }}
    >
        <li>
            <ybc::PaginationItem
              item_type={ybc::PaginationItemType::Link}
              label={"Go to Page 1"}
              onclick={onclick.clone()}
            >{"1"}</ybc::PaginationItem>
        </li>
        <li>
            <ybc::PaginationEllipsis />
        </li>
        <li>
            <ybc::PaginationItem
              item_type={ybc::PaginationItemType::Link}
              label={"Go to Page 7"}
              onclick={onclick.clone()}
            >{"7"}</ybc::PaginationItem>
        </li>
        <li>
            <ybc::PaginationItem
              item_type={ybc::PaginationItemType::Link}
              label={"Go to Page 8"}
              onclick={onclick.clone()}
            >{"8"}</ybc::PaginationItem>
        </li>
        <li>
            <ybc::PaginationItem
              item_type={ybc::PaginationItemType::Link}
              label={"Go to Page 9"}
              onclick={onclick.clone()}
            >{"9"}</ybc::PaginationItem>
        </li>
        <li>
            <ybc::PaginationEllipsis />
        </li>
        <li>
            <ybc::PaginationItem
              item_type={ybc::PaginationItemType::Link}
              label={"Go to Page 99"}
              onclick={onclick.clone()}
            >{"99"}</ybc::PaginationItem>
        </li>
    </ybc::Pagination>
}"#;
    let onclick = Callback::from(move |_| {});
    let preview_1 = html! {
        <ybc::Pagination
          next={html!{
            <ybc::PaginationItemRouter<Route>
              item_type={ybc::PaginationItemType::Next}
              route={Route::Pagination}
            >{"Next"}</ybc::PaginationItemRouter<Route>>
          }}
          previous={html!{
            <ybc::PaginationItemRouter<Route>
              item_type={ybc::PaginationItemType::Previous}
              route={Route::Pagination}
            >{"Previous"}</ybc::PaginationItemRouter<Route>>
          }}
        >
            <li>
                <ybc::PaginationItem
                  item_type={ybc::PaginationItemType::Link}
                  label={"Go to Page 1"}
                  onclick={onclick.clone()}
                >{"1"}</ybc::PaginationItem>
            </li>
            <li>
                <ybc::PaginationEllipsis />
            </li>
            <li>
                <ybc::PaginationItem
                  item_type={ybc::PaginationItemType::Link}
                  label={"Go to Page 7"}
                  onclick={onclick.clone()}
                >{"7"}</ybc::PaginationItem>
            </li>
            <li>
                <ybc::PaginationItem
                  item_type={ybc::PaginationItemType::Link}
                  label={"Go to Page 8"}
                  onclick={onclick.clone()}
                >{"8"}</ybc::PaginationItem>
            </li>
            <li>
                <ybc::PaginationItem
                  item_type={ybc::PaginationItemType::Link}
                  label={"Go to Page 9"}
                  onclick={onclick.clone()}
                >{"9"}</ybc::PaginationItem>
            </li>
            <li>
                <ybc::PaginationEllipsis />
            </li>
            <li>
                <ybc::PaginationItem
                  item_type={ybc::PaginationItemType::Link}
                  label={"Go to Page 99"}
                  onclick={onclick.clone()}
                >{"99"}</ybc::PaginationItem>
            </li>
        </ybc::Pagination>
    };

    let descriptions = vec![
        description::Item {
            title: "Pagination 1".to_owned(),
            id: "pagination1".to_owned(),
            code: code_1.to_owned(),
            html: Rc::new(preview_1),
        },
    ];

    let api: Vec<description::Api> = vec![
        (
            "<ybc::Pagination>",
            "https://docs.rs/ybc/latest/ybc/struct.PaginationProps.html",
        )
            .into(),
        (
            "<ybc::PaginationItem>",
            "https://docs.rs/ybc/latest/ybc/struct.PaginationItemProps.html",
        )
            .into(),
        (
            "<ybc::PaginationEllipsis>",
            "https://docs.rs/ybc/latest/ybc/struct.PaginationEllipsis.html",
        )
            .into(),
    ];

    html! {
        <description::Description items={descriptions} {api} />
    }
}
