# SugarFunge API

## Launch API server
```
cargo run
```

## Help
```
sugarfunge-api 0.1.0

USAGE:
    sugarfunge-api [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --db-uri <db>                  
    -l, --listen <listen>               [default: http://127.0.0.1:4000]
    -s, --node-server <node-server>     [default: ws://127.0.0.1:9944]
```

## Generate SugarFunge Types
```
subxt-cli metadata -f bytes > sugarfunge_metadata.scale
```

## Subscriptions

Ping
```
websocat ws://127.0.0.1:4000/ws 
```

