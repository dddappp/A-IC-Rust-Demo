use crate::article::{
    Article,
    ArticleBodyUpdated,
};

pub(crate) fn verify(
    body: String,
    article: &Article,
) -> ArticleBodyUpdated {
    ArticleBodyUpdated {
        article_id: article.article_id,
        version: article.version,
        body,
    }
}

pub(crate) fn mutate(
    article_body_updated: &ArticleBodyUpdated,
    article: Article,
) -> Article {
    let mut article = article.clone();
    article.body = article_body_updated.body.clone();
    article
}
