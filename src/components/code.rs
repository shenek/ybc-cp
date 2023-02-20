use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator, css_for_theme_with_class_style},
    parsing::SyntaxSet,
    util::LinesWithEndings,
    highlighting::ThemeSet,
};
use yew::prelude::*;

fn make_style() -> ClassStyle {
    ClassStyle::SpacedPrefixed{prefix:"__ybc_cp_"}
}

fn enrich_markup(input: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_name("Rust").unwrap();
    let style = make_style();
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, style);

    for line in LinesWithEndings::from(input) {
        let _ = html_generator.parse_html_for_line_which_includes_newline(line);
    }

    html_generator.finalize()
}

pub fn highlight_style() -> Html {
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["Solarized (light)"];

    let style = make_style();
    let css = css_for_theme_with_class_style(&theme, style).unwrap();
    let style = gloo_utils::document().create_element("style").unwrap();
    style.set_inner_html(&css);
    Html::VRef(style.into())
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub code: String,
}

#[function_component(Code)]
pub fn code(props: &Props) -> Html {
    let pre = gloo_utils::document().create_element("pre").unwrap();
    pre.set_inner_html(&enrich_markup(&props.code));
    Html::VRef(pre.into())
}
