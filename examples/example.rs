use std::borrow::Cow;

use duit_rs::{
    attributes::*,
    properties,
    widgets::{ComponentDescription, DuitUIBuilder, DuitWidget},
};

const TEXT_WIDGET: DuitWidget<'static> = DuitWidget::Text {
    id: "LITERAL",
    controlled: false,
    attributes: Cow::Owned(
        TextAttributes::c_new()
            .c_data("Literal data")
            .c_text_direction(properties::TextDirection::Ltr),
    ),
};

fn main() {
    // Пример 1: Использование билдера
    let builder = DuitUIBuilder::new();

    let component_layout = DuitWidget::text(
        "text",
        false,
        TextAttributes::new()
            .font_weight(properties::FontWeight::W300)
            .data("Component layout"),
    );
    // let root = DuitWidget::container("root", false, ContainerAttributes::new(), None);
    let result = builder
        .add_component(ComponentDescription::new("some_one", component_layout))
        .add_widget("text".to_string(), TEXT_WIDGET)
        .build_string_pretty();

    match result {
        Ok(json) => println!("{}", json),
        Err(e) => println!("Error: {}", e),
    }
}
