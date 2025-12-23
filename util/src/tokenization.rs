use std::cmp::Reverse;
use proc_macro2::TokenStream;
use quote::{quote};
use syn::{DeriveInput, Expr, Lit, Meta, MetaNameValue, Variant};

pub fn impl_tokenize(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let data = match input.data {
        syn::Data::Enum(data) => data,
        _ => panic!("tokenization only supported for enums"),
    };

    // the regex pattern for all characters that we will ignore
    let skip_binding = input.attrs.iter().filter_map(|a| {
        if !a.path().is_ident("skip") { return None }

        let pat = if let Meta::NameValue(MetaNameValue { value, .. }) = &a.meta {
            if let Expr::Lit(el) = value {
                if let Lit::Str(lit) = &el.lit {
                    lit.value()
                } else {
                    panic!("Expected string literal for {}", name);
                }
            } else {
                panic!("expected literal as pattern for variant {}", name)
            }
        } else {
            panic!("no literal pattern found for {}", name)
        };

        Some(pat)
    }).collect::<Vec<String>>();
    let skip = skip_binding.first().unwrap_or_else(|| panic!("No skip pattern provided"));

    // all string literals that match to a corresponding token variant
    let mut literal_patterns = data.variants.iter().filter_map(|v| {
        let pat = v.attrs.iter().find(|a| a.path().is_ident("literal"));

        if pat.is_none() { return None }

        let pat = if let Meta::NameValue(MetaNameValue { value, .. }) = &pat.unwrap().meta {
            if let Expr::Lit(el) = value {
                if let Lit::Str(lit) = &el.lit {
                    lit.value()
                } else {
                    panic!("Expected string literal for variant {} in {}", v.ident, name);
                }
            } else {
                panic!("expected literal as pattern for variant {} in {}", v.ident, name)
            }
        } else {
            panic!("no literal pattern found for variant {} in {}", v.ident, name)
        };

        Some((v, pat))
    }).collect::<Vec<(&Variant, String)>>();
    literal_patterns.sort_unstable_by_key(|(v, p)| Reverse(p.len()));

    // all regular expressions that match to a corresponding variant
    let mut regex_patterns = data.variants.iter().filter_map(|v| {
        let pat = v.attrs.iter().find(|a| a.path().is_ident("regex"));

        if pat.is_none() { return None }

        let pat = if let Meta::NameValue(MetaNameValue { value, .. }) = &pat.unwrap().meta {
            if let Expr::Lit(el) = value {
                if let Lit::Str(lit) = &el.lit {
                    lit.value()
                } else {
                    panic!("Expected string literal for variant {} in {}", v.ident, name);
                }
            } else {
                panic!("expected regular expression as pattern for variant {} in {}", v.ident, name)
            }
        } else {
            panic!("no regex pattern found for variant {} in {}", v.ident, name)
        };

        Some((v, pat))
    }).collect::<Vec<(&Variant, String)>>();
    regex_patterns.sort_unstable_by_key(|(v, p)| Reverse(p.len()));

    let mut generics = input.generics;
    generics.params.insert(0, syn::parse_quote!('lx));
    let (_impl_generics, _ty_generics, _where_clause) = generics.split_for_impl();

    // generate a check for each literal type
    let literal_checks = literal_patterns.iter().map(|(variant, pattern)| {
        let variant_ident = &variant.ident;
        quote! {
            if let Some(span) = lexer.match_literal(#pattern) {
                return Ok(Token {
                    kind: #name::#variant_ident,
                    raw : #pattern,
                    line, col,
                })
            }
        }
    });

    let regex_checks = regex_patterns.iter().map(|(variant, pattern)| {
        let variant_ident = &variant.ident;
        quote! {
            if let Some(span) = lexer.match_regex(#pattern) {
                return Ok(Token {
                    kind: #name::#variant_ident,
                    raw: lexer.get_span(&span).unwrap(),
                    line, col,
                })
            }
        }
    });

    let displays = data.variants.iter().map(|v| {
        let variant_ident = &v.ident;
        quote! {
            #name::#variant_ident => write!(f, "{}", stringify!(#variant_ident)),
        }
    });

    let stream = quote! {
        impl fmt::Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    #(#displays)*
                }
            }
        }

        /* we declare the lifetime explicitly right now,
         * but this only works on types with no lifetime params,
         * if we want to implement this for something with a lifetime
         * we need to do a bit more work
         */
        impl<'a> Tokenize<'a> for #name {
            fn next_token(lexer: &mut Lexer<'a>) -> Result<Token<'a>, CompileError> {
                let (line, col) = lexer.pos();
                lexer.match_regex(#skip);
                #(#literal_checks)*
                #(#regex_checks)*
                Err(CompileError::TokenizationError(format!("unexpected character found at line:{} col:{}", line, col)))
            }
        }
    };

    // eprintln!("{:#^72}\n{}\n{:#^72}", " Generated Code ", stream, "");
    stream
}