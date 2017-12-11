
pub fn generate_error(error: &str) -> String {
    return object!{
        "type" => "error",
        "data" => object!{
            "error" => error,
        },
    }.dump();
}
