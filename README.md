# tide-trace

Minimum overhead [USDT](http://dtrace.org/guide/chp-usdt.html) middleware to dynamically trace [tide](https://github.com/http-rs/tide) with [BPF](http://www.brendangregg.com/blog/2019-01-01/learn-ebpf-tracing.html) or [DTrace](https://en.wikipedia.org/wiki/DTrace).

```
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

```
$ sudo apt-get install systemtap-sdt-dev bpftrace
``` 

#### fedora

```
$ sudo dnf install systemtap-sdt-devel bpftrace
```

### mac

**This middleware is intended to work on mac but has not been fully tested**

```
$ brew install make 
```

## example usage

For a prebuilt applications see examples folder


Make a project 
```
$ cargo init sample-project
```

Cargo.toml
```
tide = "0.11"
tide-trace = { git = "http://github.com/No9/tide-trace.git"}
async-std = { version = "1.6.0", features = ["unstable", "attributes"] }
```

main.rs
```
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

```
$ sudo bpftrace -p $(pgrep sample-project) tools/routes.bt 
$ curl http://localhost:8080/
```

## tests 

```
$ cargo build --examples 
$ sudo ./tests/test-all.sh
```