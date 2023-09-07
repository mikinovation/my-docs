use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use super::{label::Label, RepositoryError};

#[derive(Debug, Clone)]
pub struct ArticleRepositoryForDb {
    pool: PgPool,
}

impl ArticleRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        ArticleRepositoryForDb { pool }
    }
}

#[async_trait]
impl ArticleRepository for ArticleRepositoryForDb {
/*    async fn create(&self, payload: Create) -> anyhow::Result<ArticleEntity> {
        let tx = self.pool.begin().await?;
        let row = sqlx::query_as::<_, ArticleFromRow>(
            r#"
insert into articles (text, completed)
values ($1, false)
returning *;
        "#,
        )
            .bind(payload.text.clone())
            .fetch_one(&self.pool)
            .await?;

        tx.commit().await?;

        let article = self.find(row.id).await?;
        Ok(article)
    }

    async fn find(&self, id: i32) -> anyhow::Result<ArticleEntity> {
        let items = sqlx::query_as::<_, ArticleWithLabelFromRow>(
            r#"
select articles.*, labels.id as label_id, labels.name as label_name
from articles
            left outer join article_labels tl on articles.id = tl.article_id
            left outer join labels on labels.id = tl.label_id
where articles.id=$1;
        "#,
        )
            .bind(id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                _ => RepositoryError::Unexpected(e.to_string()),
            })?;

        let articles = fold_entities(items);
        let article = articles.first().ok_or(RepositoryError::NotFound(id))?;
        Ok(article.clone())
    }
*/
    async fn all(&self) -> anyhow::Result<Vec<ArticleEntity>> {
        let items = sqlx::query_as::<_, ArticleWithLabelFromRow>(
            r#"
select articles.*, labels.id as label_id, labels.name as label_name
from articles
            left outer join article_labels tl on articles.id = tl.article_id
            left outer join labels on labels.id = tl.label_id
order by articles.id desc;
        "#,
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(fold_entities(items))
    }

/*    async fn update(&self, id: i32, payload: UpdateArticle) -> anyhow::Result<ArticleEntity> {
        let tx = self.pool.begin().await?;

        // article update
        let old_article = self.find(id).await?;
        sqlx::query(
            r#"
update articles set text=$1, completed=$2
where id=$3
returning *
        "#,
        )
            .bind(payload.text.unwrap_or(old_article.text))
            .bind(payload.completed.unwrap_or(old_article.completed))
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        tx.commit().await?;
        let article = self.find(id).await?;

        Ok(article)
    }
*/
/*    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let tx = self.pool.begin().await?;
        // article's label delete
        sqlx::query(
            r#"
delete from article_labels where article_id=$1
        "#,
        )
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                _ => RepositoryError::Unexpected(e.to_string()),
            })?;
        // article delete
        sqlx::query(
            r#"
delete from articles where id=$1
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
*/
}

#[async_trait]
pub trait ArticleRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    // async fn create(&self, payload: CreateArticle) -> anyhow::Result<ArticleEntity>;
    // async fn find(&self, id: i32) -> anyhow::Result<ArticleEntity>;
    async fn all(&self) -> anyhow::Result<Vec<ArticleEntity>>;
    // async fn update(&self, id: i32, payload: UpdateArticle) -> anyhow::Result<ArticleEntity>;
    // async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct ArticleFromRow {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
struct ArticleWithLabelFromRow {
    id: i32,
    text: String,
    completed: bool,
    label_id: Option<i32>,
    label_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArticleEntity {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub labels: Vec<Label>,
}

fn fold_entities(rows: Vec<ArticleWithLabelFromRow>) -> Vec<ArticleEntity> {
    let mut rows = rows.iter();
    let mut accum: Vec<ArticleEntity> = vec![];
    'outer: while let Some(row) = rows.next() {
        let mut articles = accum.iter_mut();
        while let Some(article) = articles.next() {
            // idが一致＝Articleに紐づくラベルが複数存在している
            if article.id == row.id {
                article.labels.push(Label {
                    id: row.label_id.unwrap(),
                    name: row.label_name.clone().unwrap(),
                });
                continue 'outer;
            }
        }

        // Articleのidに一致がなかった時のみ到達、ArticleEntityを作成
        let labels = if row.label_id.is_some() {
            vec![Label {
                id: row.label_id.unwrap(),
                name: row.label_name.clone().unwrap(),
            }]
        } else {
            vec![]
        };

        accum.push(ArticleEntity {
            id: row.id,
            text: row.text.clone(),
            completed: row.completed,
            labels,
        });
    }
    accum
}
