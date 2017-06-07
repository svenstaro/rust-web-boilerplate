use serde_json;
use serde_json::Value;
use rocket::response::Response;
use rocket::http::Method;
use rocket::Rocket;
use rocket::testing::MockRequest;
use rocket::http::ContentType;

pub fn test_request_post<'a>(route: &str, data: &Value, rocket: &'a Rocket) -> Response<'a> {
    let mut req = MockRequest::new(Method::Post, route)
        .header(ContentType::JSON)
        .body(data.to_string());
    let res = req.dispatch_with(&rocket);
    res
}

pub fn parse_body_json(res: &mut Response) -> Value {
    let body_str = res.body().and_then(|b| b.into_string()).unwrap();
    serde_json::from_str(&body_str).expect("Can't parse JSON!")
}
