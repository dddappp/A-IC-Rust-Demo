use std::cell::RefCell;

use ic_cdk::{
    query, update,
};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableLog};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};

mod article;

use article::Article;

mod article_create_logic;
mod article_update_logic;

mod event;

type Memory = VirtualMemory<DefaultMemoryImpl>;

type EventStore = StableLog<event::Event, Memory, Memory>;

type ArticleStore = StableBTreeMap<u128, Article, Memory>;

const ARTICLE_MEM_ID: MemoryId = MemoryId::new(0);
const EVENT_IDX_MEM_ID: MemoryId = MemoryId::new(1);
const EVENT_DATA_MEM_ID: MemoryId = MemoryId::new(2);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ARTICLE_STORE: RefCell<ArticleStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(ARTICLE_MEM_ID)),
        )
    );

    static EVENT_STORE: RefCell<EventStore> = RefCell::new(
        StableLog::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(EVENT_IDX_MEM_ID)),
            MEMORY_MANAGER.with(|m| m.borrow().get(EVENT_DATA_MEM_ID)),
        ).unwrap()
    );
}

#[query(name = "get")]
fn get(article_id: u128) -> Option<Article> {
    ARTICLE_STORE.with(|s| {
        s.borrow().get(&article_id)
    })
}

#[update(name = "create")]
fn create(
    article_id: u128,
    title: String,
    body: String,
) {
    let article_created = article_create_logic::verify(
        article_id,
        title,
        body,
    );
    let mut article = article_create_logic::mutate(
        &article_created,
    );
    article.version = 0;
    EVENT_STORE.with(|event_store| {
        event_store.borrow_mut().append(&event::Event::ArticleEvent(event::ArticleEvent::ArticleCreated(article_created))).unwrap();
    });
    ARTICLE_STORE.with(|s| {
        s.borrow_mut().insert(
            article_id,
            article,
        );
    });
}

#[update(name = "update")]
fn update(
    article_id: u128,
    title: String,
    body: String,
) {
    let mut article: Article = Default::default();
    ARTICLE_STORE.with(|s| {
        article = s.borrow_mut().remove(&article_id).unwrap();
    });
    let old_version = article.version;
    let mut article_updated = article_update_logic::verify(
        title,
        body,
        &article,
    );
    article_updated.version = old_version;
    let mut updated_article = article_update_logic::mutate(
        &article_updated,
        article,
    );
    updated_article.version = old_version + 1;
    EVENT_STORE.with(|event_store| {
        event_store.borrow_mut().append(&event::Event::ArticleEvent(event::ArticleEvent::ArticleUpdated(article_updated))).unwrap();
    });
    ARTICLE_STORE.with(|s| {
        s.borrow_mut().insert(
            article_id,
            updated_article,
        );
    });
}

#[query(name = "getEvent")]
fn get_event(idx: u64) -> Option<event::Event> {
    EVENT_STORE.with(|event_store| {
        event_store.borrow().get(idx)
    })
}

