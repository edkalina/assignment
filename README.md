# Sprout Therapy Assignment solution

This solution is implemented with Rust as HTTP server.

## Installation

1. Clone this repo
2. Run warp server with `cargo run`
3. Open `http://localhots:3030/` or use `/api/assignment` endpoint

## Testing

Run tests with

```sh
cargo test
```

## API endpoint

Solution can be used with `/api/assignment` endpoint. It accepts POST request with json:

```json
{
  "input": "A: true\nB: true...",
  "substitution": "base"
}
```

## Dependencies

`warp` - for http server

`serde_yaml` - for input parsing

`once_cell` - for using global values
