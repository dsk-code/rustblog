use crate::test::Video;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click:onclick }: &VideosListProps) -> Html {
    let onclick = onclick.clone();
    videos.iter().map(|video| {
        let on_video_select = {
            let onclick = onclick.clone();
            let video = video.clone();
            Callback::from(move |_| {
                onclick.emit(video.clone())
            })
        };
        
        html! {
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}
