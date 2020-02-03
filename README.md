<p align="center"><img align="left" src="meta/logo.png" width="350px"></p>

# Huemanity

A bare-bones package to control Phillips Hue lights written in Rust.

This `CLI` and `crate` is designed to serialise and deserialise lights from the
Philips Hue API and send state to the lights.

The `CLI` is a bit underdeveloped at the moment, however the general `crate`
works well. The central object (the `Bridge`) gets instantiated and is then able
to send state to each individual light.

**NOTE:** Currently the `Bridge` object needs you to know the `ip` that your Hue
Bridge is assigned on your network. Once that is known you are able to register
the application and send commands.

## Usage

### Install

Installing the latest version:

```shell
git clone git@github.com:finnkauski/huemanity.git
cd huemanity
cargo install huemanity
```

Installing from crates.io (might be outdated):

```shell
cargo install huemanity
```

### Usage

The simplest use case is passing a state to all lights.

```shell
# turn lights on
huemanity all --state "{\"on\":true}"
# change color
huemanity all --state "{\"xy\":[1.0, 0.0]}"
```

## For more info:

This follows closely (basically wraps) the interactions described in the
[hue API get-started
post](https://developers.meethue.com/develop/get-started-2/) up to the point of
sending state to the lights.

## Contributing

If you would like to contribute here are a few things that need PRs:

- The `Bridge.state_all` sequentially to each light, this needs a bit more
  concurrency so requests get sent in one go. The `reqwest` library might have
  an `async` client so that might need to be implemented.

- The `CLI` needs a much better wrapping and functionality

- I do not like that the end user needs to know the bridge ip address. Ideally
  that would be automatically detected.

## Watch this development

I stream the development of this on [twitch.tv](https://www.twitch.tv/finnkauski)
And it is currently used to create
[this](https://www.youtube.com/watch?v=fEK2DofSwEE) project which links an
electric drumkit to my HUE lights.
