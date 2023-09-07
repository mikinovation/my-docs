use axum::async_trait;
use super::article::;

#[async_trait]
pub trait ArticleRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    // async fn create(&self, payload: CreateArticle) -> Article;
    // async fn find(&self, id: i32) -> Option<Article>;
    async fn all(&self) -> Vec<Article>;
    // async fn update(&self, id: i32, payload: UpdateArticle) -> anyhow::Result<Article>;
    // async fn delete(&self, id: i32) -> anyhow::Result<()>;
}


/*#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateArticle {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    text: Option<String>,
    completed: Option<bool>,
    labels: Option<Vec<i32>>
}
*/