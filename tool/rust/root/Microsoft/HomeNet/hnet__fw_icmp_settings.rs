// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_FwIcmpSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_FwIcmpSettings {

/// 
    #[serde(rename = "AllowInboundEchoRequest")]
    pub allow_inbound_echo_request: Option<bool>,

/// 
    #[serde(rename = "AllowInboundMaskRequest")]
    pub allow_inbound_mask_request: Option<bool>,

/// 
    #[serde(rename = "AllowInboundRouterRequest")]
    pub allow_inbound_router_request: Option<bool>,

/// 
    #[serde(rename = "AllowInboundTimestampRequest")]
    pub allow_inbound_timestamp_request: Option<bool>,

/// 
    #[serde(rename = "AllowOutboundDestinationUnreachable")]
    pub allow_outbound_destination_unreachable: Option<bool>,

/// 
    #[serde(rename = "AllowOutboundParameterProblem")]
    pub allow_outbound_parameter_problem: Option<bool>,

/// 
    #[serde(rename = "AllowOutboundSourceQuench")]
    pub allow_outbound_source_quench: Option<bool>,

/// 
    #[serde(rename = "AllowOutboundTimeExceeded")]
    pub allow_outbound_time_exceeded: Option<bool>,

/// 
    #[serde(rename = "AllowRedirect")]
    pub allow_redirect: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl HNet_FwIcmpSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_inbound_echo_request: None,
            allow_inbound_mask_request: None,
            allow_inbound_router_request: None,
            allow_inbound_timestamp_request: None,
            allow_outbound_destination_unreachable: None,
            allow_outbound_parameter_problem: None,
            allow_outbound_source_quench: None,
            allow_outbound_time_exceeded: None,
            allow_redirect: None,
            name: None,
        }
    }


    /// Sets the value of AllowInboundEchoRequest
    pub fn set_allow_inbound_echo_request(&mut self, value: bool) {
        self.allow_inbound_echo_request = Some(value);
    }

    /// Gets the value of AllowInboundEchoRequest
    pub fn get_allow_inbound_echo_request(&self) -> Option<&bool> {
        self.allow_inbound_echo_request.as_ref()
    }

    /// Sets the value of AllowInboundMaskRequest
    pub fn set_allow_inbound_mask_request(&mut self, value: bool) {
        self.allow_inbound_mask_request = Some(value);
    }

    /// Gets the value of AllowInboundMaskRequest
    pub fn get_allow_inbound_mask_request(&self) -> Option<&bool> {
        self.allow_inbound_mask_request.as_ref()
    }

    /// Sets the value of AllowInboundRouterRequest
    pub fn set_allow_inbound_router_request(&mut self, value: bool) {
        self.allow_inbound_router_request = Some(value);
    }

    /// Gets the value of AllowInboundRouterRequest
    pub fn get_allow_inbound_router_request(&self) -> Option<&bool> {
        self.allow_inbound_router_request.as_ref()
    }

    /// Sets the value of AllowInboundTimestampRequest
    pub fn set_allow_inbound_timestamp_request(&mut self, value: bool) {
        self.allow_inbound_timestamp_request = Some(value);
    }

    /// Gets the value of AllowInboundTimestampRequest
    pub fn get_allow_inbound_timestamp_request(&self) -> Option<&bool> {
        self.allow_inbound_timestamp_request.as_ref()
    }

    /// Sets the value of AllowOutboundDestinationUnreachable
    pub fn set_allow_outbound_destination_unreachable(&mut self, value: bool) {
        self.allow_outbound_destination_unreachable = Some(value);
    }

    /// Gets the value of AllowOutboundDestinationUnreachable
    pub fn get_allow_outbound_destination_unreachable(&self) -> Option<&bool> {
        self.allow_outbound_destination_unreachable.as_ref()
    }

    /// Sets the value of AllowOutboundParameterProblem
    pub fn set_allow_outbound_parameter_problem(&mut self, value: bool) {
        self.allow_outbound_parameter_problem = Some(value);
    }

    /// Gets the value of AllowOutboundParameterProblem
    pub fn get_allow_outbound_parameter_problem(&self) -> Option<&bool> {
        self.allow_outbound_parameter_problem.as_ref()
    }

    /// Sets the value of AllowOutboundSourceQuench
    pub fn set_allow_outbound_source_quench(&mut self, value: bool) {
        self.allow_outbound_source_quench = Some(value);
    }

    /// Gets the value of AllowOutboundSourceQuench
    pub fn get_allow_outbound_source_quench(&self) -> Option<&bool> {
        self.allow_outbound_source_quench.as_ref()
    }

    /// Sets the value of AllowOutboundTimeExceeded
    pub fn set_allow_outbound_time_exceeded(&mut self, value: bool) {
        self.allow_outbound_time_exceeded = Some(value);
    }

    /// Gets the value of AllowOutboundTimeExceeded
    pub fn get_allow_outbound_time_exceeded(&self) -> Option<&bool> {
        self.allow_outbound_time_exceeded.as_ref()
    }

    /// Sets the value of AllowRedirect
    pub fn set_allow_redirect(&mut self, value: bool) {
        self.allow_redirect = Some(value);
    }

    /// Gets the value of AllowRedirect
    pub fn get_allow_redirect(&self) -> Option<&bool> {
        self.allow_redirect.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

