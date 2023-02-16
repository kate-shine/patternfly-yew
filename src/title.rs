use crate::ExtendClasses;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
pub enum Level {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl Default for Level {
    fn default() -> Self {
        Level::H1
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub level: Level,
    #[prop_or_default]
    pub size: Option<Size>,
}

#[function_component(Title)]
pub fn title(props: &Props) -> Html {
    let mut class = Classes::from("pf-c-title");

    class.extend_from(&props.size.unwrap_or_else(|| match props.level {
        Level::H1 => Size::XXLarge,
        Level::H2 => Size::XLarge,
        Level::H3 => Size::Large,
        Level::H4 => Size::Medium,
        Level::H5 => Size::Medium,
        Level::H6 => Size::Medium,
    }));

    let element = match props.level {
        Level::H1 => "h1",
        Level::H2 => "h2",
        Level::H3 => "h3",
        Level::H4 => "h4",
        Level::H5 => "h5",
        Level::H6 => "h6",
    };

    html! { <@{element} {class}>{ for props.children.iter() }</@> }
}
