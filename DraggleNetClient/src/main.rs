pub mod draggle_net {
    tonic::include_proto!("draggle_net");
}

use crate::draggle_net::draggle_net_client::DraggleNetClient;
use crate::draggle_net::PingRequest;
use dioxus::prelude::*;
use tonic::Request;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/tailwind.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = DraggleNetClient::connect("http://[::1]:50051").await?;

    //TODO make a function for into_request
    let request = Request::new(PingRequest { message: "ping from the client".to_string() });

    let response = client.send(request).await?;
    println!("Response: {:?}", response);


    //TODO look at musa-gui launch_app. Probably will have to put the client event handling on a separate thread and it'll take the main thread
    launch(App);

    Ok(())
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
