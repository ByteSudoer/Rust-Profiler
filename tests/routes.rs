use http::StatusCode;
use reqwest::blocking::Client;
use rstest::rstest;

fn return_status_code(url: &str) -> StatusCode {
    match Client::new().get(url).send() {
        Ok(response) => response.status(),
        Err(_) => StatusCode::NOT_FOUND,
    }
}

#[rstest]
#[case("http://127.0.0.1:3000/return_404", StatusCode::NOT_FOUND)]
#[case("http://127.0.0.1:3000/return_500", StatusCode::INTERNAL_SERVER_ERROR)]
#[case("http://127.0.0.1:3000/users_info", StatusCode::OK)]
#[case("http://127.0.0.1:3000/full_system", StatusCode::OK)]
#[case("http://127.0.0.1:3000/disk_info", StatusCode::OK)]
fn test_return_status_code(#[case] url: &str, #[case] expected_status: StatusCode) {
    assert_eq!(return_status_code(url), expected_status);
}

#[rstest]
#[case("http://127.0.0.1:3000/user_info/find_uid/0", StatusCode::OK)]
#[case("http://127.0.0.1:3000/user_info/find_uid/9789", StatusCode::NOT_FOUND)]
#[case("http://127.0.0.1:3000/user_info/find/root", StatusCode::OK)]
#[case(
    "http://127.0.0.1:3000/user_info/find/non_existant",
    StatusCode::NOT_FOUND
)]
fn test_return_status_code_query(#[case] url: &str, #[case] expected_status: StatusCode) {
    assert_eq!(return_status_code(url), expected_status);
}
