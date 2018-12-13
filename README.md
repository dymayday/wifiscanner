# wifiscanner

This is my fork from [on crates.io](https://crates.io/crates/wifiscanner) to add use an interface name as argument to the *scanner*.
A crate to list WiFi hotspots in your area.

As of v0.3.x now supports OSX and Linux. Windows to
follow.

Inspired by Maurice Svay's node-wifiscanner (https://github.com/mauricesvay/node-wifiscanner)

Tests shameless pilfered from Christian Kuster's node-wifi-scanner (https://github.com/ancasicolica/node-wifi-scanner)

Full documentation can be found [here](https://booyaa.github.io/wifiscanner/wifiscanner/index.html).

# Usage

This crate is [on crates.io](https://crates.io/crates/wifiscanner) and can be
used by adding `wifiscanner` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
wifiscanner = "0.3.*"
```

and this to your crate root:

```rust
extern crate wifiscanner;
```
# Example

```rust
use wifiscanner;
// This is the my wireless interface's name on my machine
let iface: &str = "wlp2s0"
println!("{:?}", wifiscanner::scan(iface));
```

Alternatively if you've cloned the the Git repo, you can run the above example
using: `cargo run --example scan`.

# Changelog

- 0.3.6 - crates.io metadata update
- 0.3.5 - remove hardcoded path for iwlist (props to @alopatindev)
- 0.3.4 - initial stable release


# Copyright

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.
