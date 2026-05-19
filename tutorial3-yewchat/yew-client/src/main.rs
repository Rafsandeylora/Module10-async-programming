use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, InputEvent, KeyboardEvent, MessageEvent, WebSocket};
use yew::{prelude::*, TargetCast};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ChatMessage {
    user: String,
    message: String,
}

#[function_component(App)]
fn app() -> Html {
    let messages = use_state(Vec::<ChatMessage>::new);
    let input_message = use_state(String::new);
    let username = use_state(|| "Rafsan".to_string());
    let connection_status = use_state(|| "Connecting...".to_string());
    let ws_ref = use_mut_ref(|| None::<WebSocket>);

    {
        let messages = messages.clone();
        let connection_status = connection_status.clone();
        let ws_ref = ws_ref.clone();

        use_effect_with((), move |_| {
            let ws = WebSocket::new("ws://127.0.0.1:9001")
                .expect("failed to create websocket connection");

            let onopen_status = connection_status.clone();
            let onopen = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_| {
                onopen_status.set("Connected".to_string());
            }));

            ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
            onopen.forget();

            let onmessage_messages = messages.clone();
            let onmessage = Closure::<dyn FnMut(MessageEvent)>::wrap(Box::new(move |event: MessageEvent| {
                if let Some(text) = event.data().as_string() {
                    match serde_json::from_str::<ChatMessage>(&text) {
                        Ok(chat_message) => {
                            let mut next_messages = (*onmessage_messages).clone();
                            next_messages.push(chat_message);
                            onmessage_messages.set(next_messages);
                        }
                        Err(_) => {
                            let mut next_messages = (*onmessage_messages).clone();
                            next_messages.push(ChatMessage {
                                user: "server".to_string(),
                                message: text,
                            });
                            onmessage_messages.set(next_messages);
                        }
                    }
                }
            }));

            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();

            let onerror_status = connection_status.clone();
            let onerror = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_| {
                onerror_status.set("Error".to_string());
            }));

            ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
            onerror.forget();

            let onclose_status = connection_status.clone();
            let onclose = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_| {
                onclose_status.set("Closed".to_string());
            }));

            ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
            onclose.forget();

            let ws_for_cleanup = ws.clone();
            *ws_ref.borrow_mut() = Some(ws);

            move || {
                let _ = ws_for_cleanup.close();
            }
        });
    }

    let oninput_message = {
        let input_message = input_message.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            input_message.set(input.value());
        })
    };

    let send_message: Callback<()> = {
        let input_message = input_message.clone();
        let username = username.clone();
        let ws_ref = ws_ref.clone();

        Callback::from(move |_| {
            let text = (*input_message).trim().to_string();

            if text.is_empty() {
                return;
            }

            let chat_message = ChatMessage {
                user: (*username).clone(),
                message: text,
            };

            let json = serde_json::to_string(&chat_message)
                .expect("failed to serialize chat message");

            if let Some(ws) = ws_ref.borrow().as_ref() {
                let _ = ws.send_with_str(&json);
            }

            input_message.set(String::new());
        })
    };

    let onclick_send = {
        let send_message = send_message.clone();

        Callback::from(move |_| {
            send_message.emit(());
        })
    };

    let onkeydown_message = {
        let send_message = send_message.clone();

        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                send_message.emit(());
            }
        })
    };

    html! {
        <div style="max-width: 720px; margin: 40px auto; font-family: Arial, sans-serif;">
            <h1>{ "YewChat - Original" }</h1>
            <p>{ format!("Status: {}", *connection_status) }</p>

            <div style="border: 1px solid #ccc; padding: 16px; min-height: 300px; margin-bottom: 12px;">
                {
                    for messages.iter().map(|message| {
                        html! {
                            <p>
                                <strong>{ format!("{}: ", message.user) }</strong>
                                { &message.message }
                            </p>
                        }
                    })
                }
            </div>

            <input
                style="width: 75%; padding: 8px;"
                value={(*input_message).clone()}
                oninput={oninput_message}
                onkeydown={onkeydown_message}
                placeholder="Type message here..."
            />

            <button
                style="padding: 8px 16px; margin-left: 8px;"
                onclick={onclick_send}
            >
                { "Send" }
            </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}