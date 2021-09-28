use hyper::{Body, Client, Error, Method, Request};
use url::Url;
use uuid::Uuid;

pub enum Hit {
    Pageview,
    Screenview,
    Event,
    Transaction,
    Item,
    Social,
    Exception,
    Timing
}

impl Hit {
    fn value(&self) -> String {
        match *self {
            Hit::Pageview => "pageview".to_string(),
            Hit::Screenview => "screenview".to_string(),
            Hit::Event => "event".to_string(),
            Hit::Transaction => "transaction".to_string(),
            Hit::Item => "item".to_string(),
            Hit::Social => "social".to_string(),
            Hit::Exception => "exception".to_string(),
            Hit::Timing => "timing".to_string()
        }
    }
}

pub struct GoogleAnalyticsHit {
    tracking_id: String
}

pub struct GoogleAnalyticsHitOptions {
    pub version: String,
    pub event_type: Hit,
    pub client_id: String,
    pub event_category: Option<String>,
    pub event_action: Option<String>,
    pub event_label: Option<String>,
    pub event_value: Option<String>
}

impl Default for GoogleAnalyticsHitOptions {
    fn default() -> Self { 
        Self {
            version: "1".to_string(),
            event_type: Hit::Event,
            client_id: Uuid::new_v4().to_string(),
            event_category: None,
            event_action: None,
            event_label: None,
            event_value: None
        }
    }
}

impl GoogleAnalyticsHit {
    pub fn new(tracking_id: String) -> Self {
        Self {
            tracking_id
        }
    }

    pub async fn send(&self, input: GoogleAnalyticsHitOptions) -> Result<(), Error> {
        let mut payload = vec![
            ("v".to_string(), input.version),
            ("tid".to_string(), self.tracking_id.to_string()),
            ("t".to_string(), input.event_type.value()),
            ("cid".to_string(), input.client_id)
        ];

        if input.event_category.is_some() {
            payload.push(("ec".to_string(), input.event_category.unwrap().to_string()));
        }

        if input.event_action.is_some() {
            payload.push(("ea".to_string(), input.event_action.unwrap().to_string()));
        }
        
        if input.event_label.is_some() {
            payload.push(("el".to_string(), input.event_label.unwrap().to_string()));
        }

        if input.event_value.is_some() {
            payload.push(("ev".to_string(), input.event_value.unwrap().to_string()));
        }
        
        let uri = Url::parse_with_params("http://www.google-analytics.com/collect", &payload).unwrap();
        let client = Client::new();
        let request = Request::builder()
                                    .method(Method::POST)
                                    .uri(uri.as_str())
                                    .header("Content-Length", 0)
                                    .body(Body::empty())
                                    .unwrap();
        let response = client.request(request).await;
        match response {
            Ok(_) => {
                Ok(())
            },
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{GoogleAnalyticsHit, GoogleAnalyticsHitOptions};

    #[tokio::test]
    async fn it_works() {
        let ga_hit = GoogleAnalyticsHit::new("UA-xxxxxxxx-x".to_string());
        ga_hit.send(GoogleAnalyticsHitOptions {
            event_category: Some("test".to_string()),
            event_action: Some("test".to_string()),
            event_label: Some("test".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert!(true);
    }
}
