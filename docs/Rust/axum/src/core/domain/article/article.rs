use validator::Validate;
use serde::Deserialize;

#[derive(Validate, Deserialize)]
struct Id {
    #[validate(range(min = 1))]
    id: i32,
}

#[derive(Validate, Deserialize)]
struct Title {
    #[validate(length(min = 1))]
    #[validate(length(max = 255))]
    title: String,
}

#[derive(Validate, Deserialize)]
struct Body {
    #[validate(length(min = 1))]
    body: String,
}

// TODO: タグの型を定義
#[derive(Validate, Deserialize)]
struct TagList {
    #[validate(length(min = 1))]
    tag_list: Vec<String>,
}
