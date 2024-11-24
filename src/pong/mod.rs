use poem_openapi::{payload::Json, Object, OpenApi};

pub struct PongApi;

#[derive(Object)]
struct Pong {
    message: String,
}

#[OpenApi]
impl PongApi {
    #[oai(path = "/ping", method = "get")]
    async fn pong(&self) -> Json<Pong> {
        Json(Pong {
            message: String::from("pong"),
        })
    }
}
