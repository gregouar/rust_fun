use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;

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

    let setting_fields = filter_fields_by_typename(fields, "ConfigSetting");

    let print_fields = setting_fields.map(|f| {
        let name = f.ident.as_ref().unwrap().to_string();
        let field = &f.ident;
        quote! {
            write!(f, "{} = {}\n", #name, &self.#field.value.to_string())?;
        }
    });

    let gen = quote! {
        impl fmt::Display for #struct_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                #(#print_fields)*
                Ok(())
            }
        }
        impl ConfigSection for #struct_name {
        }
    };
    gen.into()
}

pub fn impl_config(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = extract_named_fields_from_struct(ast, "Config");

    let section_fields = fields.iter();

    let print_fields = section_fields.map(|f| {
        let name = f.ident.as_ref().unwrap().to_string();
        let field = &f.ident;
        quote! {
            string_content.write_str(&format!("[{}]\n", #name))?;
            string_content.write_str(&format!("{}\n", &self.#field))?;
        }
    });

    let gen = quote! {
        impl Config for #struct_name {
            fn save_to_string(&self) -> DynResult<String> {
                let mut string_content = String::new();
                #(#print_fields)*
                Ok(string_content)
            }

            fn save_to_file(&self, filepath: &str) -> DynResult {
                Ok(fs::write(filepath, self.save_to_string()?)?)
            }

        }
    };
    gen.into()
}
