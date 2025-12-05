// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WSP_HealthActionEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WSP_HealthActionEvent {
    #[serde(flatten)]
    pub base: MSFT_HealthActionEvent,
}

impl WSP_HealthActionEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_HealthActionEvent::new(),
        }
    }


/// This method manually fires alerts

    /// * `health_action_alert` - Copy of the alert payload to be fired (WSP_HealthActionEvent)

    /// * `return_value` -  (u32)
    pub fn fire_alert(&self, health_action_alert: WSP_HealthActionEvent) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HealthActionAlert".to_string(), value: health_action_alert.into() });
        self.invoke_method("FireAlert", &args)

    }

}

