# tide-trace

Minimum overhead [USDT](http://dtrace.org/guide/chp-usdt.html) middleware to dynamically trace [tide](https://github.com/http-rs/tide) with [BPF](http://www.brendangregg.com/blog/2019-01-01/learn-ebpf-tracing.html) or [DTrace](https://en.wikipedia.org/wiki/DTrace).

```sh
$ cargo run --example histogram

$ sudo bpftrace -p $(pgrep histogram) tools/route-histogram.bt
Attaching 2 probes...

$ curl http://localhost:8080/route1 && curl http://localhost:8080/route2


^C
@us[GET, /favicon.ico]: 
[32, 64)               1 |@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@|

@us[GET, /route1]: 
[32, 64)               2 |@@@@@@@@@@@@@@@@@                                   |
[64, 128)              6 |@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@|
[128, 256)             2 |@@@@@@@@@@@@@@@@@                                   |

@us[GET, /route2]: 
[32, 64)               2 |@@@@@@@@@@@@@@@@@@@@                                |
[64, 128)              5 |@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@|
[128, 256)             4 |@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           |
```

## pre-requisites

### linux

#### ubuntu

```sh
$ sudo apt-get install systemtap-sdt-dev bpftrace
``` 

#### fedora

```sh
$ sudo dnf install systemtap-sdt-devel bpftrace
```

### mac

**This middleware is intended to work on mac but has not been fully tested**

```sh
$ brew install make 
```

## example usage

For a prebuilt applications see examples folder


Make a project 
```sh
$ cargo init sample-project
```

Cargo.toml
```toml
tide = "0.15"
async-std = { version = "1.8.0", features = ["attributes"] }
tide-trace = { version = "0.4.0" }
```

main.rs
```rust
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.middleware(tide_trace::USDTMiddleware::new(0));
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
```

## trace requests

```sh
$ sudo bpftrace -p $(pgrep sample-project) tools/routes.bt 
$ curl http://localhost:8080/
```

## tests 

```sh
$ cargo build --examples 
$ sudo ./tests/test-all.sh
```
