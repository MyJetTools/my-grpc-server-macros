use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::TokenStream;
use quote::ToTokens;

pub fn generate(
    _attr: TokenStream,
    input: TokenStream,
) -> Result<proc_macro::TokenStream, syn::Error> {
    let mut result: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut fn_is_engaged = false;
    let mut fn_name = None;
    let mut injection_is_done = false;

    for token in input.into_iter() {
        match &token {
            proc_macro::TokenTree::Ident(ident) => {
                let ident_string = ident.to_string();
                if !injection_is_done {
                    if fn_is_engaged {
                        fn_name = Some(ident_string);
                        fn_is_engaged = false;
                    } else if ident_string.as_str() == "fn" {
                        fn_is_engaged = true;
                    }
                }
            }
            proc_macro::TokenTree::Group(group) => {
                if !injection_is_done {
                    if let Some(fn_name) = &fn_name {
                        if let Delimiter::Brace = group.delimiter() {
                            inject_body(group);

                            //println!("Group: {}", group_as_text);

                            /*
                            result.push(quote::quote! {
                                let my_telemetry = my_grpc_extensions::get_telemetry(
                                    &request.metadata(),
                                    request.remote_addr(),
                                    #fn_name,
                                );
                            });
                             */
                            injection_is_done = true;
                        }
                    }
                }
            }

            _ => {}
        }

        let token_stream = TokenStream::from(token);
        let token_stream: proc_macro2::TokenStream = token_stream.into();

        result.push(token_stream);
    }

    println!("fn_name: {:?}", fn_name);

    /*
    let fn_name = extract_fn_name(as_string.as_str());

    let body_start = find_fn_body_start(as_string.as_str());

    let text_to_insert = format!(
        r#"let my_telemetry =
    my_grpc_extensions::get_telemetry(&request.metadata(), request.remote_addr(), "{}");"#,
        fn_name
    );

    as_string.insert_str(body_start, text_to_insert.as_str());

    println!("as_string: {}", as_string);

    let result = TokenStream::from_str(as_string.as_str()).unwrap(); */

    //Ok(ast.into_token_stream())
    let result = quote::quote! { #(#result)* };

    Ok(result.into())
}

fn inject_body(group: &Group) {
    for token in group.stream() {
        println!("token: {:#?}", token);
    }
}
