// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetDnsTransitionConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetDnsTransitionConfiguration {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "AcceptInterface")]
    pub accept_interface: Vec<String>,

/// 
    #[serde(rename = "AlwaysSynthesize")]
    pub always_synthesize: Option<bool>,

/// 
    #[serde(rename = "ExclusionList")]
    pub exclusion_list: Vec<String>,

/// 
    #[serde(rename = "Latency")]
    pub latency: Option<u32>,

/// 
    #[serde(rename = "OnlySendAQuery")]
    pub only_send_aquery: Option<bool>,

/// 
    #[serde(rename = "PrefixMapping")]
    pub prefix_mapping: Vec<String>,

/// 
    #[serde(rename = "SendInterface")]
    pub send_interface: Vec<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,
}

impl MSFT_NetDnsTransitionConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            accept_interface: Vec::new(),
            always_synthesize: None,
            exclusion_list: Vec::new(),
            latency: None,
            only_send_aquery: None,
            prefix_mapping: Vec::new(),
            send_interface: Vec::new(),
            state: None,
        }
    }


    /// Sets the value of AcceptInterface
    pub fn set_accept_interface(&mut self, value: Vec<String>) {
        self.accept_interface = value;
    }

    /// Gets the value of AcceptInterface
    pub fn get_accept_interface(&self) -> &Vec<String> {
        &self.accept_interface
    }

    /// Sets the value of AlwaysSynthesize
    pub fn set_always_synthesize(&mut self, value: bool) {
        self.always_synthesize = Some(value);
    }

    /// Gets the value of AlwaysSynthesize
    pub fn get_always_synthesize(&self) -> Option<&bool> {
        self.always_synthesize.as_ref()
    }

    /// Sets the value of ExclusionList
    pub fn set_exclusion_list(&mut self, value: Vec<String>) {
        self.exclusion_list = value;
    }

    /// Gets the value of ExclusionList
    pub fn get_exclusion_list(&self) -> &Vec<String> {
        &self.exclusion_list
    }

    /// Sets the value of Latency
    pub fn set_latency(&mut self, value: u32) {
        self.latency = Some(value);
    }

    /// Gets the value of Latency
    pub fn get_latency(&self) -> Option<&u32> {
        self.latency.as_ref()
    }

    /// Sets the value of OnlySendAQuery
    pub fn set_only_send_aquery(&mut self, value: bool) {
        self.only_send_aquery = Some(value);
    }

    /// Gets the value of OnlySendAQuery
    pub fn get_only_send_aquery(&self) -> Option<&bool> {
        self.only_send_aquery.as_ref()
    }

    /// Sets the value of PrefixMapping
    pub fn set_prefix_mapping(&mut self, value: Vec<String>) {
        self.prefix_mapping = value;
    }

    /// Gets the value of PrefixMapping
    pub fn get_prefix_mapping(&self) -> &Vec<String> {
        &self.prefix_mapping
    }

    /// Sets the value of SendInterface
    pub fn set_send_interface(&mut self, value: Vec<String>) {
        self.send_interface = value;
    }

    /// Gets the value of SendInterface
    pub fn get_send_interface(&self) -> &Vec<String> {
        &self.send_interface
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

    /// * `output_object` -  (MSFT_NetDnsTransitionConfiguration)
    /// * `return_value` -  (u32)
    pub fn enable(&self, pass_thru: bool, output_object: &mut MSFT_NetDnsTransitionConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Enable", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }


/// 

    /// * `pass_thru` -  (bool)

    /// * `output_object` -  (MSFT_NetDnsTransitionConfiguration)
    /// * `return_value` -  (u32)
    pub fn disable(&self, pass_thru: bool, output_object: &mut MSFT_NetDnsTransitionConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Disable", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }


/// 

    /// * `accept_interface` -  (bool)
    /// * `always_synthesize` -  (bool)
    /// * `exclusion_list` -  (bool)
    /// * `latency` -  (bool)
    /// * `only_send_aquery` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `prefix_mapping` -  (bool)
    /// * `send_interface` -  (bool)
    /// * `state` -  (bool)

    /// * `output_object` -  (MSFT_NetDnsTransitionConfiguration)
    /// * `return_value` -  (u32)
    pub fn reset(&self, state: bool, only_send_aquery: bool, latency: bool, always_synthesize: bool, prefix_mapping: bool, exclusion_list: bool, send_interface: bool, accept_interface: bool, pass_thru: bool, output_object: &mut MSFT_NetDnsTransitionConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "State".to_string(), value: state.into() });
        args.push(MethodParameter { name: "OnlySendAQuery".to_string(), value: only_send_aquery.into() });
        args.push(MethodParameter { name: "Latency".to_string(), value: latency.into() });
        args.push(MethodParameter { name: "AlwaysSynthesize".to_string(), value: always_synthesize.into() });
        args.push(MethodParameter { name: "PrefixMapping".to_string(), value: prefix_mapping.into() });
        args.push(MethodParameter { name: "ExclusionList".to_string(), value: exclusion_list.into() });
        args.push(MethodParameter { name: "SendInterface".to_string(), value: send_interface.into() });
        args.push(MethodParameter { name: "AcceptInterface".to_string(), value: accept_interface.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Reset", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }

}

