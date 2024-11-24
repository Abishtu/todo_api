use poem_openapi::{payload::Json, Object, OpenApi};

pub struct TodoApi;
pub struct TodoListApi;

#[derive(Object)]
struct Todo {
    id: i64,
    created_at: i64,
    updated_at: i64,
    name: String,
    description: String,
    start_date: Option<i32>,
    end_date: Option<i32>,
    closed: bool,
}

#[derive(Object)]
struct TodoList {
    data: Vec<Todo>,
    total: i64,
}

#[OpenApi]
impl TodoListApi {
    #[oai(path = "/todo", method = "get")]
    async fn hello(&self) -> Json<TodoList> {
        let todoItem = Todo {
            id: 1,
            created_at: 1,
            updated_at: 1,
            name: String::from("Dummy todo"),
            description: String::from("This is a dummy todo for testing API"),
            start_date: None,
            end_date: None,
            closed: false,
        };

        let todo_list = TodoList {
            data: vec![todoItem],
            total: 1,
        };

        Json(todo_list)
    }
}
