use axum::{Router, routing::get};
use maud::{DOCTYPE, Markup, html};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(mainpage));

    const IP_PORT: &str = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(IP_PORT).await.unwrap();

    println!("Listening on http://{IP_PORT}");

    axum::serve(listener, app).await.unwrap();
}

async fn mainpage() -> Markup {
    html!(
        (DOCTYPE)
        head {
            // TODO: Replace cdns in production
            script src="https://cdn.tailwindcss.com" {}
            script src="https://unpkg.com/htmx.org@2.0.4" integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+" crossorigin="anonymous"{}
            script type="module" src="/static/index.js" {}
        }

        body {
        div class="flex flex-col items-center justify-between h-screen w-screen"{
            div {}
            div class="flex flex-col" {
                div class="flex flex-row gap-2"{
                    h1 class="text-4xl hover:scale-110 hover:font-weight-bold hover:cursor-pointer " 
                        hx-on:click="hello()"
                        {"Hello"}
                    h1 class="text-4xl " {"world! 👋"}
                }
                p class="text-xs text-slate-400"{ "Template created by MagnumTrader"}
            }
            div class="flex justify-center gap-1 mb-2" {
                a href="https://github.com/MagnumTrader" target="_blank" class="text-xs text-sky-900 "{ "Github"}
                a href="https://x.com/magnum_trader" target="_blank" class="text-xs text-sky-900 "{ "Twitter"}
            }
        }
        }
    )
}
