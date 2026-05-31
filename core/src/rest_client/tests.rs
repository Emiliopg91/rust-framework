use crate::rest_client::{HttpMethod, RestClient, error::RestClientError};

#[tokio::test]
pub async fn test_01_get() {
    let response = RestClient::new("https://jsonplaceholder.typicode.com/todos/1")
        .invoke()
        .await
        .unwrap();
    assert!(response.status >= 200 && response.status < 400)
}

#[tokio::test]
pub async fn test_02_post() {
    let response = RestClient::new("https://jsonplaceholder.typicode.com/posts")
        .method(HttpMethod::POST)
        .header("Content-Type", "application/json")
        .unwrap()
        .body(
            r#"{
    "title": "Test post",
    "body": "This is a test post",
    "userId": 1
}"#,
        )
        .invoke()
        .await
        .unwrap();

    assert!(response.status >= 200 && response.status < 400)
}

#[test]
pub fn test_03_bad_header() {
    if let Some(e) = RestClient::new("https://jsonplaceholder.typicode.com/posts")
        .header("Content Type", "application/json")
        .err()
    {
        match e {
            RestClientError::InvalidRequestHeader(_, _, _) => {
                //OK!!
            }
            _ => {
                panic!("Expected InvalidRequestHeader error, {} found", e)
            }
        }
    } else {
        panic!("Expected InvalidRequestHeader error")
    }
}
