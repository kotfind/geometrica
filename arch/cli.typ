= CLI crate

== Usage

```
geometrica-cli [OPTIONS] [filename]
```

#figure(table(
    columns: 2,
    align: left,

    table.header[*Flag*][*Description*],

    `-i, --no-input`,
    [Don't process input input.],

    `-o, --no-output`,
    [Don't print output.],

    `-h, --host`,
    [Specify custom server host name. Default is `127.0.0.1`.],

    `-p, --port`,
    [Specify custom server port. Default is *TODO*.],

    `-s, --no-server-init`,
    [By default if host is `127.0.0.1` and server is not running, than server is
    started. This flag disables this behaviour.]
))
