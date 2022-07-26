# SugarFunge API

## Usage
- Copy the environment file as **.env**
```
cp .env.example .env
```

- Update the KEYCLOAK_PUBLIC_KEY on the .env file file using the rsa-generated provider public key that can be found on Realms -> Sugarfunge -> Realm Settings -> Keys

- Update the KEYCLOAK_CLIENT_SECRET on the .env file using the Secret that can be found on Realms -> Sugarfunge -> Clients -> Sugarfunge-api -> Credentials -> Regenerate Secret

## Environment configuration

- Default environment file: **.env**
- Example environment file: **.env.example**

| Variable Name               | Description                                   |
| --------------------------- | --------------------------------------------- |
| KEYCLOAK_USERNAME           | Keycloak user for Sugarfunge API              |
| KEYCLOAK_USER_PASSWORD      | Password of the Keycloak user                 |
| KEYCLOAK_CLIENT_ID          | Keycloak client ID of Sugarfunge API          |
| KEYCLOAK_CLIENT_SECRET      | Keycloak client secret of Sugarfunge API      |
| KEYCLOAK_HOST               | Keycloak service host                         |
| KEYCLOAK_REALM              | Keycloak realm for Sugarfunge                 |
| KEYCLOAK_PUBLIC_KEY         | Keycloak RS256 public key of Sugarfunge realm |

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

