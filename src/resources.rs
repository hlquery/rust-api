use crate::error::{HlqueryError, Result};
use crate::request::Request;
use crate::response::Response;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

fn enc(value: &str) -> String {
    urlencoding::encode(value).into_owned()
}
fn method(value: &str, allowed: &[&str]) -> Result<String> {
    let value = value.to_uppercase();
    if allowed.contains(&value.as_str()) {
        Ok(value)
    } else {
        Err(HlqueryError::Unknown(format!(
            "Unsupported route method: {}",
            value
        )))
    }
}

macro_rules! service {
    ($name:ident) => {
        pub struct $name {
            request: Arc<Request>,
        }
        impl $name {
            pub fn new(request: Arc<Request>) -> Self {
                Self { request }
            }
        }
    };
}

service!(Synonyms);
impl Synonyms {
    pub async fn list_all(&self, query: Option<HashMap<String, String>>) -> Result<Response> {
        self.request.execute("GET", "/synonyms", None, query).await
    }
    pub async fn list(
        &self,
        collection: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.request
            .execute(
                "GET",
                &format!("/collections/{}/synonyms", enc(collection)),
                None,
                query,
            )
            .await
    }
    pub async fn get(&self, collection: &str, id: &str) -> Result<Response> {
        self.request
            .execute(
                "GET",
                &format!("/collections/{}/synonyms/{}", enc(collection), enc(id)),
                None,
                None,
            )
            .await
    }
    pub async fn upsert(
        &self,
        collection: &str,
        id: &str,
        body: Value,
        verb: &str,
    ) -> Result<Response> {
        let verb = method(verb, &["POST", "PUT"])?;
        self.request
            .execute(
                &verb,
                &format!("/collections/{}/synonyms/{}", enc(collection), enc(id)),
                Some(body),
                None,
            )
            .await
    }
    pub async fn delete(&self, collection: &str, id: &str) -> Result<Response> {
        self.request
            .execute(
                "DELETE",
                &format!("/collections/{}/synonyms/{}", enc(collection), enc(id)),
                None,
                None,
            )
            .await
    }
    pub async fn list_global(&self, query: Option<HashMap<String, String>>) -> Result<Response> {
        self.request
            .execute("GET", "/synonyms/global", None, query)
            .await
    }
    pub async fn get_global(&self, id: &str) -> Result<Response> {
        self.request
            .execute("GET", &format!("/synonyms/global/{}", enc(id)), None, None)
            .await
    }
    pub async fn upsert_global(&self, id: &str, body: Value, verb: &str) -> Result<Response> {
        let verb = method(verb, &["POST", "PUT"])?;
        self.request
            .execute(
                &verb,
                &format!("/synonyms/global/{}", enc(id)),
                Some(body),
                None,
            )
            .await
    }
    pub async fn delete_global(&self, id: &str) -> Result<Response> {
        self.request
            .execute(
                "DELETE",
                &format!("/synonyms/global/{}", enc(id)),
                None,
                None,
            )
            .await
    }
}

service!(Stopwords);
impl Stopwords {
    pub async fn list_all(&self, query: Option<HashMap<String, String>>) -> Result<Response> {
        self.request.execute("GET", "/stopwords", None, query).await
    }
    pub async fn list(
        &self,
        collection: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.request
            .execute(
                "GET",
                &format!("/collections/{}/stopwords", enc(collection)),
                None,
                query,
            )
            .await
    }
    pub async fn create(&self, collection: &str, body: Value) -> Result<Response> {
        self.request
            .execute(
                "POST",
                &format!("/collections/{}/stopwords", enc(collection)),
                Some(body),
                None,
            )
            .await
    }
    pub async fn delete(&self, collection: &str, word: &str) -> Result<Response> {
        self.request
            .execute(
                "DELETE",
                &format!("/collections/{}/stopwords/{}", enc(collection), enc(word)),
                None,
                None,
            )
            .await
    }
    pub async fn list_global(&self, query: Option<HashMap<String, String>>) -> Result<Response> {
        self.request
            .execute("GET", "/stopwords/global", None, query)
            .await
    }
    pub async fn create_global(&self, body: Value) -> Result<Response> {
        self.request
            .execute("POST", "/stopwords/global", Some(body), None)
            .await
    }
    pub async fn delete_global(&self, word: &str) -> Result<Response> {
        self.request
            .execute(
                "DELETE",
                &format!("/stopwords/global/{}", enc(word)),
                None,
                None,
            )
            .await
    }
}

