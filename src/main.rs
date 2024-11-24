use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use todo_api::{api, pong, todo};

struct Api {
    some_text: String,
}

#[OpenApi]
impl Api {
    // Hello World
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<String> {
        let starting_string = String::from("Hello World ") + &self.some_text;
        PlainText(starting_string)
    }
}

#[tokio::main]
async fn main() {
    let user = match std::env::var("POSTGRES_USER") {
        Ok(env_var) => String::from(env_var),
        Err(_) => String::from(""),
    };

    let password = match std::env::var("POSTGRES_PASSWORD") {
        Ok(env_var) => String::from(env_var),
        Err(_) => String::from(""),
    };

    let port = match std::env::var("POSTGRES_PORT") {
        Ok(env_var) => String::from(env_var),
        Err(_) => String::from(""),
    };

    let db_name = match std::env::var("POSTGRES_DB") {
        Ok(env_var) => String::from(env_var),
        Err(_) => String::from(""),
    };

    let url = String::from("postgres://")
        + &user
        + &String::from(":")
        + &(password)
        + &String::from("@db:")
        + &(port)
        + &String::from("/")
        + &(db_name);
    println!("Connecting to DB");
    let pool = match sqlx::postgres::PgPool::connect(&url).await {
        Ok(p) => {
            println!("Postgres connection established");
            p
        }
        Err(err) => panic!("Error::Could not connect to DB\n\t{:?}", err),
    };

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => {
            println!("Data migration successful");
        }
        Err(err) => {
            eprintln!("Could not complete data migrations::\n\t{}", err);
        }
    };

    let endpoints = (
        Api {
            some_text: String::from("Foo World"),
        },
        pong::PongApi,
        todo::TodoListApi,
        api::task::TaskApi {
            db_pool: Box::new(pool.clone()),
        },
        api::task_status::TaskStatusApi {
            db_pool: Box::new(pool.clone()),
        },
        api::task_history::TaskHistoryApi {
            db_pool: Box::new(pool.clone()),
        },
    );
    let api_service =
        OpenApiService::new(endpoints, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    println!("Strting Server on 0.0.0.0:3000");
    let _ = Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await;
}