// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetISATAPConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetISATAPConfiguration {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "PolicyStore")]
    pub policy_store: Option<String>,

/// 
    #[serde(rename = "ResolutionInterval")]
    pub resolution_interval: Option<u32>,

/// 
    #[serde(rename = "ResolutionState")]
    pub resolution_state: Option<u32>,

/// 
    #[serde(rename = "Router")]
    pub router: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,
}

impl MSFT_NetISATAPConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            policy_store: None,
            resolution_interval: None,
            resolution_state: None,
            router: None,
            state: None,
        }
    }


    /// Sets the value of PolicyStore
    pub fn set_policy_store(&mut self, value: String) {
        self.policy_store = Some(value);
    }

    /// Gets the value of PolicyStore
    pub fn get_policy_store(&self) -> Option<&String> {
        self.policy_store.as_ref()
    }

    /// Sets the value of ResolutionInterval
    pub fn set_resolution_interval(&mut self, value: u32) {
        self.resolution_interval = Some(value);
    }

    /// Gets the value of ResolutionInterval
    pub fn get_resolution_interval(&self) -> Option<&u32> {
        self.resolution_interval.as_ref()
    }

    /// Sets the value of ResolutionState
    pub fn set_resolution_state(&mut self, value: u32) {
        self.resolution_state = Some(value);
    }

    /// Gets the value of ResolutionState
    pub fn get_resolution_state(&self) -> Option<&u32> {
        self.resolution_state.as_ref()
    }

    /// Sets the value of Router
    pub fn set_router(&mut self, value: String) {
        self.router = Some(value);
    }

    /// Gets the value of Router
    pub fn get_router(&self) -> Option<&String> {
        self.router.as_ref()
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

    /// * `pass_thru` -  (bool)
    /// * `resolution_interval` -  (bool)
    /// * `resolution_state` -  (bool)
    /// * `router` -  (bool)
    /// * `state` -  (bool)

    /// * `output_object` -  (MSFT_NetISATAPConfiguration)
    /// * `return_value` -  (u32)
    pub fn reset(&self, state: bool, router: bool, resolution_state: bool, resolution_interval: bool, pass_thru: bool, output_object: &mut MSFT_NetISATAPConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "State".to_string(), value: state.into() });
        args.push(MethodParameter { name: "Router".to_string(), value: router.into() });
        args.push(MethodParameter { name: "ResolutionState".to_string(), value: resolution_state.into() });
        args.push(MethodParameter { name: "ResolutionInterval".to_string(), value: resolution_interval.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Reset", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }

}

