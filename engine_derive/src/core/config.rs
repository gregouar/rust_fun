use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;

// TODO: Generate section hashmap at init

fn extract_named_fields_from_struct<'a>(
    ast: &'a syn::DeriveInput,
    struct_typename: &str,
) -> &'a Punctuated<syn::Field, syn::token::Comma> {
    match &ast.data {
        syn::Data::Struct(x) => match &x.fields {
            syn::Fields::Named(x) => &x.named,
            _ => unimplemented!("A {} must contains named fields", struct_typename),
        },
        _ => unimplemented!("A {} must be a struct", struct_typename),
    }
}

fn filter_fields_by_typename<'a>(
    fields: &'a Punctuated<syn::Field, syn::token::Comma>,
    typename: &'a str,
) -> impl Iterator<Item = &'a syn::Field> + 'a {
    fields.iter().filter(move |&field| match &field.ty {
        syn::Type::Path(type_path) => match type_path.path.segments.first() {
            Some(first_segment) => first_segment.ident == &typename,
            _ => false,
        },
        _ => false,
    })
}

pub fn impl_config_section(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = extract_named_fields_from_struct(&ast, "ConfigSection");

    let setting_fields: Vec<_> = filter_fields_by_typename(fields, "ConfigSetting").collect();
    let setting_field_idents: Vec<_> = setting_fields
        .iter()
        .map(|f| {
            let field = &f.ident;
            quote! {
                #field
            }
        })
        .collect();
    let setting_field_names: Vec<_> = setting_fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap().to_string();
            quote! {
                #name
            }
        })
        .collect();

    let gen = quote! {
        impl ConfigSection for #struct_name {
            fn load_setting_from_string(&mut self, raw_string: &str) {
                if let Some((setting_name, setting_value)) = engine::core::config::parse_setting(&raw_string) {
                    match setting_name.as_str() {
                        #(
                            #setting_field_names => self.#setting_field_idents.load_value_from_string(&setting_value),
                        )*
                        _ => {}
                    }
                }
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #(write!(f, "{} = {}\n", #setting_field_names, &self.#setting_field_idents.value.to_string())?;)*
                Ok(())
            }
        }
    };
    gen.into()
}

pub fn impl_config(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = extract_named_fields_from_struct(ast, "Config");

    let section_field_names = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap().to_string();
        quote! {
            #name
        }
    });

    let section_fields = fields.iter().map(|f| {
        let field = &f.ident;
        quote! {
            #field
        }
    });

    let print_fields = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap().to_string();
        let field = &f.ident;
        quote! {
            string_content.push_str(&format!("[{}]\n", #name));
            string_content.push_str(&format!("{}\n", &self.#field));
        }
    });

    let gen = quote! {
        impl Config for #struct_name {
            fn load_from_string(&mut self, raw_string: &str) -> DynResult {
                let mut cur_section: Option<&mut &mut dyn ConfigSection> = None;

                let mut section_map = std::collections::HashMap::<&str,&mut dyn ConfigSection>::new();
                #(section_map.insert(#section_field_names, &mut self.#section_fields);)*

                for string_content in raw_string.lines().map(|x| x.trim()) {
                    if let Some(section_name) = engine::core::config::parse_section_name(&string_content) {
                        cur_section = section_map.get_mut(section_name.as_str());
                    } else if let Some(section) = cur_section.as_mut() {
                        section.load_setting_from_string(string_content);
                    }
                }

                Ok(())
            }

            fn save_to_string(&self) -> DynResult<String> {
                let mut string_content = String::new();
                #(#print_fields)*
                Ok(string_content)
            }
        }
    };
    gen.into()
}
