use leptos::*;
use nostr::nips::nip46::Request;
use nostr_signer::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{self, HtmlElement};

pub struct NoPassAction;

impl NostrConnectSignerActions for NoPassAction {
    fn approve(&self, req: &Request) -> bool {
        let text_alert = match req {
            Request::SignEvent(even) => {
                let contents = &even.content;
                let event_kind: Kind = even.kind;
                format!("Sign events for {:#?} kind\ncontents: {}", event_kind, contents)
            }
            Request::Connect { public_key: _, secret } => {
                let passwd = secret.as_ref();
                if let Some(password) = passwd {
                    format!("Login Connect with passwd: {}", password)
                }else {
                    "Login Connect".to_string()
                }
            },
            Request::GetRelays => "GetRelays".to_string(),
            Request::GetPublicKey => "GetPublicKey".to_string(),
            Request::Nip04Encrypt { public_key:_, text } => {
                format!("Nip04Encrypt: {}", text)
            },
            Request::Nip04Decrypt { public_key:_, ciphertext } => {
                format!("Nip04Decrypt: {}", ciphertext)
            },
            Request::Nip44Encrypt { public_key:_, text } => {
                format!("Nip04Encrypt: {}", text)
            },
            Request::Nip44Decrypt { public_key:_, ciphertext } => {
                format!("Nip04Decrypt: {}", ciphertext)
            },
            Request::Ping => "Pong!".to_string(),
        };

        let text_alert_div = window()
            .document()
            .expect("docs")
            .create_element("div")
            .expect("new div");
        let _ = text_alert_div.append_with_str_1(&text_alert);
        let _ = window().document()
            .expect("docs")
            .get_element_by_id("eventsSigned")
            .expect("eventsSigned loaded")
            .dyn_ref::<HtmlElement>()
            .expect("HtmlElement")
            .append_child(&text_alert_div);

        match req {
            Request::GetRelays|Request::GetPublicKey|Request::Ping => { true },
            _ => {
                // Create a new window
                let popup_window =
                    window().confirm_with_message(&text_alert);
                match popup_window.expect("accept") {
                    true => true,
                    false => false,
                }
            }
        }
    }
}


pub struct WithPassAction;

impl NostrConnectSignerActions for WithPassAction {
    fn approve(&self, req: &Request) -> bool {
        let text_alert = match req {
            Request::SignEvent(even) => {
                let contents = &even.content;
                let event_kind: Kind = even.kind;
                format!("Sign events for {:#?} kind\ncontents: {}", event_kind, contents)
            }
            Request::Connect { public_key: _, secret } => {
                let passwd = secret.as_ref();
                if let Some(password) = passwd {
                    format!("Login Connect with passwd: {}", password)
                }else {
                    "Login Connect".to_string()
                }
            },
            Request::GetRelays => "GetRelays".to_string(),
            Request::GetPublicKey => "GetPublicKey".to_string(),
            Request::Nip04Encrypt { public_key:_, text } => {
                format!("Nip04Encrypt: {}", text)
            },
            Request::Nip04Decrypt { public_key:_, ciphertext } => {
                format!("Nip04Decrypt: {}", ciphertext)
            },
            Request::Nip44Encrypt { public_key:_, text } => {
                format!("Nip04Encrypt: {}", text)
            },
            Request::Nip44Decrypt { public_key:_, ciphertext } => {
                format!("Nip04Decrypt: {}", ciphertext)
            },
            Request::Ping => "Pong!".to_string(),
        };

        let text_alert_div = window()
            .document()
            .expect("docs")
            .create_element("div")
            .expect("new div");
        let _ = text_alert_div.append_with_str_1(&text_alert);
        let _ = window().document()
            .expect("docs")
            .get_element_by_id("eventsSigned")
            .expect("eventsSigned loaded")
            .dyn_ref::<HtmlElement>()
            .expect("HtmlElement")
            .append_child(&text_alert_div);

        true
    }
}
