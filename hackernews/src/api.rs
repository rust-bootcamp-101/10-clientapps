#![allow(unused)]

use anyhow::Result;
use futures::future::join_all;

use crate::{Comment, StoryData, StoryItem};

const MAX_SCORES: usize = 50;

pub async fn get_top_stores(n: usize) -> Result<Vec<StoryItem>> {
    let n = n.min(MAX_SCORES);
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let ids: Vec<i64> = reqwest::get(url).await?.json().await?;
    let story_futures = ids.into_iter().take(n).map(get_story_item);
    // Vec 不使用collect，collect是一个同步的操作
    // 使用futures的join_all使用异步操作
    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|item| item.ok())
        .collect::<Vec<StoryItem>>();
    Ok(stories)
}

pub async fn get_story_item(id: i64) -> Result<StoryItem> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let item = reqwest::get(url).await?.json().await?;
    Ok(item)
}

pub async fn get_story_comments(item: StoryItem) -> Result<StoryData> {
    let comment_futures = item.kids.iter().map(|id| get_comment_by_id(*id));
    let comments = join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|item| item.ok())
        .collect::<Vec<Comment>>();
    Ok(StoryData { item, comments })
}

pub async fn get_comment_by_id(id: i64) -> Result<Comment> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let item = reqwest::get(url).await?.json().await?;
    Ok(item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_top_stores_should_work() {
        let stories = get_top_stores(1).await.unwrap();
        dbg!(&stories);
        assert_eq!(stories.len(), 1);
    }

    #[tokio::test]
    async fn get_story_comments_should_work() {
        let stories = get_top_stores(1).await.unwrap();
        let item = stories[0].clone();
        let comments = get_story_comments(item).await.unwrap();
        dbg!(&comments);
    }
}
