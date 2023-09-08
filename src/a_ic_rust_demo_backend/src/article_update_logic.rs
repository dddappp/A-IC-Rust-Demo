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
    article_updated: &ArticleUpdated,
    article: &Article,
) -> Article {
    let mut article = article.clone();
    article.title = article_updated.title.clone();
    article.body = article_updated.body.clone();
    article
}
