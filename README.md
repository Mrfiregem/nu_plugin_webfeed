# nu_plugin_webfeed

A simple nushell plugin that wraps [feed-rs](https://github.com/feed-rs/feed-rs).
It currently has a single command, `feed fetch`, which will read a file on disk or online and parse it into a record with standardized fields for all three feed types.
