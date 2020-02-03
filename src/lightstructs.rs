// serde deserialisation
use serde::*;

/// This struct is just a mirror of the default reqwest methods
#[derive(PartialEq)]
pub enum RequestType {
    Get,
    Post,
    Put,
}

/// This object contains the state part  of each light
#[derive(Serialize, Deserialize, Debug)]
pub struct LightState {
    pub on: bool,
    pub bri: u8,
    pub hue: u32,
    pub sat: u8,
    pub effect: String,
    pub xy: [f32; 2],
    pub ct: u32,
    pub alert: String,
    pub colormode: String,
    pub mode: String,
    pub reachable: bool,
}

impl Default for LightState {
    fn default() -> Self {
        // TODO: find sensible values for the defaults
        LightState {
            on: false,
            bri: 254,
            hue: 8000,
            sat: 140,
            effect: "none".to_owned(),
            xy: [0.0, 0.0],
            ct: 366,
            alert: "select".to_owned(),
            colormode: "ct".to_owned(),
            mode: "homeautomation".to_owned(),
            reachable: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct LightSwUpdate {
    pub state: String,
    pub lastinstall: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LightCapabilities {
    pub certified: bool,
    pub control: LightCapabilityControl,
    pub streaming: LightStreamingCapabilities,
}

#[derive(Serialize, Deserialize, Debug)]
struct LightCT {
    pub min: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct LightCapabilityControl {
    pub mindimlevel: u64,
    pub maxlumen: u64,
    pub colorgamuttype: String,
    pub colorgamut: [[f32; 2]; 3],
    pub ct: LightCT,
}

#[derive(Serialize, Deserialize, Debug)]
struct LightStreamingCapabilities {
    pub renderer: bool,
    pub proxy: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct LightConfig {
    pub archetype: String,
    pub function: String,
    pub direction: String,
    pub startup: LightConfigStartup,
}

#[derive(Serialize, Deserialize, Debug)]
struct LightConfigStartup {
    pub mode: String,
    pub configured: bool,
}

// TODO: Decided which bits to expose
/// Light object representing the complete state of a light
#[derive(Serialize, Deserialize, Debug)]
pub struct Light {
    pub state: LightState,
    swupdate: LightSwUpdate,
    pub r#type: String,
    pub name: String,
    pub modelid: String,
    pub manufacturername: String,
    pub productname: String,
    capabilities: LightCapabilities,
    config: LightConfig,
    pub uniqueid: String,
    pub swversion: String,
    pub swconfigid: String,
    pub productid: String,
}

impl std::fmt::Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
Name: {}
On: {:?}
Color: {:?}
",
            self.name, self.state.on, self.state.xy
        )
    }
}

/// Helper object with some tweaks to serialisation. In order to
/// use this object you have to do one of the following:
/// ```
/// let state_1: SendableState = serde_json::from_str(r#"{"on":true}"#);
/// let state_2: SendableState = SendableState {on:true, ..SendableState::default()};
/// let state_3: SendableState = state!(on: true, xy: [1.0, 0.123])
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct SendableState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy: Option<[f32; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
}

impl Default for SendableState {
    /// Initialises the state as Nones
    fn default() -> Self {
        Self {
            on: None,
            bri: None,
            hue: None,
            sat: None,
            effect: None,
            xy: None,
            alert: None,
        }
    }
}

/// Super useful macro to create `SendibleState`
/// ```
/// // Usage example
/// let sendable_state: SendableState = state!(on: true, xy: [1.0, 0.0])
/// ```
#[macro_export]
macro_rules! state {
    ($($i:ident:$v:expr), *) => {
        SendableState {
            $($i: Some($v),) *
            ..SendableState::default()
        }
    };
}
