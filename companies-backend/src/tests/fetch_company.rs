use crate::fetch_company::*;

// Uses tokio to execute async tasks in test
#[tokio::test]
async fn valid_id() {
    let id: i32 = 1;

    let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<Data>
	<id>1</id>
	<name>MWNZ</name>
	<description>..is awesome</description>
</Data>
"#;

    let body = try_fetch_company(id).await.unwrap();

    assert_eq!(body, expected);
}

#[tokio::test]
async fn invalid_id() {
    let id: i32 = -1000;

    match try_fetch_company(id).await {
        Ok(body) => panic!("Got a valid body\n{body}"),
        Err(e) => match e {
            Error::NotFound(_) => {} // Expected
            _ => panic!("Unexpected error result\n{e}"),
        },
    };
}
