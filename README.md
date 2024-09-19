# dav server

A dav server based on https://github.com/messense/dav-server-rs and actix-web.

dav-server-rs provides decent support for both MacOS and Windows while Nginx's dav module support badly for MacOS.

# Usage

```
Usage: dav [OPTIONS] --dest-dir <DEST_DIR> --username <USERNAME> --password <PASSWORD>

Options:
  -d, --dest-dir <DEST_DIR>        
  -l, --listen-addr <LISTEN_ADDR>  [default: 127.0.0.1:4918]
  -u, --username <USERNAME>        
  -p, --password <PASSWORD>        
  -h, --help                       Print help
  -V, --version                    Print version
```