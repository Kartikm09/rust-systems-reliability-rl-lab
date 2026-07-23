# Docker verification

Verified locally on 2026-07-23 with Docker Engine 29.5.2 on an ARM64 Linux virtual
machine. The image was rebuilt from the current Cargo manifests and lockfile.

```console
$ docker compose up -d --build --wait
Container event-lab Healthy
$ curl --fail --silent http://127.0.0.1:8082/health
{"status":"ok"}
$ docker compose ps
event-lab   Up (healthy)   0.0.0.0:8082->8082/tcp
$ docker compose down
```

This verifies the documented local container workflow and health endpoint. It is not
a production deployment or a multi-architecture compatibility claim.
