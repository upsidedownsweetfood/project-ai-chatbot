# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

### Ideas && References
Evaluate [Candle](https://github.com/huggingface/candle) in Rust
* [Docs](https://huggingface.github.io/candle/inference/hub.html)
* [Example of Pytorch Model in Candle](https://github.com/ToluClassics/candle-tutorial)
Implement Candle API server in [Actix](https://actix.rs/)
* [Example Directory](https://github.com/huggingface/candle/tree/main/candle-examples/examples)