use crate::article::{
    Article,
    ArticleCreated,
};

pub(crate) fn verify(
    article_id: u128,
    title: String,
    body: String,
) -> ArticleCreated {
    ArticleCreated { article_id, title, body }
}

pub(crate) fn mutate(
    article_created: &ArticleCreated
) -> Article {
    Article {
        article_id: article_created.article_id,
        version: 0,
        title: article_created.title.clone(),
        body: article_created.body.clone(),
    }
}