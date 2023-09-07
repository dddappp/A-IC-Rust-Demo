use crate::article::{
    Article,
    ArticleUpdated,
};

pub(crate) fn verify(
    title: String,
    body: String,
    article: &Article,
) -> ArticleUpdated {
    ArticleUpdated {
        article_id: article.article_id,
        version: article.version,
        title,
        body,
    }
}

pub(crate) fn mutate(
    article_created: &ArticleUpdated,
    article: &Article,
) -> Article {
    let mut article = article.clone();
    article.title = article_created.title.clone();
    article.body = article_created.body.clone();
    article
}