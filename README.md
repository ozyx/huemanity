# Huemanity

A bare-bones package to control Phillips Hue lights written in Rust.

![Hue-Manatee](https://external-preview.redd.it/waqya42uIaxSwINQAuQK4p5JrQg4MyeafH5lutQRGqI.jpg?auto=webp&s=bc78d2cfe9fcb9320454b821f6ac8bd43b19afba)

Currently it is incredibly bare bones, I am using it as a stepping stone for a
future projects and might eventually just move my projects to a more developed
crate.

That said, if you know your HUE bridge IP on your local network, you can use the
`Bridge`, register you application and use the struct to send a `json!` created
state (`json!` is a `serde_json` macro).

I stream the development of this on [twitch.tv](https://www.twitch.tv/finnkauski)
And it is currently used to create
[this](https://www.youtube.com/watch?v=fEK2DofSwEE) project which links an
electric drumkit to my HUE lights.

## For more info:

This follows closely (basically wraps) the interactions described in the
[hue API get-started
post](https://developers.meethue.com/develop/get-started-2/) up to the point of
sending state to the lights.

## In development

- CLI wrapper binary for the tool to turn this into something people can use
  - currently it can ship json state strings from the CLI and thats about it

## Limitations and non-developments

1. Currently it does not discover your bridge on the network and you need to
   know your IP.
2. This doesn't have fancy state serialisation or such as of this point. Wether
   or not this is in scope is debatable in my head.
3. A lot of the `Result` Enums aren't handled correctly so the package is prone
   to panic
