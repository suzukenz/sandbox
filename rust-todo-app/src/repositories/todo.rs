use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use validator::Validate;

use super::{label::Label, RepositoryError};

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity>;
    async fn find(&self, id: i32) -> anyhow::Result<TodoEntity>;
    async fn all(&self) -> anyhow::Result<Vec<TodoEntity>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct TodoFromRow {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, FromRow)]
pub struct TodoWithLabelFromRow {
    id: i32,
    title: String,
    completed: bool,
    label_id: Option<i32>,
    label_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TodoEntity {
    id: i32,
    title: String,
    completed: bool,
    pub labels: Vec<Label>,
}

fn fold_entities(rows: Vec<TodoWithLabelFromRow>) -> Vec<TodoEntity> {
    let mut rows = rows.iter();
    let mut accum: Vec<TodoEntity> = vec![];
    'outer: while let Some(row) = rows.next() {
        let todos = accum.iter_mut();
        for todo in todos {
            // idが一致=Todoに紐づくラベルが複数存在
            if todo.id == row.id {
                todo.labels.push(Label {
                    id: row.label_id.unwrap(),
                    name: row.label_name.clone().unwrap(),
                });
                continue 'outer;
            }
        }

        // Todoのidに一致がなかったのみ到達、TodoEntityを作成
        let labels = if row.label_id.is_some() {
            vec![Label {
                id: row.label_id.unwrap(),
                name: row.label_name.clone().unwrap(),
            }]
        } else {
            vec![]
        };
        accum.push(TodoEntity {
            id: row.id,
            title: row.title.clone(),
            completed: row.completed,
            labels,
        });
    }
    accum
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Can not be longer than 100 characters"))]
    title: String,
    labels: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Can not be longer than 100 characters"))]
    title: Option<String>,
    completed: Option<bool>,
    labels: Option<Vec<i32>>,
}

#[derive(Debug, Clone)]
pub struct TodoRepositoryForDb {
    pool: PgPool,
}

impl TodoRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDb {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity> {
        let tx = self.pool.begin().await?;
        let row = sqlx::query_as::<_, TodoFromRow>(
            r#"
            insert into todos (title, completed)
            values ($1, false)
            returning *;
            "#,
        )
        .bind(payload.title.clone())
        .fetch_one(&self.pool)
        .await?;

        sqlx::query(
            r#"
            insert into todo_labels (todo_id, label_id)
            select $1, id
            from unnest($2) as t(id);
            "#,
        )
        .bind(row.id)
        .bind(payload.labels)
        .execute(&self.pool)
        .await?;

        tx.commit().await?;

        let todo = self.find(row.id).await?;

        Ok(todo)
    }

