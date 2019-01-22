/*!
Using `env_logger`.

Before running this example, try setting the `MY_LOG_LEVEL` environment variable to `info`:

```no_run,shell
$ export MY_LOG_LEVEL='info'
```

Also try setting the `MY_LOG_STYLE` environment variable to `never` to disable colors
or `auto` to enable them:

```no_run,shell
$ export MY_LOG_STYLE=never
```
*/

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
#[cfg(feature = "json")]
extern crate serde_json;

#[cfg(feature = "json")]
fn main() {
    use serde_json::json;
    use log::key_values::source;

    let mut builder = env_logger::Builder::from_env("MY_LOG_LEVEL");
    builder.json_format().init();

    let value = json!({
        "id": 15,
        "name": "Whiskers"
    });

    struct MyTypeLol(i32);

    let any = MyTypeLol(1);
    let any = source::Value::any(&any, |v, visitor| visitor.visit_i64(v.0 as i64));

    log!(log::Level::Info, msg: { "A structured {name}", name = "log" }, kvs: {
        id: 1,
        name: "log",
        path: "./monkey-path",
        details: value,
        any: any,
    });
}

#[cfg(not(feature = "json"))]
fn main() {
    panic!("compile with the `json` feature to run this example")
}