service!(Overrides);
impl Overrides {
    fn path(collection: &str, id: Option<&str>) -> String {
        match id {
            Some(id) => format!("/collections/{}/overrides/{}", enc(collection), enc(id)),
            None => format!("/collections/{}/overrides", enc(collection)),
        }
    }
    pub async fn list(&self, c: &str, q: Option<HashMap<String, String>>) -> Result<Response> {
        self.request
            .execute("GET", &Self::path(c, None), None, q)
            .await
    }
    pub async fn get(&self, c: &str, id: &str) -> Result<Response> {
        self.request
            .execute("GET", &Self::path(c, Some(id)), None, None)
            .await
    }
    pub async fn upsert(&self, c: &str, id: &str, b: Value, verb: &str) -> Result<Response> {
        let verb = method(verb, &["POST", "PUT"])?;
        self.request
            .execute(&verb, &Self::path(c, Some(id)), Some(b), None)
            .await
    }
    pub async fn delete(&self, c: &str, id: &str) -> Result<Response> {
        self.request
            .execute("DELETE", &Self::path(c, Some(id)), None, None)
            .await
    }
}

service!(Aliases);
impl Aliases {
    pub async fn list(&self, q: Option<HashMap<String, String>>) -> Result<Response> {
        self.request.execute("GET", "/aliases", None, q).await
    }
    pub async fn list_collection(
        &self,
        c: &str,
        q: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.request
            .execute("GET", &format!("/collections/{}/aliases", enc(c)), None, q)
            .await
    }
    pub async fn get(&self, n: &str) -> Result<Response> {
        self.request
            .execute("GET", &format!("/aliases/{}", enc(n)), None, None)
            .await
    }
    pub async fn upsert(&self, n: &str, b: Value, verb: &str) -> Result<Response> {
        let verb = method(verb, &["POST", "PUT"])?;
        self.request
            .execute(&verb, &format!("/aliases/{}", enc(n)), Some(b), None)
            .await
    }
    pub async fn delete(&self, n: &str) -> Result<Response> {
        self.request
            .execute("DELETE", &format!("/aliases/{}", enc(n)), None, None)
            .await
    }
}

macro_rules! crud {
    ($name:ident,$root:literal) => {
        service!($name);
        impl $name {
            pub async fn list(&self, q: Option<HashMap<String, String>>) -> Result<Response> {
                self.request.execute("GET", $root, None, q).await
            }
            pub async fn create(&self, b: Value) -> Result<Response> {
                self.request.execute("POST", $root, Some(b), None).await
            }
            pub async fn get(&self, id: &str) -> Result<Response> {
                self.request
                    .execute("GET", &format!(concat!($root, "/{}"), enc(id)), None, None)
                    .await
            }
            pub async fn update(&self, id: &str, b: Value) -> Result<Response> {
                self.request
                    .execute(
                        "PUT",
                        &format!(concat!($root, "/{}"), enc(id)),
                        Some(b),
                        None,
                    )
                    .await
            }
            pub async fn delete(&self, id: &str) -> Result<Response> {
                self.request
                    .execute(
                        "DELETE",
                        &format!(concat!($root, "/{}"), enc(id)),
                        None,
                        None,
                    )
                    .await
            }
        }
    };
}
crud!(Users, "/users");
crud!(Keys, "/keys");

service!(Modules);
impl Modules {
    pub async fn list(&self) -> Result<Response> {
        self.request.execute("GET", "/modules", None, None).await
    }
    pub async fn load(&self, name: &str) -> Result<Response> {
        self.request
            .execute("POST", &format!("/loadmodule/{}", enc(name)), None, None)
            .await
    }
    pub async fn unload(&self, name: &str) -> Result<Response> {
        self.request
            .execute("POST", &format!("/unloadmodule/{}", enc(name)), None, None)
            .await
    }
    pub async fn syntax(&self, name: &str) -> Result<Response> {
        self.request
            .execute("GET", &format!("/modules/{}/syntax", enc(name)), None, None)
            .await
    }
    pub async fn call(
        &self,
        name: &str,
        route: &str,
        verb: &str,
        body: Option<Value>,
        query: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        let verb = method(verb, &["GET", "POST", "PUT", "DELETE"])?;
        let path = if route.is_empty() {
            format!("/modules/{}", enc(name))
        } else {
            format!("/modules/{}/{}", enc(name), route.trim_matches('/'))
        };
        self.request.execute(&verb, &path, body, query).await
    }
}

service!(Analytics);
impl Analytics {
    pub async fn click(&self, body: Value) -> Result<Response> {
        self.request
            .execute("POST", "/analytics/click", Some(body), None)
            .await
    }
}
