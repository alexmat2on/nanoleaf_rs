use std::vec::Vec;
use serde_json::json;

mod api;
pub mod color;

use color::NanoLeafColor;
use api::NanoLeafAPI;
use api::effects::{PluginOption, EffectDetails};

pub struct NanoLeafClient {
    access_token: String,
    host: String,
}

impl NanoLeafClient {
    pub fn new(access_token: &str, host: &str) -> NanoLeafClient {
        NanoLeafClient {
            access_token: access_token.to_string(),
            host: host.to_string()
        }
    }

    fn api(&self, endpoint: &str) -> String {
        format!("{}/api/v1/{}/{}", self.host, self.access_token, endpoint.to_string())
    }

    pub async fn info(&self) -> Result<NanoLeafAPI, Box<dyn std::error::Error>> {
        let endpoint = self.api("");
        let resp = reqwest::get(&endpoint)
            .await?
            .json::<NanoLeafAPI>()
            .await?;
        
        Ok(resp)
    }

    pub async fn on(&self, value: bool) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint = self.api("state/on");
        let body = json!({ "on": api::state::On { value } });

        let client = reqwest::Client::new();
        let resp = client.put(&endpoint)
            .json(&body)
            .send()
            .await?;

        Ok(resp.text().await?)
    }

    pub async fn select(&self, name: Option<String>) -> Result<Option<String>, Box<dyn std::error::Error>> {
        match name {
            None => {
                let endpoint = self.api("effects/select");
                let resp = reqwest::get(&endpoint)
                    .await?
                    .json::<String>()
                    .await?;
                
                Ok(Some(resp))
            },
            Some(effect) => {
                let endpoint = self.api("effects");
                let body = json!({"select": effect});
                
                let client = reqwest::Client::new();
                let resp = client.put(&endpoint)
                    .json(&body)
                    .send()
                    .await?;
                
                Ok(Some(resp.text().await?))
            }
        }
    }

    pub async fn request_effect(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = self.api("effects");
        let body = json!({
            "write": {
                "command": "request",
                "animName": &name,
            }
        });
        
        let client = reqwest::Client::new();
        let _ = client.put(&endpoint)
            .json(&body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn add_wheel_effect(
        &self,
        name: String,
        palette: Vec<NanoLeafColor>
    ) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint = self.api("effects");
        
        let plugin_options = vec![
            PluginOption::TransTime(24),
            PluginOption::DelayTime(0),
            PluginOption::LinDirection("right"),
            PluginOption::Loop(true)
        ];
                
        let body = json!({
            "write": EffectDetails {
                command: "add",
                version: "2.0",
                anim_type: "plugin",
                anim_name: &name,
                color_type: "HSB",
                has_overlay: false,
                plugin_uuid: "027842e4-e1d6-4a4c-a731-be74a1ebd4cf",
                plugin_type: "color",
                plugin_options,
                palette
            }
        });

        let client = reqwest::Client::new();
        let resp = client.put(&endpoint)
            .json(&body)
            .send()
            .await?;
                
        Ok(resp.text().await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::NanoLeafClient;
    use crate::color::NanoLeafColor;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, body_json};
    use serde_json::json;
    
    #[tokio::test]
    async fn get_info_test() {        
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_json(json!({
            "name": "Light Panels Name",
            "serialNo": "S16331A0217",
            "manufacturer": "Nanoleaf",
            "firmwareVersion": "1.5.0",
            "model": "NL22",
            "state": {
                "on": {
                    "value": false
                },
                "brightness": {
                    "value": 100,
                    "max": 100,
                    "min": 0
                },
                "hue": {
                    "value": 0,
                    "max": 360,
                    "min": 0
                },
                "sat": {
                    "value": 0,
                    "max": 100,
                    "min": 0
                },
                "ct": {
                    "value": 4000,
                    "max": 100,
                    "min": 0
                },
                "colorMode": "effect"
            },
            "effects": {
                "select": "Flames",
                "effectsList": [
                    "Color Burst",
                    "Flames",
                    "Forest",
                    "Inner Peace",
                    "Nemo",
                    "Northern Lights",
                    "Romantic",
                    "Snowfall"
                ]
            },
            "panelLayout": {
                "layout": {
                    "numPanels": 2,
                    "sideLength": 150,
                    "positionData": [
                        {
                            "panelId": 107,
                            "x": -74,
                            "y": 43,
                            "o": 180,
                            "shapeType": 0
                        },
                        {
                            "panelId": 114,
                            "x": -149,
                            "y": 0,
                            "o": 360,
                            "shapeType": 0
                        }
                    ]
                },
                "globalOrientation": {
                    "value": 120,
                    "max": 360,
                    "min": 0
                }
            },
            "rhythm": {
                "rhythmConnected": true,
                "rhythmActive": true,
                "rhythmId": 309,
                "hardwareVersion": "1.4",
                "firmwareVersion": "1.7-R",
                "auxAvailable": true,
                "rhythmMode": 1,
                "rhythmPos": {
                    "x": 299,
                    "y": -86,
                    "o": 300
                }
            }
        }));


        Mock::given(method("GET"))
            .and(path("/api/v1/TOKEN/"))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;
        
     
        let client = NanoLeafClient::new("TOKEN", &mock_server.uri());
        let result = client.info().await.expect("Something went wrong");
        
        assert_eq!(result.serial_no, "S16331A0217");
    }

    #[tokio::test]
    async fn put_on_test() {        
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(200);

        Mock::given(method("PUT"))
            .and(path("/api/v1/TOKEN/state/on"))
            .and(body_json(json!({ "on": { "value" : true }})))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;
        
     
        let client = NanoLeafClient::new("TOKEN", &mock_server.uri());
        let result = client.on(true).await.expect("Something went wrong");
        
        assert_eq!(result, "");
    }
    
    #[tokio::test]
    async fn get_select_test() {        
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(200)
            .set_body_json(json!("Solid"));

        Mock::given(method("GET"))
            .and(path("/api/v1/TOKEN/effects/select"))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;
        
     
        let client = NanoLeafClient::new("TOKEN", &mock_server.uri());
        let result = client.select(None).await.expect("Something went wrong").unwrap();
        
        assert_eq!(result, "Solid");
    }

    #[tokio::test]
    async fn put_wheel_effect() {
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(200);

        Mock::given(method("PUT"))
            .and(path("/api/v1/TOKEN/effects"))
            .and(body_json(json!({
                "write": {
                    "animName": "My Animation",
                    "animType": "plugin",                    
                    "colorType": "HSB",
                    "command": "add",
                    "hasOverlay": false,
                    "palette": [
                        {
                            "brightness": 100,
                            "hue": 0,
                            "saturation": 100
                        },
                        {
                            "brightness": 100,
                            "hue": 120,
                            "saturation": 100
                        },
                        {
                            "brightness": 100,
                            "hue": 240,
                            "saturation": 100
                        }
                    ],
                    "pluginOptions": [
                        {
                            "name": "transTime",
                            "value": 24
                        },
                        {
                            "name": "delayTime",
                            "value": 0
                        },
                        {
                            "name": "linDirection",
                            "value": "right"
                        },
                        {
                            "name": "loop",
                            "value": true
                        }
                    ],
                    "pluginType": "color",
                    "pluginUuid": "027842e4-e1d6-4a4c-a731-be74a1ebd4cf",
                    "version": "2.0",                    
                }
            })))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = NanoLeafClient::new("TOKEN", &mock_server.uri());
        let _ = client.add_wheel_effect(
            "My Animation".to_string(),
            vec![NanoLeafColor::new(0, 100, 100), NanoLeafColor::new(120, 100, 100), NanoLeafColor::new(240, 100, 100)]
        ).await.expect("Something went wrong");
    }
}
