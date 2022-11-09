use gloo::console::log;
use serde::Serialize;
use yew::prelude::*;

pub mod models;

use models::Video;

#[derive(Serialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];
    let videos = videos
        .iter()
        .map(|video| {
            html! {
                <li>{format!("{}: {}", video.speaker, video.title)}</li>
            }
        })
        .collect::<Html>();
    let name: &str = "Brooks";
    let my_obj: MyObject = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_string(),
    };

    log!(serde_json::to_string_pretty(&my_obj).unwrap());
    log!(name);
    let class: &str = "my_title";
    let message: Option<&str> = None;
    let tasks: Vec<&str> = vec!["record video", "grocery shopping", "pet"];
    html! {
        <>
            <h1 class={class}>{"Hello world!!"}</h1>
            if class == "my_title" {<p>{"Hi there!"}</p>}
            else {<p>{"I'm not a titles"}</p>}
            if let Some(m) = message {
                <p>{m}</p>
            } else {<p>{"message not found"}</p>}
            <ul>
                {tasks.iter().map(|task| html!{<li>{task}</li>}).collect::<Html>()}
            </ul>
            <p>
                {my_obj.username}
            </p>
            <ul>{videos}</ul>

        </>
    }
}
