use proc_macro::TokenStream;

mod with_telemetry;

#[proc_macro_attribute]
pub fn generate_grpc_client(attr: TokenStream, item: TokenStream) -> TokenStream {
    match crate::with_telemetry::generate(attr, item) {
        Ok(result) => result,
        Err(err) => err.into_compile_error().into(),
    }
}
