<p align="center"><img align="left" src="meta/logo.png" width="350px"></p>

# Huemanity

A bare-bones package to control Phillips Hue lights written in Rust.

This `CLI` and `crate` is designed to serialise and deserialise lights from the
Philips Hue API.

The `CLI` is a bit underdeveloped at the moment, however the general `crate`
works well. The central object (the `Bridge`) gets instantiated and is then able
to send state to each individual light.

**NOTE:** Currently the `Bridge` object needs you to know the `ip` that your Hue
Bridge is assigned on your network. Once that is known you are able to register
the application and send commands.

## For more info:

This follows closely (basically wraps) the interactions described in the
[hue API get-started
post](https://developers.meethue.com/develop/get-started-2/) up to the point of
sending state to the lights.

## Limitations and non-developments

- Currently it does not discover your bridge on the network and you need to
  know your IP.

## Watch this development

I stream the development of this on [twitch.tv](https://www.twitch.tv/finnkauski)
And it is currently used to create
[this](https://www.youtube.com/watch?v=fEK2DofSwEE) project which links an
electric drumkit to my HUE lights.
