# vapi-logger

Varnish logging via varnishapi shared memory segment.
Emits JSON for easier ingestion into other logging systems.

## Usage

```
vapi-logger 0.1.0

USAGE:
    vapi-logger <config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <config>    Path to logger config file
```

## Config Format

The config is in TOML format.

Example:

```
# the [output] section controls the JSON output
[output]

# either "stdout" or "tcp".  Default "stdout"
destination = "tcp"

# hostname or IP of remote server. Required if destination = "tcp"
host = "127.0.0.1"

# port of remote server. Required if destination = "tcp"
port = 12345


# the [logging] section controls what gets logged
[logging]

# Array of request headers that will be added to log output. Default []
request_headers = ["Host"]

# Array of response headers that will be added to log output. Default []
response_headers = ["Content-Type"]

# Static tags that will be added to the "meta" section of the log output.
# Useful for environment or server identification.  Default {}
tags = { environment = "prod" }

# VSL Query used to filter the logs. Default ""
# See https://varnish-cache.org/docs/6.0/reference/vsl-query.html for examples
query = ''

# Select which log field is used for the client IP Address. Either "request" or "header"
# Default is "request", which is the connecting IP
ip_source = "header"

# Required if ip_source = "header".
# Selects which header to use to determine client IP,

# for example X-Forwarded-For or CF-Connecting-IP
ip_source_header = "X-Forwarded-For"
```
