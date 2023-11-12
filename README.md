# tp-wik-dps-tp01
## How it works
When the server receives a GET request on /ping, it returns the client's headers in json. Otherwise it will return a 404 error.
It is possible to change the listen port of the web server by setting the value of the "PING_LISTEN_PORT" environment variable. 
You can do it by doing :
```
export PING_LISTEN_PORT=8080
```
## Implementations
- Typescript (the main one)
- Rust