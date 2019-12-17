# Huemanity

A package to control Phillips Hue lights written in Rust.

![Hue-Manatee](https://external-preview.redd.it/waqya42uIaxSwINQAuQK4p5JrQg4MyeafH5lutQRGqI.jpg?auto=webp&s=bc78d2cfe9fcb9320454b821f6ac8bd43b19afba)

Currently it is incredibly bare bones, I am using it as a stepping stone for a
future project. That said, if you have your app key registered with the bridge
and you know you HUE bridge IP on your local network, you can use the `Bridge`
struct to send a `json!` made state change to your lights.

I stream the development of this on [twitch.tv](https://www.twitch.tv/finnkauski)

## For more info:
This follows closely (basically wraps) the interactions described in the
[hue API get-started post](https://developers.meethue.com/develop/get-started-2/).

## In development

- CLI wrapper binary for the tool to turn this into something people can use
- Automated registration with the bridge using `ssdp`
- Tests and documentations

## Limitations

Currently you need to know you bridge IP address on your network and perform the
handshake as described
[here.](https://developers.meethue.com/develop/get-started-2/)
