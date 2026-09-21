![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/data5tream/enano-web-test/lint.yml?style=flat-square&label=Lints)
![Crates.io version](https://img.shields.io/crates/v/enano-web-test?style=flat-square&label=crates.io%20version&link=https%3A%2F%2Fcrates.io%2Fcrates%2Fenano-web-test)

# enano-web-test
Minimal web service for testing networking software and setups

```shell
docker pull ghcr.io/data5tream/enano-web-test:latest
docker run --rm -p 8080:8080 ghcr.io/data5tream/enano-web-test:latest
```

## Configuration

Configure via environment variables

- `CONTAINER_NAME` - Name of the container, defaults to `enano-web-test`
- `LOG_LEVEL` - Log level, defaults to `info`

## Endpoints

- `/` - Returns the value of the `CONTAINER_NAME` environment variable or `enano-web-test`
- `/ping` - Pong
- `/status/<u16>` - Returns the status code provided in the path
- `/headers` - Returns all headers received by the server
