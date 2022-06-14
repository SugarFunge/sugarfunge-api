# SugarFunge API

## Usage
- Copy the environment file as **.env**
```
cp .env.example .env
```

- Update the KEYCLOAK_PK on the main file using the rsa-generated provider public key that can be found on Realms -> Sugarfunge -> Realm Settings -> Keys

- Update the KEYCLOAK_CLIENT_SECRET on the .env file using the Secret that can be found on Realms -> Sugarfunge -> Clients -> Sugarfunge-api -> Credentials -> Regenerate Secret

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
