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

fn main() {
    env_logger::init_from_env("MY_LOG_LEVEL");

    let correlation_id = 123;
    let user = "some user";

    log!(log::Level::Info, msg: { "This is the rendered {message}. It is not structured", message = "message" }, kvs: {
        correlation: correlation_id,
        user: user,
    });
}
