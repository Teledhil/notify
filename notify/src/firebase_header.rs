pub(crate) struct FirebaseHeader {}

impl FirebaseHeader {
    const CONTENT_TYPE: &'static str = "application/json";

    pub fn headers(auth_key: String) -> reqwest::header::HeaderMap {
        let authorization = format!("key={}", auth_key);

        let mut map = reqwest::header::HeaderMap::new();

        map.insert(
            reqwest::header::AUTHORIZATION,
            authorization.parse().unwrap(),
        );
        map.insert(
            reqwest::header::CONTENT_TYPE,
            Self::CONTENT_TYPE.parse().unwrap(),
        );

        map
    }
}
