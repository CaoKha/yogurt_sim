use gloo::console::log;
use yew::function_component;
use yew::prelude::*;

pub mod models;
use models::MyObject;
use models::Video;
use models::VideosDetailsProps;
use models::VideosListProps;

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
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

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
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
    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };
    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });
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
                {list_to_html(tasks)}
            </ul>
            <p>
                {my_obj.username}
            </p>
            <VideosList videos={videos} on_click={on_video_select}/>
            {for details}  
        </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}
