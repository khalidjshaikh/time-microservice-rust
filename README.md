# time-microservice-rust

A small Rust microservice that returns the current date and time for the root path (`/`).

## Endpoints

- `GET /` — returns the current date and time as JSON.
- `GET /healthz` — liveness probe.
- `GET /readyz` — readiness probe.

## Configuration

| Env var | Default | Description |
|---------|---------|-------------|
| `PORT`  | `8080`  | HTTP listen port |
| `TZ`    | (local) | Timezone, e.g. `UTC` |
| `VERSION` | `dev` | Version string |

## Build

```bash
./scripts/build.sh
```

## Test

```bash
./scripts/test.sh
```

## Docker

```bash
./scripts/package.sh
docker run --rm -p 8080:8080 time-microservice-rust:latest
```

## Deploy to Kubernetes

```bash
./scripts/deploy.sh
```

## License

MIT
EOF
cd /Users/kshaikh/git/rust/time-microservice-rust; git add README.md; git commit -m "Add README"; git push 2>&1 | tail -3