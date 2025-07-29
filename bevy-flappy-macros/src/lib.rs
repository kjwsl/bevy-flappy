use quote::quote;

#[proc_macro]
pub fn hex_to_color(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lit_str = syn::parse_macro_input!(input as syn::LitStr);

    let color = lit_str.value();

    if !color.starts_with("#") {
        return syn::Error::new_spanned(lit_str, "Invalid color format; must start with #").to_compile_error().into();
    }

    if ![7, 9].contains(&color.len()) {
        return syn::Error::new_spanned(lit_str, "Invalid color format; must be 7 or 9 characters long").to_compile_error().into();
    }

    if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return syn::Error::new_spanned(lit_str, "Invalid color format; must be a hex color").to_compile_error().into();
    }

    let r = u8::from_str_radix(&color[1..=2], 16).unwrap();
    let g = u8::from_str_radix(&color[3..=4], 16).unwrap();
    let b = u8::from_str_radix(&color[5..=6], 16).unwrap();

    let new_tokens = if color.len() == 9 {
        let a = u8::from_str_radix(&color[7..=8], 16).unwrap();
        quote! { ::bevy::prelude::Color::srgba_u8(#r, #g, #b, #a) }
    } else {
        quote! { ::bevy::prelude::Color::srgb_u8(#r, #g, #b) }
    };

    new_tokens.into()
}
