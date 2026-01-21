use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Fields, ItemStruct};

/// Атрибут макрос, который автоматически добавляет поле `refs: Vec<u8>` к структуре
///
/// Этот макрос генерирует новую структуру с добавленным полем `pub refs: Vec<u8>`
/// и вспомогательными методами для работы с ним.
///
/// # Пример
///
/// ```rust
/// #[with_refs]
/// struct MyStruct {
///     pub data: String,
/// }
///
/// // После применения макроса структура будет иметь поле refs: Vec<u8>
/// ```
#[proc_macro_attribute]
pub fn with_refs(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident; // Идентификатор структуры
    let generics = &input.generics; // Собираем generic параметры
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl(); // Разделяем generic параметры на impl, ty и where
    let vis = &input.vis; // Видимость структуры
    let attrs = &input.attrs; // Атрибуты структуры

    // Проверяем, что это структура с именованными полями
    let fields = match &input.fields {
        Fields::Named(fields) => fields,
        Fields::Unnamed(_) => {
            return syn::Error::new(
                input.ident.span(),
                "with_refs может быть применен только к структурам с именованными полями",
            )
            .to_compile_error()
            .into();
        }
        Fields::Unit => {
            return syn::Error::new(
                input.ident.span(),
                "with_refs может быть применен только к структурам с именованными полями",
            )
            .to_compile_error()
            .into();
        }
    };

    // Проверяем, не добавлено ли уже поле refs
    let has_refs_field = fields
        .named
        .iter()
        .any(|f| f.ident.as_ref().map(|i| i == "refs").unwrap_or(false));

    if has_refs_field {
        return syn::Error::new(
            input.ident.span(),
            "Поле 'refs' уже существует в структуре. Макрос with_refs добавляет его автоматически.",
        )
        .to_compile_error()
        .into();
    }

    // Собираем все существующие поля
    let struct_fields = fields.named.iter();
    let field_idents: Vec<_> = fields
        .named
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();

    // Фильтруем атрибуты: убираем with_refs, сохраняем остальные
    let filtered_attrs: Vec<&Attribute> = attrs
        .iter()
        .filter(|attr| {
            // Пропускаем сам атрибут with_refs
            !attr.path().is_ident("with_refs")
        })
        .collect();

    // Генерируем расширенную структуру с полем refs и impl блоки
    // Сохраняем все атрибуты, включая derive
    let expanded = quote! {
        #(#filtered_attrs)*
        #vis struct #name #ty_generics #where_clause {
            #(#struct_fields),*,
            pub refs: Vec<u8>,
        }

        impl #impl_generics #name #ty_generics #where_clause {
            /// Создает новый экземпляр с пустым вектором refs
            pub fn new() -> Self {
                Self {
                    #(#field_idents: Default::default()),*,
                    refs: Vec::new(),
                }
            }

            /// Возвращает ссылку на поле refs
            pub fn refs(&self) -> &Vec<u8> {
                &self.refs
            }

            /// Возвращает изменяемую ссылку на поле refs
            pub fn refs_mut(&mut self) -> &mut Vec<u8> {
                &mut self.refs
            }

            /// Добавляет значение в refs
            pub fn add_ref(&mut self, value: u8) {
                self.refs.push(value);
            }
        }

        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                Self {
                    #(#field_idents: Default::default()),*,
                    refs: Vec::new(),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
