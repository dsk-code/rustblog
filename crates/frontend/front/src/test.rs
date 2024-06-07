use serde::Deserialize;
use gloo_net::http::Request;
use yew::prelude::*;
use crate::components::{VideoDetails, VideosList};
// use webpage::{WebpageOptions, Webpage};

#[derive(Clone, PartialEq ,Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

// fn image(url: &str) -> String {
//     let info = Webpage::from_url(url, WebpageOptions::default())
//         .expect("test");
//     let html = info.html;
//     let image_url = html
//         .opengraph
//         .images
//         .first()
//         .map(|x| x.url.clone())
//         .unwrap_or("https://www.rust-lang.org/static/images/rust-social-wide.jpg".to_string());

//     log::info!("Image URL: {}", image_url);
//     image_url
// }

#[function_component(Test)]
pub fn test() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }   

    // let videos = vec![
    //     Video {
    //         id: 1,
    //         title: "Building and breaking things".to_string(),
    //         speaker: "John Doe".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 2,
    //         title: "The development process".to_string(),
    //         speaker: "Jane Smith".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 3,
    //         title: "The Web 7.0".to_string(),
    //         speaker: "Matt Miller".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 4,
    //         title: "Mouseless development".to_string(),
    //         speaker: "Tom Jerry".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    // ];
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });

    // let image_url = image("https://www.rust-lang.org");

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                // <VideosList videos={videos} on_click={on_video_select.clone()} />
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
            // <img src={image_url}/>
        </>
    }
    // html! {
    //     <p>{"hello world!"}</p>
    // }
}   

