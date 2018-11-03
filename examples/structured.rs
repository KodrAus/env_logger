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

use log::kv::source;

fn main() {
    env_logger::init_from_env("MY_LOG_LEVEL");

    let correlation_id = 123;
    let user = "some user";

    struct MyTypeLol(i32);

    let v = MyTypeLol(1);
    let v = source::Value::any(&v, |v, visitor| visitor.visit_i64(v.0 as i64));

    log!(log::Level::Info, msg: { "This is the rendered {message}. It is not structured", message = "message" }, kvs: {
        correlation: correlation_id,
        user: user,
        any: v,
    });
}
