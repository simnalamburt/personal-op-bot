personal-op-bot
========
A simple and ignorant IRC OP maintenance bot.

```bash
cargo build

# Prepare config.toml using config.toml.example
cargo run
```

### Using Docker
```bash
# Prepare ~/personal-op-bot/config.toml then run below:
docker run -d --restart=always \                                                                                    
  --mount type=bind,source=${HOME}/personal-op-bot,target=/a \                                                  
  ghcr.io/simnalamburt/personal-op-bot:1.0.1
```

&nbsp;

--------
*personal-op-bot* is primarily distributed under the terms of both the [Apache
License (Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
