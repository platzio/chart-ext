#[cfg(feature = "utoipa")]
#[test]
fn test_openapi() {
    use utoipa::OpenApi;
    let openapi = platz_chart_ext::openapi::OpenApi::openapi();
    println!("{}", openapi.to_json().unwrap());
}
