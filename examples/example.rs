use duit_rs::{attributes, properties, widgets::{DuitWidget, WidgetJson}};
use value_ref_holder_macro::with_refs;

// Пример использования атрибут макроса #[with_refs]
// Этот макрос автоматически добавляет поле `pub refs: Vec<u8>` к структуре
#[with_refs]
#[derive(Debug, Clone)]
struct MyStruct {
    pub data: String,
    pub count: i32,
}

fn main() {
    // Использование структуры с атрибут макросом
    let mut my_struct = MyStruct::new();
    my_struct.data = "Hello".to_string();
    my_struct.count = 42;
    my_struct.add_ref(1);
    my_struct.add_ref(2);
    my_struct.add_ref(3);

    println!("MyStruct: {:?}", my_struct);
    println!("Refs: {:?}", my_struct.refs());

    // Пример 1: Использование временного значения (без явного биндинга)
    let container = DuitWidget::container(
        String::from("container"),
        false,
        DuitWidget::text(
            String::from("text"),
            false,
            attributes::TextAttributes::new().font_weight(properties::FontWeight::W300),
        ),
    );
    let widget_json: WidgetJson = container.into();
    let json = serde_json::to_string(&widget_json).unwrap();
    println!("Пример 1 (временное значение):\n{}", json);

    // Пример 2: Переиспользование атрибутов через ссылку
    let attr = attributes::TextAttributes::new().font_weight(properties::FontWeight::W300);
    let widget1 = DuitWidget::text(String::from("text1"), false, &attr);
    let widget2 = DuitWidget::text(String::from("text2"), false, &attr);
    let row = DuitWidget::row(String::from("row"), false, vec![widget1, widget2]);
    let widget_json2: WidgetJson = row.into();
    let json2 = serde_json::to_string(&widget_json2).unwrap();
    println!("\nПример 2 (переиспользование):\n{}", json2);
}
