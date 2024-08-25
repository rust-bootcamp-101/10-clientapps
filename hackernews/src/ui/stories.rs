use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::{
    api::{get_story_comments, get_top_stores},
    StoryData, StoryItem,
};

use super::CommentsState;

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_top_stores(20));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
            ul {
                for story in stories {
                    StoryItem{ story: story.clone() }
                }
            }
        },
        Some(Err(e)) => rsx! {
            div { class: "mt-6 text-red-500",
                p { "Failed to fetch stories" }
                p { "{e}" }
            }
        },
        None => rsx! {
            div { class: "mt-6",
                p { "Loading stories..." }
            }
        },
    }
}

#[component]
pub fn StoryItem(story: StoryItem) -> Element {
    let mut comments_state = use_context::<Signal<CommentsState>>();
    // cache of the already loaded comments: Option<StoryData>
    // let full_story = use_signal(|| None);
    rsx! {
        li { class: "px-3 py-5 transition border-b hover:bg-indigo-100",
            a { href: "#", class: "flex items-center justify-between",
                h3 { class: "text-lg font-semibold", "{story.title}" }
                p { class: "text-gray-400 text-md" }
            }
            div { class: "italic text-gray-400 text-md",
                span{"{story.score} points by {story.by} {story.time} | "}
                a {
                    href: "#",
                    prevent_default: "onclick",
                    onclick: move |_| {
                        info!("Clicked on story: {}", story.title.clone());
                        let story = story.clone();
                        async move {
                            *comments_state.write() = CommentsState::Loading;
                            if let Ok(story_data) = get_story_comments(story).await {
                                *comments_state.write() = CommentsState::Loaded(story_data);
                            }
                        }
                    },
                    "{story.kids.len()} comments"
                }
            }
        }
    }
}

#[allow(unused)]
async fn load_comments(
    mut comment_state: Signal<CommentsState>,
    mut full_story: Signal<Option<StoryData>>,
    story: StoryItem,
) {
    // if the comments are already loaded, just change comments_state and return
    if let Some(story_data) = full_story.as_ref() {
        *comment_state.write() = CommentsState::Loaded(story_data.clone());
        return;
    }

    // if no, set comments_state to Loading and fetch the comments
    *comment_state.write() = CommentsState::Loading;

    if let Ok(story_data) = get_story_comments(story).await {
        *comment_state.write() = CommentsState::Loaded(story_data.clone());
        *full_story.write() = Some(story_data);
    }
}
