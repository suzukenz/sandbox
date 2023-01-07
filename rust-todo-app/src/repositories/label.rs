use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use super::RepositoryError;

#[async_trait]
pub trait LabelRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, name: String) -> anyhow::Result<Label>;
    async fn all(&self) -> anyhow::Result<Vec<Label>>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Label {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateLabel {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Can not be longer than 100 characters"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateLabel {
    id: i32,
    name: String,
}

#[derive(Debug, Clone)]
pub struct LabelRepositoryForDb {
    pool: PgPool,
}

impl LabelRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LabelRepository for LabelRepositoryForDb {
    async fn create(&self, name: String) -> anyhow::Result<Label> {
        let optional_label = sqlx::query_as::<_, Label>(
            r#"
            select * from labels where name = $1
            "#,
        )
        .bind(name.clone())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(label) = optional_label {
            return Err(RepositoryError::Duplicate(label.id).into());
        }

        let label = sqlx::query_as::<_, Label>(
            r#"
            insert into labels (name) values ($1) returning *
            "#,
        )
        .bind(name.clone())
        .fetch_one(&self.pool)
        .await?;

        Ok(label)
    }

    async fn all(&self) -> anyhow::Result<Vec<Label>> {
        let labels = sqlx::query_as::<_, Label>(
            r#"
            select * from labels
            order by labels.id asc;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(labels)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            delete from labels where id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "database-test")]
mod test {
    use std::env;

    use super::*;

    use dotenv::dotenv;

    #[tokio::test]
    async fn crud_scenario() {
        dotenv().ok();
        let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(database_url)
            .await
            .unwrap_or_else(|_| panic!("failed connect database: {}", database_url));

        let repository = LabelRepositoryForDb::new(pool);
        let label_text = "test_label";

        // create
        let label = repository
            .create(label_text.to_string())
            .await
            .expect("failed create");
        assert_eq!(label.name, label_text);

        // all
        let labels = repository.all().await.expect("failed all");
        let label = labels.last().unwrap();
        assert_eq!(label.name, label_text);

        // delete
        repository.delete(label.id).await.expect("failed delete");
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

    use anyhow::Context;

    use super::*;

    type LabelDatas = HashMap<i32, Label>;

    impl Label {
        pub fn new(id: i32, name: String) -> Label {
            Self { id, name }
        }
    }

    #[derive(Debug, Clone)]
    pub struct LabelRepositoryInMemory {
        store: Arc<RwLock<LabelDatas>>,
    }

    impl LabelRepositoryInMemory {
        pub fn new() -> Self {
            Self {
                store: Arc::default(),
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<LabelDatas> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<LabelDatas> {
            self.store.read().unwrap()
        }
    }

    #[async_trait]
    impl LabelRepository for LabelRepositoryInMemory {
        async fn create(&self, name: String) -> anyhow::Result<Label> {
            let mut store = self.write_store_ref();
            let id = store.len() as i32 + 1;
            let label = Label { id, name };
            store.insert(id, label.clone());
            Ok(label)
        }

        async fn all(&self) -> anyhow::Result<Vec<Label>> {
            let store = self.read_store_ref();
            Ok(store.values().cloned().collect())
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
        async fn label_crud_scenario() {
            let label_text = "test_label";

            let repository = LabelRepositoryInMemory::new();

            // create
            let label = repository
                .create(label_text.to_string())
                .await
                .expect("failed create");
            assert_eq!(label.name, label_text);

            // all
            let labels = repository.all().await.expect("failed all");
            let label = labels.last().unwrap();
            assert_eq!(label.name, label_text);

            // delete
            repository.delete(label.id).await.expect("failed delete");
        }
    }
}
