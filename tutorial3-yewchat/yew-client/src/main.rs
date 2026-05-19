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
        <>
            <style>
                {"
                    body {
                        margin: 0;
                        min-height: 100vh;
                        background: linear-gradient(135deg, #eef2ff, #f8fafc);
                        font-family: Arial, sans-serif;
                    }

                    .page {
                        max-width: 860px;
                        margin: 0 auto;
                        padding: 40px 20px;
                    }

                    .app-card {
                        background: white;
                        border-radius: 24px;
                        box-shadow: 0 20px 50px rgba(15, 23, 42, 0.14);
                        overflow: hidden;
                        border: 1px solid #e2e8f0;
                    }

                    .header {
                        padding: 28px;
                        background: #1e293b;
                        color: white;
                    }

                    .title {
                        margin: 0;
                        font-size: 32px;
                    }

                    .subtitle {
                        margin-top: 8px;
                        opacity: 0.85;
                    }

                    .status {
                        display: inline-block;
                        margin-top: 14px;
                        padding: 6px 12px;
                        border-radius: 999px;
                        background: #22c55e;
                        color: #052e16;
                        font-weight: bold;
                        font-size: 14px;
                    }

                    .chat-area {
                        min-height: 360px;
                        max-height: 420px;
                        overflow-y: auto;
                        padding: 24px;
                        background: #f8fafc;
                    }

                    .message-row {
                        display: flex;
                        gap: 12px;
                        align-items: flex-start;
                        margin-bottom: 16px;
                    }

                    .avatar {
                        width: 42px;
                        height: 42px;
                        border-radius: 50%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        background: #dbeafe;
                        font-size: 22px;
                    }

                    .bubble {
                        background: white;
                        border: 1px solid #e2e8f0;
                        border-radius: 18px;
                        padding: 12px 16px;
                        max-width: 70%;
                    }

                    .sender {
                        font-weight: bold;
                        color: #1e293b;
                        margin-bottom: 4px;
                    }

                    .text {
                        color: #334155;
                        line-height: 1.4;
                    }

                    .input-area {
                        display: flex;
                        gap: 12px;
                        padding: 20px;
                        background: white;
                        border-top: 1px solid #e2e8f0;
                    }

                    .input {
                        flex: 1;
                        padding: 14px 16px;
                        border-radius: 14px;
                        border: 1px solid #cbd5e1;
                        font-size: 16px;
                    }

                    .button {
                        padding: 14px 20px;
                        border: none;
                        border-radius: 14px;
                        background: #2563eb;
                        color: white;
                        font-weight: bold;
                        cursor: pointer;
                    }

                    .footer {
                        text-align: center;
                        padding: 16px;
                        color: #64748b;
                        font-size: 14px;
                    }
                "}
            </style>

            <div class="page">
                <div class="app-card">
                    <div class="header">
                        <h1 class="title">{ "💬 Rafsan YewChat" }</h1>
                        <div class="subtitle">
                            { "A simple asynchronous WebSocket chat client built with Rust, WASM, and Yew." }
                        </div>
                        <div class="status">
                            { format!("Status: {}", *connection_status) }
                        </div>
                    </div>

                    <div class="chat-area">
                        {
                            if messages.is_empty() {
                                html! {
                                    <p style="color: #64748b; text-align: center; margin-top: 120px;">
                                        { "No messages yet. Open another browser tab and start chatting." }
                                    </p>
                                }
                            } else {
                                html! {
                                    <>
                                        {
                                            for messages.iter().map(|message| {
                                                html! {
                                                    <div class="message-row">
                                                        <div class="avatar">{ "🦀" }</div>
                                                        <div class="bubble">
                                                            <div class="sender">{ &message.user }</div>
                                                            <div class="text">{ &message.message }</div>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                        }
                                    </>
                                }
                            }
                        }
                    </div>

                    <div class="input-area">
                        <input
                            class="input"
                            value={(*input_message).clone()}
                            oninput={oninput_message}
                            onkeydown={onkeydown_message}
                            placeholder="Write a message and press Enter..."
                        />

                        <button class="button" onclick={onclick_send}>
                            { "Send" }
                        </button>
                    </div>

                    <div class="footer">
                        { "Created for Module 10 Asynchronous Programming — Rafsan's Computer" }
                    </div>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}