use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize)]
pub struct MyObject {
    pub username: String,
    pub favorite_language: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>
}

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct VideosDetailsProps {
    pub video: Video,
}

