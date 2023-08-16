use std::str::FromStr;

use proc_macro::TokenStream;

pub fn generate(
    _attr: TokenStream,
    input: TokenStream,
) -> Result<proc_macro::TokenStream, syn::Error> {
    println!("input: {}", input.to_string());
    let mut as_string = input.to_string();

    let fn_name = extract_fn_name(as_string.as_str());

    let body_start = find_fn_body_start(as_string.as_str());

    let text_to_insert = format!(
        r#"let my_telemetry =
    my_grpc_extensions::get_telemetry(&request.metadata(), request.remote_addr(), "{}");"#,
        fn_name
    );

    as_string.insert_str(body_start, text_to_insert.as_str());

    println!("as_string: {}", as_string);

    let result = TokenStream::from_str(as_string.as_str()).unwrap();

    Ok(result)
}

fn extract_fn_name(content: &str) -> &str {
    let fn_start_index = content.find("fn");
    if fn_start_index.is_none() {
        panic!("Can not find fn keyword");
    }

    let fn_start_index = fn_start_index.unwrap();

    println!("fn_start_index: {}", fn_start_index);

    let fn_name_end = content.find("(");

    if fn_name_end.is_none() {
        panic!("Can not find start of function params");
    }

    let fn_name_end = fn_name_end.unwrap();
    println!("fn_name_end: {}", fn_name_end);

    content[fn_start_index + 2..fn_name_end].trim()
}

fn find_fn_body_start(content: &str) -> usize {
    let fn_body_start = content.find("{");
    if fn_body_start.is_none() {
        panic!("Can not find start of function body");
    }

    let fn_body_start = fn_body_start.unwrap();

    fn_body_start + 1
}
