// use crate::components::counter_btn::Button;
use leptos::leptos_dom::logging::console_log;
use leptos::*;
use nostr::nips::nip46::Request;
use nostr_signer::prelude::*;
use web_sys;

struct CustomActions;

impl NostrConnectSignerActions for CustomActions {
    fn approve(&self, req: &Request) -> bool {
        let text_alert = match req {
            Request::SignEvent(even) => {
                let contents = &even.content;
                let event_kind: Kind = even.kind;
                format!("{:#?} kind\ncontents: {}", event_kind, contents)
            }
            _ => "".to_string(),
        };
        let window = web_sys::window().unwrap();

        // Create a new window
        let popup_window =
            window.confirm_with_message(&format!("New sign events for {}", text_alert));

        match popup_window.expect("accept") {
            true => true,
            false => false,
        }
    }
}

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let nsec_key = create_rw_signal("".to_string());
    let bunker_uri = create_rw_signal("".to_string());
    let _ = web_sys::Notification::request_permission();
    let start_serve = move |_| {
        spawn_local(async move {
            let nsec_keys = SecretKey::parse(nsec_key.clone().get_untracked())
                .ok()
                .unwrap();

            let signer =
                NostrConnectRemoteSigner::new(nsec_keys, ["wss://sign.siamstr.com"], None, None)
                    .await
                    .ok()
                    .unwrap();
            let uri = signer.nostr_connect_uri().await.to_string();
            console_log(&format!("{}", &uri));
            bunker_uri.set(uri);

            signer.serve(CustomActions).await.expect("serving");
        })
    };
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"ไม่นะ! บางอย่างผิดพลาด โปรดลองใหม่อีกครั้ง"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                        <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                            "nsec"
                            <input
                                type="password"
                                class="text-gray-900 dark:text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                                prop:placeholder="nsec_key"
                                on:input=move |ev| {
                                    let val = event_target_value(&ev)
                                        .parse::<String>()
                                        .unwrap_or("".to_string());
                                    if val.is_empty() {
                                        nsec_key.set(val);
                                    } else {
                                        nsec_key.set(val);
                                    }
                                }
                            />
                        </label>


                <div class="buttons">
                <button class="btn btn--primary" on:click=start_serve>
                    "serve"
                </button>
                </div>
                <div>
                {move || bunker_uri.get()}
                </div>

            </div>
        </ErrorBoundary>
    }
}
