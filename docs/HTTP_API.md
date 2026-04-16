# HTTP and WebSocket API Reference

This reference documents the endpoints and payloads that `csi-webclient` currently uses.

## Base Addresses

- HTTP base URL: `http://{host}:{port}`
- WebSocket URL: `ws://{host}:{port}/api/ws`

Default host/port in the app: `127.0.0.1:3000`.

## Config Endpoints

### `GET /api/config`

- Request body: none
- Purpose: fetch current server/device configuration

### `POST /api/config/reset`

- Request body: none
- Purpose: reset configuration to backend defaults

### `POST /api/config/wifi`

- Request body (JSON):

```json
{
  "mode": "sta | monitor | sniffer",
  "sta_ssid": "string or null",
  "sta_password": "string or null",
  "channel": 1
}
```

Notes:

- `sta_ssid` and `sta_password` are sent as `null` when input is empty.
- `channel` is optional and can be `null` when input is empty.
- `channel` must parse as `u16` when provided.

### `POST /api/config/traffic`

- Request body (JSON):

```json
{
  "frequency_hz": 100
}
```

Notes:

- `frequency_hz` is required and must parse as `u16`.

### `POST /api/config/csi`

- Request body (JSON):

```json
{
  "disable_lltf": false,
  "disable_htltf": false,
  "disable_stbc_htltf": false,
  "disable_ltf_merge": false,
  "disable_csi": false,
  "disable_csi_legacy": false,
  "disable_csi_ht20": false,
  "disable_csi_ht40": false,
  "disable_csi_su": false,
  "disable_csi_mu": false,
  "disable_csi_dcm": false,
  "disable_csi_beamformed": false,
  "csi_he_stbc": 0,
  "val_scale_cfg": 0
}
```

Notes:

- `csi_he_stbc` and `val_scale_cfg` are required and must parse as `u8`.

### `POST /api/config/collection-mode`

- Request body (JSON):

```json
{
  "mode": "collector | listener"
}
```

### `POST /api/config/log-mode`

- Request body (JSON):

```json
{
  "mode": "text | array-list | serialized"
}
```

### `POST /api/config/output-mode`

- Request body (JSON):

```json
{
  "mode": "stream | dump | both"
}
```

## Control Endpoints

### `POST /api/control/start`

- Request body: optional JSON

```json
{
  "duration": 30
}
```

Notes:

- If duration input is empty, the client sends no request body.
- If provided, `duration` must parse as `u64`.

### `POST /api/control/reset`

- Request body: none
- Purpose: reset/stop collection session and device state

## WebSocket Stream

### `GET /api/ws`

- Upgraded to WebSocket by the client.
- Binary frames are forwarded as raw bytes.
- Text frames are converted to bytes and handled through the same frame path.

## Response Handling in Client

- HTTP status `2xx` is considered success.
- Empty response body:
  - success => "Request completed"
  - failure => "Request failed"
- Non-empty response body:
  - parsed best-effort as JSON
  - if envelope fields exist (`message`, `data`), they are used
  - otherwise fallback message is either generic success text or a truncated error body