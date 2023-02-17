use crate::State;

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    Extension,
};

pub async fn handler(ws: WebSocketUpgrade, Extension(state): Extension<State>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

pub async fn handle_socket(mut socket: WebSocket, _state: State) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
