use stylist::{yew::styled_component, Style};
use yew::function_component;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("videos.css");

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
    pub color: Color,
    pub on_load: Callback<String>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[derive(PartialEq, Clone)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "ok".to_owned(),
            Color::Error => "error".to_owned(),
        }
    }
}

#[styled_component(VideoDetails)]
/// show title and img
pub fn video_details(VideosDetailsProps { video, color, on_load }: &VideosDetailsProps) -> Html {
    let style_detail = Style::new(STYLE_FILE).unwrap();
    on_load.emit("loaded!!!".to_owned());
    html! {
        <div class={style_detail}>
            <h3 class={color.to_string()}>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };

            html! {
                <p onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}