    async fn find(&self, id: i32) -> anyhow::Result<TodoEntity> {
        let items = sqlx::query_as::<_, TodoWithLabelFromRow>(
            r#"
            select todos.*, labels.id as label_id, labels.name as label_name
            from todos
                        left outer join todo_labels tl on todos.id = tl.todo_id
                        left outer join labels on labels.id = tl.label_id
            where todos.id=$1;
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        let todos = fold_entities(items);
        let todo = todos.first().ok_or(RepositoryError::NotFound(id))?;
        Ok(todo.clone())
    }

    async fn all(&self) -> anyhow::Result<Vec<TodoEntity>> {
        let items = sqlx::query_as::<_, TodoWithLabelFromRow>(
            r#"
            select todos.*, labels.id as label_id, labels.name as label_name from todos
            left join todo_labels tl on tl.todo_id = todos.id
            left join labels on labels.id = tl.label_id
            order by todos.id desc
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(fold_entities(items))
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity> {
        let tx = self.pool.begin().await?;

        // todo update
        let old_todo = self.find(id).await?;
        sqlx::query(
            r#"
            update todos
            set title = coalesce($1, title),
                completed = coalesce($2, completed)
            where id = $3
            "#,
        )
        .bind(payload.title.unwrap_or(old_todo.title))
        .bind(payload.completed.unwrap_or(old_todo.completed))
        .bind(id)
        .execute(&self.pool)
        .await?;

        if let Some(labels) = payload.labels {
            // delete old labels
            sqlx::query(
                r#"
                delete from todo_labels where todo_id = $1
                "#,
            )
            .bind(id)
            .execute(&self.pool)
            .await?;

            // insert new labels
            sqlx::query(
                r#"
                insert into
                    todo_labels (todo_id, label_id)
                select $1, id from unnest($2) as t(id);
                "#,
            )
            .bind(id)
            .bind(labels)
            .execute(&self.pool)
            .await?;
        }

        tx.commit().await?;
        let todo = self.find(id).await?;

        Ok(todo)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let tx = self.pool.begin().await?;

        // todo's label delete
        sqlx::query(
            r#"
            delete from todo_labels where todo_id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        // todo delete
        sqlx::query(
            r#"
            delete from todos where id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        tx.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "database-test")]
mod test {
    // scenario testing for TodoRepositoryForDb
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;

    use super::*;

    #[test]
    fn fold_entities_test() {
        let label_1 = Label {
            id: 1,
            name: "label_1".to_string(),
        };
        let label_2 = Label {
            id: 2,
            name: "label_2".to_string(),
        };
        let rows = vec![
            TodoWithLabelFromRow {
                id: 1,
                title: "todo_1".to_string(),
                completed: false,
                label_id: Some(label_1.id),
                label_name: Some(label_1.name.clone()),
            },
            TodoWithLabelFromRow {
                id: 1,
                title: "todo_1".to_string(),
                completed: false,
                label_id: Some(label_2.id),
                label_name: Some(label_2.name.clone()),
            },
            TodoWithLabelFromRow {
                id: 2,
                title: "todo_2".to_string(),
                completed: false,
                label_id: Some(label_1.id),
                label_name: Some(label_1.name.clone()),
            },
        ];

        let res = fold_entities(rows);

        assert_eq!(
            res,
            vec![
                TodoEntity {
                    id: 1,
                    title: "todo_1".to_string(),
                    completed: false,
                    labels: vec![label_1.clone(), label_2],
                },
                TodoEntity {
                    id: 2,
                    title: "todo_2".to_string(),
                    completed: false,
                    labels: vec![label_1],
                }
            ]
        )
    }

    #[tokio::test]
    async fn crud_scenario() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        // label data prepare
        let label_name = "test label".to_string();
        let optional_label = sqlx::query_as::<_, Label>(
            r#"
            select * from labels where name = $1
            "#,
        )
        .bind(label_name.clone())
        .fetch_optional(&pool)
        .await
        .expect("Failed to fetch label");
        let label_1 = if let Some(label) = optional_label {
            label
        } else {
            sqlx::query_as::<_, Label>(
                r#"
                insert into labels (name) values ($1) returning *
                "#,
            )
            .bind(label_name.clone())
            .fetch_one(&pool)
            .await
            .expect("Failed to insert label")
        };

        let repository = TodoRepositoryForDb::new(pool);
        let todo_title = "[crud_scenario] todo";

        // create
        let created = repository
            .create(CreateTodo::new(todo_title.to_string(), vec![label_1.id]))
            .await
            .expect("create failed");
        assert_eq!(created.title, todo_title);
        assert!(!created.completed);
        assert_eq!(*created.labels.first().unwrap(), label_1);

        // find
        let todo = repository.find(created.id).await.expect("find failed");
        assert_eq!(todo, created);

        // all
        let todos = repository.all().await.expect("all failed");
        let todo = todos.first().unwrap();
        assert_eq!(*todo, created);

        // update
        let updated_title = "[crud_scenario] updated todo";
        let updated = repository
            .update(
                todo.id,
                UpdateTodo {
                    title: Some(updated_title.to_string()),
                    completed: Some(true),
                    labels: Some(vec![]),
                },
            )
            .await
            .expect("update failed");
        assert_eq!(updated.id, todo.id);
        assert_eq!(updated.title, updated_title);
        assert!(updated.completed);

        // delete
        repository.delete(todo.id).await.expect("delete failed");
        let result = repository.find(todo.id).await;
        assert!(result.is_err());

        let todo_rows = sqlx::query(
            r#"
            select * from todos where id = $1
            "#,
        )
        .bind(todo.id)
        .fetch_all(&repository.pool)
        .await
        .expect("todo_labels fetch error");
        let todo_labels_rows = sqlx::query(
            r#"
            select * from todo_labels where todo_id = $1
            "#,
        )
        .bind(todo.id)
        .fetch_all(&repository.pool)
        .await
        .expect("todo_labels fetch error");

        assert!(todo_rows.is_empty());
        assert!(todo_labels_rows.is_empty());
    }
}

#[cfg(test)]
pub mod test_utils {
    use anyhow::Context;
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

    use super::*;

    type TodoDatas = HashMap<i32, TodoEntity>;

    impl TodoEntity {
        pub fn new(id: i32, title: String, labels: Vec<Label>) -> Self {
            Self {
                id,
                title,
                completed: false,
                labels,
            }
        }

        pub fn set_completed(&mut self, completed: bool) {
            self.completed = completed;
        }
    }

    impl CreateTodo {
        pub fn new(title: String, label_ids: Vec<i32>) -> Self {
            Self {
                title,
                labels: label_ids,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct TodoRepositoryInMemory {
        store: Arc<RwLock<TodoDatas>>,
        labels: Vec<Label>,
    }

    impl TodoRepositoryInMemory {
        pub fn new(labels: Vec<Label>) -> Self {
            Self {
                store: Arc::default(),
                labels,
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
            self.store.read().unwrap()
        }

        fn resolve_labels(&self, label_ids: Vec<i32>) -> Vec<Label> {
            self.labels
                .iter()
                .filter(|label| label_ids.contains(&label.id))
                .cloned()
                .collect()
        }
    }

    #[async_trait]
    impl TodoRepository for TodoRepositoryInMemory {
        async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity> {
            let mut store = self.write_store_ref();
            let id = (store.len() + 1) as i32;
            let labels = self.resolve_labels(payload.labels);
            let todo = TodoEntity::new(id, payload.title, labels);
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn find(&self, id: i32) -> anyhow::Result<TodoEntity> {
            let store = self.read_store_ref();
            let todo = store
                .get(&id)
                .cloned()
                .ok_or(RepositoryError::NotFound(id))?;
            Ok(todo)
        }

        async fn all(&self) -> anyhow::Result<Vec<TodoEntity>> {
            let store = self.read_store_ref();
            Ok(Vec::from_iter(store.values().cloned()))
        }

        async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity> {
            let mut store = self.write_store_ref();
            let todo = store.get(&id).context(RepositoryError::NotFound(id))?;
            let title = payload.title.unwrap_or_else(|| todo.title.clone());
            let completed = payload.completed.unwrap_or(todo.completed);
            let labels = match payload.labels {
                Some(label_ids) => self.resolve_labels(label_ids),
                None => todo.labels.clone(),
            };
            let mut todo = TodoEntity::new(id, title, labels);
            todo.set_completed(completed);
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn delete(&self, id: i32) -> anyhow::Result<()> {
            let mut store = self.write_store_ref();
            store.remove(&id).context(RepositoryError::NotFound(id))?;
            Ok(())
        }
    }

    mod test {
        use super::*;

        #[tokio::test]
        async fn todo_crud_scenario() {
            let title = "todo title".to_string();
            let id = 1;
            let label_data = Label::new(1, "label title".to_string());
            let labels = vec![label_data.clone()];
            let expected = TodoEntity::new(id, title.clone(), labels.clone());

            // create
            let repository = TodoRepositoryInMemory::new(labels.clone());
            let todo = repository
                .create(CreateTodo::new(title, vec![label_data.id]))
                .await
                .expect("failed create todo");
            assert_eq!(todo, expected);

            // find
            let todo = repository.find(id).await.unwrap();
            assert_eq!(todo, expected);

            // all
            let todos = repository.all().await.expect("failed get all todos");
            assert_eq!(todos, vec![expected]);

            // update
            let title = "updated todo title".to_string();
            let todo = repository
                .update(
                    id,
                    UpdateTodo {
                        title: Some(title.clone()),
                        completed: Some(true),
                        labels: Some(vec![]),
                    },
                )
                .await
                .expect("failed update todo.");

            let mut expected = TodoEntity::new(id, title, vec![]);
            expected.set_completed(true);

            assert_eq!(todo, expected);

            // delete
            let res = repository.delete(id).await;
            assert!(res.is_ok());
        }
    }
}
