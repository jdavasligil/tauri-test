use brainrot::{
    twitch::Anonymous,
    youtube::{self, Action, ChatContext},
    TwitchChatEvent, VariantChat,
};
use futures_util::StreamExt;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// Channels https://v2.tauri.app/develop/calling-frontend/#channels

// TODO:
// - Message Channel to Front End
// - SQLite storage for durability and loading past messages
#[tauri::command]
async fn download_chat(twitch_url: &str, youtube_name: &str) -> Result<(), ()> {
    let context = ChatContext::new_from_channel(
        youtube_name,
        youtube::ChannelSearchOptions::LatestLiveOrUpcoming,
    )
    .await
    .unwrap();
    let mut multiclient = brainrot::Multicast::new();
    multiclient.push_youtube(&context).await.unwrap();
    multiclient
        .push_twitch(twitch_url, Anonymous)
        .await
        .unwrap();

    let handle_twitch_event = |e: TwitchChatEvent| {};
    let handle_youtube_action = |a: Action| {};

    while let Some(chat) = multiclient.next().await.transpose().unwrap() {
        match chat {
            VariantChat::Twitch(e) => handle_twitch_event(e),
            VariantChat::YouTube(a) => handle_youtube_action(a),
        }
        // if let TwitchChatEvent::Message { user, contents, .. } = message {
        //     println!(
        //         "{}: {}",
        //         user.display_name,
        //         contents.iter().map(|c| c.to_string()).collect::<String>()
        //     );
        // }
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    {
        if std::path::Path::new("/dev/dri").exists()
            && std::env::var("WAYLAND_DISPLAY").is_err()
            && std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "x11"
        {
            // SAFETY: There's potential for race conditions in a multi-threaded context.
            unsafe {
                std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
            }
        }
    }
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Elora Chat")?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![download_chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
