#[cfg(feature = "utoipa")]
#[test]
fn test_openapi() {
    let openapi = crate::openapi::OpenApi::openapi();
    println!("{}", openapi.to_json());
}
