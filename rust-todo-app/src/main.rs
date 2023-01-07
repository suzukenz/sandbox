mod handlers;
mod repositories;

use crate::repositories::{label::LabelRepositoryForDb, todo::TodoRepositoryForDb};
use axum::{
    extract::FromRef,
    routing::{delete, get, post},
    Router,
};
use dotenv::dotenv;
use handlers::{
    label::{all_label, create_label, delete_label, LabelState},
    todo::{all_todo, create_todo, delete_todo, find_todo, update_todo, TodoState},
};
use hyper::header::CONTENT_TYPE;
use repositories::{label::LabelRepository, todo::TodoRepository};
use sqlx::PgPool;
use std::{env, net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;
use tower_http::cors::{AllowOrigin, Any};

#[derive(Clone)]
struct AppState<T: TodoRepository, L: LabelRepository> {
    todo_state: TodoState<T>,
    label_state: LabelState<L>,
}

impl<T: TodoRepository, L: LabelRepository> FromRef<AppState<T, L>> for TodoState<T> {
    fn from_ref(state: &AppState<T, L>) -> TodoState<T> {
        state.todo_state.clone()
    }
}

impl<T: TodoRepository, L: LabelRepository> FromRef<AppState<T, L>> for LabelState<L> {
    fn from_ref(state: &AppState<T, L>) -> LabelState<L> {
        state.label_state.clone()
    }
}

impl<T: TodoRepository, L: LabelRepository> AppState<T, L> {
    fn new(todo_repository: T, label_repository: L) -> Self {
        Self {
            todo_state: TodoState {
                repository: Arc::new(todo_repository),
            },
            label_state: LabelState {
                repository: Arc::new(label_repository),
            },
        }
    }
}

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::debug!("starting connect database... (url: {})", database_url);
    let pool = PgPool::connect(database_url)
        .await
        .expect("failed connect database");

    // NOTE: with_state は create_routes 内に移せない
    // ref: https://github.com/tokio-rs/axum/issues/1592
    let app = create_routes().with_state(AppState::new(
        TodoRepositoryForDb::new(pool.clone()),
        LabelRepositoryForDb::new(pool.clone()),
    ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_routes<T: TodoRepository, L: LabelRepository>() -> Router<AppState<T, L>> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", post(create_todo::<T>).get(all_todo::<T>))
        .route(
            "/todos/:id",
            get(find_todo::<T>)
                .delete(delete_todo::<T>)
                .patch(update_todo::<T>),
        )
        .route("/labels", post(create_label::<L>).get(all_label::<L>))
        .route("/labels/:id", delete(delete_label::<L>))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact("http://localhost:3001".parse().unwrap()))
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
}

#[cfg(test)]
mod test {
    use axum::response::Response;
    use hyper::{header, Body, Method, Request, StatusCode};
    use tower::ServiceExt;

    use crate::repositories::label::{test_utils::LabelRepositoryInMemory, Label, LabelRepository};
    use crate::repositories::todo::{test_utils::TodoRepositoryInMemory, CreateTodo, TodoEntity};
    use crate::{create_routes, AppState};

    fn build_json_req(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body))
            .unwrap()
    }

    fn build_empty_req(path: &str, method: Method) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let app = create_routes().with_state(AppState::new(
            TodoRepositoryInMemory::new(vec![]),
            LabelRepositoryInMemory::new(),
        ));
        let res = app.oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, World!");
    }

    mod test_todo {
        use super::*;
        use crate::repositories::todo::TodoRepository;

        async fn res_to_todo(res: Response) -> TodoEntity {
            let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
            let body: String = String::from_utf8(bytes.to_vec()).unwrap();
            serde_json::from_str(&body)
                .unwrap_or_else(|_| panic!("cannot convert Todo instance. body: {}", body))
        }

        async fn res_to_label(res: Response) -> Label {
            let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
            let body: String = String::from_utf8(bytes.to_vec()).unwrap();
            let label: Label = serde_json::from_str(&body)
                .expect(&format!("cannot convert Label instance. body: {}", body));
            label
        }

        fn label_fixture() -> (Vec<Label>, Vec<i32>) {
            let id = 999;
            (
                vec![Label {
                    id,
                    name: String::from("test label"),
                }],
                vec![id],
            )
        }

        #[tokio::test]
        async fn should_create_todo() {
            let (labels, _label_ids) = label_fixture();
            let expected =
                TodoEntity::new(1, "should_return_crated_todo".to_string(), labels.clone());
            let req = build_json_req(
                "/todos",
                Method::POST,
                r#"{"title": "should_return_crated_todo", "labels": [999]}"#.to_string(),
            );
            let app = create_routes().with_state(AppState::new(
                TodoRepositoryInMemory::new(labels.clone()),
                LabelRepositoryInMemory::new(),
            ));
            let res = app.oneshot(req).await.unwrap();
            let todo = res_to_todo(res).await;
            assert_eq!(todo, expected);
        }

        #[tokio::test]
        async fn should_find_todo() {
            let (labels, label_ids) = label_fixture();
            let expected = TodoEntity::new(1, "should_find_todo".to_string(), labels.clone());

            let todo_repository = TodoRepositoryInMemory::new(labels.clone());
            todo_repository
                .create(CreateTodo::new("should_find_todo".to_string(), label_ids))
                .await
                .expect("failed to create todo");
            let req = build_empty_req("/todos/1", Method::GET);
            let app = create_routes().with_state(AppState::new(
                todo_repository,
                LabelRepositoryInMemory::new(),
            ));
            let res = app.oneshot(req).await.unwrap();
            let todo = res_to_todo(res).await;
            assert_eq!(todo, expected);
        }

        #[tokio::test]
        async fn should_get_all_todos() {
            let (labels, label_ids) = label_fixture();
            let expected = vec![TodoEntity::new(
                1,
                "should_get_all_todos".to_string(),
                labels.clone(),
            )];
            let todo_repository = TodoRepositoryInMemory::new(labels.clone());
            todo_repository
                .create(CreateTodo::new(
                    "should_get_all_todos".to_string(),
                    label_ids,
                ))
                .await
                .expect("failed to create todo");
            let req = build_empty_req("/todos", Method::GET);
            let app = create_routes().with_state(AppState::new(
                todo_repository,
                LabelRepositoryInMemory::new(),
            ));
            let res = app.oneshot(req).await.unwrap();
            let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
            let body: String = String::from_utf8(bytes.to_vec()).unwrap();
            let todos: Vec<TodoEntity> = serde_json::from_str(&body)
                .unwrap_or_else(|_| panic!("cannot convert Todo instance. body: {}", body));
            assert_eq!(todos, expected);
        }

        #[tokio::test]
        async fn should_update_todo() {
            let (labels, label_ids) = label_fixture();
            let mut expected = TodoEntity::new(1, "should_update_todo".to_string(), labels.clone());
            expected.set_completed(true);
            let todo_repository = TodoRepositoryInMemory::new(labels.clone());
            todo_repository
                .create(CreateTodo::new("before_update_todo".to_string(), label_ids))
                .await
                .expect("failed to create todo");
            let req = build_json_req(
                "/todos/1",
                Method::PATCH,
                r#"{
                    "id": 1,
                    "title": "should_update_todo",
                    "completed": true
                }"#
                .to_string(),
            );
            let app = create_routes().with_state(AppState::new(
                todo_repository,
                LabelRepositoryInMemory::new(),
            ));
            let res = app.oneshot(req).await.unwrap();
            let todo = res_to_todo(res).await;
            assert_eq!(todo, expected);
        }

        #[tokio::test]
        async fn should_delete_todo() {
            let (labels, label_ids) = label_fixture();
            let todo_repository = TodoRepositoryInMemory::new(labels);
            todo_repository
                .create(CreateTodo::new("should_delete_todo".to_string(), label_ids))
                .await
                .expect("failed to create todo");
            let req = build_empty_req("/todos/1", Method::DELETE);
            let app = create_routes().with_state(AppState::new(
                todo_repository,
                LabelRepositoryInMemory::new(),
            ));
            let res = app.oneshot(req).await.unwrap();
            assert_eq!(res.status(), StatusCode::NO_CONTENT);
        }
    }

    mod test_label {
        use super::*;

        async fn res_to_label(res: Response) -> Label {
            let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
            let body: String = String::from_utf8(bytes.to_vec()).unwrap();
            serde_json::from_str(&body)
                .unwrap_or_else(|_| panic!("cannot convert Label instance. body: {}", body))
        }

        fn build_app_state<T: LabelRepository>(
            repository: T,
        ) -> AppState<TodoRepositoryInMemory, T> {
            AppState::new(TodoRepositoryInMemory::new(vec![]), repository)
        }

        #[tokio::test]
        async fn should_create_label() {
            let expected = Label::new(1, "should_create_label".to_string());
            let req = build_json_req(
                "/labels",
                Method::POST,
                r#"{"name": "should_create_label"}"#.to_string(),
            );
            let app = create_routes().with_state(build_app_state(LabelRepositoryInMemory::new()));
            let res = app.oneshot(req).await.unwrap();
            let label = res_to_label(res).await;
            assert_eq!(label, expected);
        }

        #[tokio::test]
        async fn should_get_all_labels() {
            let label_name = "should_get_all_labels";
            let expected = vec![Label::new(1, label_name.to_string())];
            let repository = LabelRepositoryInMemory::new();
            repository
                .create(label_name.to_string())
                .await
                .expect("failed to create label");
            let req = build_empty_req("/labels", Method::GET);
            let app = create_routes().with_state(build_app_state(repository));
            let res = app.oneshot(req).await.unwrap();
            let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
            let body: String = String::from_utf8(bytes.to_vec()).unwrap();
            let labels: Vec<Label> = serde_json::from_str(&body)
                .unwrap_or_else(|_| panic!("cannot convert Label instance. body: {}", body));
            assert_eq!(labels, expected);
        }

        #[tokio::test]
        async fn should_delete_label() {
            let repository = LabelRepositoryInMemory::new();
            repository
                .create("should_delete_label".to_string())
                .await
                .expect("failed to create label");
            let req = build_empty_req("/labels/1", Method::DELETE);
            let app = create_routes().with_state(build_app_state(repository));
            let res = app.oneshot(req).await.unwrap();
            assert_eq!(res.status(), StatusCode::NO_CONTENT);
        }
    }
}
