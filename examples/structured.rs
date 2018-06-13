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

    log!(log::Level::Info, msg: { "A structured {name}", name = "log" }, properties: {
        id: 1,
        #[log(display)]
        name: "log",
        #[log(path)]
        path: "./monkey",
    });

    /*
    log!(log::Level::Info; "A structured {name}", name = "log"; {
        id: 1,
        #[log(display)]
        name: "log",
    });

    log!(log::Level::Info; "A structured {name}", name = "log"; ctxt = {
        id: 1,
        #[log(display)]
        name: "log",
    });

    log!(log::Level::Info; "A structured {name}", name = "log"; ctxt = {
        id = 1,
        #[log(display)]
        name = "log",
    });

    log!(log::Level::Info; "A structured {name}", name = "log"; ctxt: {
        id: 1,
        #[log(display)]
        name: "log",
    });

    log!(log::Level::Info; "A structured {name:?}", name = "log"; id = 1, #[log(display)] name = "User");
    log!(log::Level::Info; "A structured {name:?}", name = "log"; id = 1, name = "log");

    // log as display
    log!(log::Level::Info; "A structured {name:?}", name = "log"; id: = 1);
    log!(log::Level::Info; "A structured {name:?}", name = "log"; #[log(display)] id = 1);
    log!(log::Level::Info; "A structured {name:?}", name = "log"; #[log(fmt = Display::fmt)] id = 1);

    // log as debug
    log!(log::Level::Info; "A structured {name:?}", name = "log"; id:? = 1);
    log!(log::Level::Info; "A structured {name:?}", name = "log"; #[log(debug)] id = 1);
    log!(log::Level::Info; "A structured {name:?}", name = "log"; #[log(fmt = Debug::fmt)] id = 1);

    // log as serde
    log!(log::Level::Info; "A structured {name:?}", name = "log"; #[log(serde)] id = 1);
    */
}
