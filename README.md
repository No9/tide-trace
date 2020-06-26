# tide-trace

USDT trace middleware for dynamic tracing with BPF or Dtrace.

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

## example usage

For a prebuilt example application see https://github.com/No9/tide-trace-test 


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

see https://github.com/No9/tide-trace-test