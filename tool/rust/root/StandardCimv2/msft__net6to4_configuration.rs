// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Net6to4Configuration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Net6to4Configuration {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "AutoSharing")]
    pub auto_sharing: Option<u32>,

/// 
    #[serde(rename = "PolicyStore")]
    pub policy_store: Option<String>,

/// 
    #[serde(rename = "RelayName")]
    pub relay_name: Option<String>,

/// 
    #[serde(rename = "RelayState")]
    pub relay_state: Option<u32>,

/// 
    #[serde(rename = "ResolutionInterval")]
    pub resolution_interval: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,
}

impl MSFT_Net6to4Configuration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            auto_sharing: None,
            policy_store: None,
            relay_name: None,
            relay_state: None,
            resolution_interval: None,
            state: None,
        }
    }


    /// Sets the value of AutoSharing
    pub fn set_auto_sharing(&mut self, value: u32) {
        self.auto_sharing = Some(value);
    }

    /// Gets the value of AutoSharing
    pub fn get_auto_sharing(&self) -> Option<&u32> {
        self.auto_sharing.as_ref()
    }

    /// Sets the value of PolicyStore
    pub fn set_policy_store(&mut self, value: String) {
        self.policy_store = Some(value);
    }

    /// Gets the value of PolicyStore
    pub fn get_policy_store(&self) -> Option<&String> {
        self.policy_store.as_ref()
    }

    /// Sets the value of RelayName
    pub fn set_relay_name(&mut self, value: String) {
        self.relay_name = Some(value);
    }

    /// Gets the value of RelayName
    pub fn get_relay_name(&self) -> Option<&String> {
        self.relay_name.as_ref()
    }

    /// Sets the value of RelayState
    pub fn set_relay_state(&mut self, value: u32) {
        self.relay_state = Some(value);
    }

    /// Gets the value of RelayState
    pub fn get_relay_state(&self) -> Option<&u32> {
        self.relay_state.as_ref()
    }

    /// Sets the value of ResolutionInterval
    pub fn set_resolution_interval(&mut self, value: u32) {
        self.resolution_interval = Some(value);
    }

    /// Gets the value of ResolutionInterval
    pub fn get_resolution_interval(&self) -> Option<&u32> {
        self.resolution_interval.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

/// 

    /// * `auto_sharing` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `relay_name` -  (bool)
    /// * `relay_state` -  (bool)
    /// * `resolution_interval` -  (bool)
    /// * `state` -  (bool)

    /// * `output_object` -  (MSFT_Net6to4Configuration)
    /// * `return_value` -  (u32)
    pub fn reset(&self, state: bool, auto_sharing: bool, relay_name: bool, relay_state: bool, resolution_interval: bool, pass_thru: bool, output_object: &mut MSFT_Net6to4Configuration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "State".to_string(), value: state.into() });
        args.push(MethodParameter { name: "AutoSharing".to_string(), value: auto_sharing.into() });
        args.push(MethodParameter { name: "RelayName".to_string(), value: relay_name.into() });
        args.push(MethodParameter { name: "RelayState".to_string(), value: relay_state.into() });
        args.push(MethodParameter { name: "ResolutionInterval".to_string(), value: resolution_interval.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Reset", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }

}

