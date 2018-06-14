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

    log!(log::Level::Info, msg: { "A structured {name}", name = "log" }, kvs: {
        id: 1,
        #[log(display)]
        name: "log",
        #[log(path)]
        path: "./monkey-path",
    });
}
