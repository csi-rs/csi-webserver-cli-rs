# csi-webclient

Native Rust desktop client for controlling `csi-webserver` through REST and WebSocket APIs.

This crate ships the full application and exposes modules that keep UI code, state, and side
effects separate so behavior stays maintainable as protocol features evolve.

## Module Layout

- `app`: top-level orchestration that translates `UserIntent` values into core commands and
  applies core events back into app state.
- `state`: source-of-truth state models, user intents, and API-facing enum mappings.
- `core`: side-effect layer for HTTP requests, WebSocket receive loop, and worker runtime.
- `ui`: rendering modules for each tab (dashboard/config/control/stream) without direct IO.

## Runtime Flow

1. UI interactions enqueue intents in `state::AppState`.
2. `app::CsiClientApp` drains intents and submits `core::messages::CoreCommand` values.
3. `core` executes HTTP/WebSocket side effects on a worker thread and Tokio runtime.
4. `core` emits `core::messages::CoreEvent` values over channels.
5. `app::CsiClientApp` polls and applies events to state on each frame.

This design avoids blocking work in the egui frame callback and keeps network concerns out of
view code.

## API Coverage

The client targets these server routes:

- `GET /api/config`
- `POST /api/config/reset`
- `POST /api/config/wifi`
- `POST /api/config/traffic`
- `POST /api/config/csi`
- `POST /api/config/collection-mode`
- `POST /api/config/log-mode`
- `POST /api/config/output-mode`
- `POST /api/control/start`
- `POST /api/control/reset`
- `GET /api/ws`

For detailed request/response behavior and payload fields, see:

- <https://github.com/csi-rs/csi-webclient-rs/blob/main/docs/HTTP_API.md>

## Protocol Values Used By The Client

- Wi-Fi modes: `sta`, `monitor`, `sniffer`
- Collection modes: `collector`, `listener`
- Log modes: `text`, `array-list`, `serialized`
- Output modes: `stream`, `dump`, `both`

## Notes

- HTTP success is treated as status code in the `2xx` range.
- API responses are parsed best-effort from either a generic envelope
  (`success`/`message`/`data`) or direct JSON payload.
- WebSocket text and binary messages are both stored as frame bytes for stream inspection.