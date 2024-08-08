# Wat 2 do

## API

Directory: `api/`

### Basic

- [x] How to read and write files
- [x] TOML Config encoding/decoding
- [x] Formatters, linters, CI
- [x] [File structure of go projects](https://go.dev/doc/modules/layout)
- [x] Get base [gin HTTP server](https://github.com/gin-gonic/gin) going
- [ ] Unit test API endpoints
- [ ] ORM database interactions with [gorm](https://github.com/go-gorm/gorm)

### Later

- [ ] Job scheduling with [go-quartz](https://github.com/reugn/go-quartz)
- [ ] Use [minio](https://github.com/minio/minio-go/) to do object storage with config files
- [ ] Tracing with [jaeger](https://github.com/jaegertracing/jaeger)
- [ ] Implement webhooks onto the server such that clients can be notified of changes

## Site

Directory: `site/`

### Basic

- [x] Base [shadcn/ui](https://github.com/shadcn-ui/ui) + [next.js](https://github.com/vercel/next.js) files
- [x] Ping the API
