use duit_rs::{
    attributes::*,
    properties,
    widgets::{DuitJsonWidget, DuitWidget},
};

// Пример конфигурации виджета, которая будет просчитана на этапе компиляции
const TEXT_WIDGET: DuitWidget<'static> = DuitWidget::c_text(
    "LITERAL",
    false,
    TextAttributes::c_new()
        .c_data("Literal data")
        .c_text_direction(properties::TextDirection::Ltr),
);

fn main() {
    // Пример 1: Использование рантайм варианта виджетов
    let container = DuitWidget::container(
        "123",
        false,
        ContainerAttributes::new(),
        TEXT_WIDGET, // const-виджеты могут быть использованы в рантайм-контексте
    );
    let widget_json: DuitJsonWidget = container.into();
    let json = serde_json::to_string_pretty(&widget_json).unwrap();
    println!("{}", json);

    // Пример 2: Переиспользование атрибутов через ссылку

    // Создаем атрибуты для виджетов, которые хотим переиспользовать
    let attr = &TextAttributes::new().font_weight(properties::FontWeight::W300);

    // Создаем виджеты, используя переиспользуемые атрибуты
    let widget1 = DuitWidget::text("text1", false, attr);
    let widget2 = DuitWidget::text("text2", false, attr);
    let widget3 = DuitWidget::text(
        "text3",
        true,
        TextAttributes::new().font_weight(properties::FontWeight::W300),
    );

    // Создаем multichild-виджет
    let row = DuitWidget::row("row", false, vec![widget1, widget2, widget3]);

    // Конвертируем в JSON
    let widget_json2: DuitJsonWidget = row.into();
    let json2 = serde_json::to_string_pretty(&widget_json2).unwrap();
    println!("{}", json2);
}
