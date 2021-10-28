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
# the [input] section controls the connection to Varnish
[input]
# the directory for the varnish instance. Default is /var/lib/varnish/<hostname>
path = /path/to/varnish/instance_dir
# how long to wait to connect to the varnish instance before failing
connect_timeout_secs = 10

# the [output] section controls the JSON output
[output]

# either "stdout" or "tcp".  Default "stdout"
destination = "tcp"

# hostname or IP of remote server. Required if destination = "tcp"
host = "127.0.0.1"

# port of remote server. Required if destination = "tcp"
port = 12345

# number of threads to start to send logs if destination = "tcp". Default is 2
sender_threads = 1


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

# Selects the log grouping type.  Default is "Vxid", possible values are "Request", or "Vxid"
grouping = "Request"

# Setting tail = true will start the cursor at the head of the logs on startup, ignoring what's currently in the buffer.
# Setting tail = false will read all logs from the beginning of the buffer on startup.
# tail = true may lose logs between restarts, tail = false may duplicate logs between restarts
# default is true
tail = true

# List of log record types to collect.  A list containing zero or more of "Request", "BackendRequest", "Session", "Raw".
# Default is [], which captures all records.
type_filter = [ "Request" ]

# List of log reasons to collect. Valid values: "Unknown", "Http1", "RxReq", "Esi", "Restart", "Pass", "Fetch", "BgFetch", "Pipe"
# Default is [], which captures all records.
reason_filter = [ "RxReq" ]
```
