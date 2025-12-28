# avfaudio2-sys

Bindings to `AVFAudio/AVFAudio.h` generated with `bindgen`.

Based on https://github.com/augmented-audio/avfaudio-sys

## Supported platforms

- iphoneos (iOS)
- iphonesimulator (iOS Simulator)
- xros (visionos)
- xrsimulator (visionOS Simulator)
- macosx (macOS)
- watchos (watchOS)
- watchsimulator (watchOS Simulator)
- appletvos (tvOS)

## Development

### Build

```shell
cargo +nightly build -Zbuild-std --target aarch64-apple-visionos
```

### Run example

```shell
cargo run --example session
```

## Authors

Copyright (c) 2022 Pedro Tacla Yamada

Copyright (c) 2024-2025 Eugene Hauptmann

## License

[MIT](/LICENSE)
