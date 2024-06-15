# `dyn_graphic`

> https://g.dyn.art

### Run Locally

```
cargo run --bin dyn_graphic --profile release
```

### Create OpenAPI

```
cargo run --bin cli --features cli generate-open-api --output-path ../../packages/types/src/graphic/resources/openapi-v1.yaml
```

## Load Testing

- [API Load Testing (Stress, Spike, Load, Soak)](https://www.youtube.com/watch?v=r-Jte8Y8zag)
- [Drill](https://github.com/fcsonline/drill/tree/master)
- [K6](https://k6.io/)

## Good To Know

### NodeJs vs Axum

https://github.com/ishtms/learn-nodejs-hard-way/blob/master/chapters/ch00-nodejs-faster-than-you-think.md
