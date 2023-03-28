use web_sys::window;

pub mod components;
pub mod elements;
pub mod form;
pub mod home;
pub mod layout;
pub mod page_not_found;

pub fn base_uri() -> String {
    window()
        .unwrap()
        .document()
        .unwrap()
        .base_uri()
        .unwrap()
        .unwrap_or_default()
        .to_string()
}
