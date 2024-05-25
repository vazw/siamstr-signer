use leptos::leptos_dom::logging::console_log;
use leptos::*;
use nostr_signer::prelude::*;
use qrcode_generator::QrCodeEcc;
use crate::nostr_connect::connect_signer::{NoPassAction, WithPassAction};
use rand::{distributions::Alphanumeric, Rng};

#[component]
pub fn QrCodeCmp(svg_text: RwSignal<String>) -> impl IntoView {
    {move || {
        let svgs = view! { <div></div> };
        let svg_text = svg_text.get();
        svgs.set_inner_html(&svg_text);
        svgs
    }}
}

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let nsec_key = create_rw_signal("".to_string());
    let bunker_uri = create_rw_signal("".to_string());
    let bunker_qr = create_rw_signal("".to_string());
    let use_pass = create_rw_signal(false);
    let start_serve = move |_| {
        let _ = web_sys::Notification::request_permission();
        spawn_local(async move {
            let nsec_keys = SecretKey::parse(nsec_key.clone().get_untracked())
                .expect("incorrect nsec key");

            let keys = Keys::new(nsec_keys);
            if use_pass.clone().get_untracked() {
                let random_pass = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect();
                let signer =
                    NostrConnectRemoteSigner::new(keys.secret_key().unwrap().clone(), ["wss://sign.siamstr.com"], Some(random_pass), None)
                        .await
                        .expect("remote signer initialized");
                let uri = signer.nostr_connect_uri().await.to_string();
                let result: String = qrcode_generator::to_svg_to_string(uri.clone(), QrCodeEcc::Low, 256, None::<&str>).unwrap();

                bunker_uri.set(uri);
                bunker_qr.set(result);
                loop {
                    signer.serve(WithPassAction).await.expect("serving");
                }
            } else {
                let signer =
                    NostrConnectRemoteSigner::new(keys.secret_key().unwrap().clone(), ["wss://sign.siamstr.com"], None, None)
                        .await
                        .expect("remote signer initialized");
                let uri = signer.nostr_connect_uri().await.to_string();
                let result: String = qrcode_generator::to_svg_to_string(uri.clone(), QrCodeEcc::Low, 256, None::<&str>).unwrap();

                bunker_uri.set(uri);
                bunker_qr.set(result);
                loop {
                signer.serve(NoPassAction).await.expect("serving");
                }
            }
        })
    };
    let trigger_show_event = move |_| {
        let menu = web_sys::window().unwrap().document().unwrap().get_element_by_id("events-list").expect("element found");
        let _ = menu.class_list().toggle("hidden");
    };
    let trigger_show_qr = move |_| {
        let menu = web_sys::window().unwrap().document().unwrap().get_element_by_id("bunker-uri").expect("element found");
        let _ = menu.class_list().toggle("hidden");
    };

    view! {
        <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
            "siamstr signer"
        </label>
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>
                    "ไม่นะ! บางอย่างผิดพลาด โปรดลองใหม่อีกครั้ง"
                </h1>

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

            <div class="flex flex-col">
                <div>
                    <input
                        type="password"
                        class="text-gray-900 dark:text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                        prop:placeholder="nsec_key"
                        on:input=move |ev| {
                            let val = event_target_value(&ev)
                                .parse::<String>()
                                .unwrap_or("".to_string());
                            nsec_key.set(val);
                        }
                    />

                </div>
                <div>
                    <label class="relative inline-flex items-center cursor-pointer">
                        <input
                            type="checkbox"
                            value=""
                            class="sr-only peer"
                            on:change=move |_| {
                                if use_pass.get() {
                                    use_pass.set(false)
                                } else {
                                    use_pass.set(true)
                                };
                            }
                        />

                        <div class="w-9 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-orange-300 dark:peer-focus:ring-orange-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-orange-600"></div>
                        <span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                            "auto-sign event"
                        </span>
                    </label>
                    <button class="btn btn--primary" on:click=start_serve>
                        "serve"
                    </button>
                </div>
                <div>
                    <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                        "auto-sign suit for bunker only if using nip05 this should be disabled"
                    </label>
                </div>
                <div>
                    <label class="relative inline-flex items-center cursor-pointer">
                        <input
                            type="checkbox"
                            value=""
                            class="sr-only peer"
                            on:change=trigger_show_qr
                        />

                        <div class="w-9 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-orange-300 dark:peer-focus:ring-orange-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-orange-600"></div>
                        <span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                            "Hide QR-CODE"
                        </span>
                    </label>
                    <label class="relative inline-flex items-center cursor-pointer">
                        <input
                            checked
                            type="checkbox"
                            value=""
                            class="sr-only peer"
                            on:change=trigger_show_event
                        />

                        <div class="w-9 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-orange-300 dark:peer-focus:ring-orange-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-orange-600"></div>
                        <span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                            "Show Last Event"
                        </span>
                    </label>
                </div>
                <div id="bunker-uri">
                    {move || bunker_uri.get()} <QrCodeCmp svg_text=bunker_qr/>
                </div>
                <div class="" id="events-list">
                    <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                        "Lastest Events:"
                    </label>
                    <div id="eventsSigned" class="flex flex-col-reverse"></div>
                </div>
            </div>
        </ErrorBoundary>
    }
}
