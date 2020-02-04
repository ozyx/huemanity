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

**NOTE:** It is not unexpected that there will be early snags with different
lights. So if you do get a bug, please report it. I should be right on it.

## Shoutout:

Special thanks for the RustLang discord for helping when I needed support! ❤
Community is king!

## Immediate TODO list:

- Remote API flow. Allows you to control lights externally with a token.
- Examples in the repo

## Installation and Usage

### Install

**Note**: At the moment I rely on the end user having `cargo` and rust compiler in
order to install this. In future, once this tool has enough traction, release
binaries will be compiled and maybe even provided through package managers. PR's
will expedite this process!

Installing the latest version:

```shell
git clone git@github.com:finnkauski/huemanity.git
cd huemanity
cargo install --path .
```

Installing from [crates.io](https://crates.io/crates/huemanity) (might be outdated):

```shell
cargo install huemanity
```

### Usage

Here are a few simple use cases you might want to try once you have it installed:

```shell
# get a state of the lights found on your bridge
huemanity info

# turn lights on
huemanity all "{\"on\":true}"
# change color of all lights
huemanity all "{\"xy\":[1.0, 0.0]}"

# turn a light on
huemanity state "{\"on\":true}" 1
# change color of a given light
huemanity state "{\"xy\":[1.0, 0.0]}" 1


# get request sent to bridge and state printed out
huemanity debug

# discover bridges on the network (experimental)
huemanity discover
```

Even simpler, if you have a file with the state already recorded, you can do the
following:

```shell
huemanity all $(cat file_with_state.json)
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
