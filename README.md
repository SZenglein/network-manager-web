# Network Manager Web

Web API for managing network connections on Linux. Only WiFi for now.

Not to be confused with the Linux `NetworkManager` utility, we may support backends other than it. 
It **is** the only supported backend for now, though. 

## Motivation

The goal of this repository is a WiFi management web API 
that can be used to control wifi settings on an embedded device without having to resort to SSH access.

The settings are intentionally kept user-friendly as opposed to fully featured and aim to cover the most common use-cases.

Despite this motivation, feel free to contribute more functionality if you need it!

## Implementation

There is currently only one implementation for NetworkManager using the `nmrs` crate.

The axum app is designed as a library so it can easily be embedded inside another `axum` application.
However a simple example application is provided that is fully functional, albeit minimal.

The backend is currently abstracted using the `WifiBackend` trait. 
This avoids a reliance on NetworkManager or a specific implementation.

In addition, it opens up the possibility of adding more functionality, such as Bluetooth control.

## Limitations

Exposing passwords is not yet nicely implemented by `nmrs`, so we do not support it. Thus, also no QR-Code generation.


## License

This project is licensed under the [MIT license](./LICENSE).


### Contribution

Contributions are welcome, though keep in mind this project's motivation when adding new features.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you shall be licensed as MIT, without any additional
terms or conditions.
