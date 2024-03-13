# atproto-sdk-rust

```sh
$ just test
```

## example

```sh
$ export ATPROTO_BASE_URL=https://bsky.social
$ just test-full
```

## openapi

```sh
$ libninja gen --lang rust --repo lib/atproto -o . Atproto openapi.json
```

## ref

- https://swagger.io
- https://github.com/kurtbuilds/libninja
- https://github.com/rdmurphy/atproto-openapi-types

