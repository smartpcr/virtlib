// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_FC4STATISTICS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_FC4STATISTICS {

/// 
    #[serde(rename = "ControlRequests")]
    pub control_requests: Option<u64>,

/// 
    #[serde(rename = "InputMegabytes")]
    pub input_megabytes: Option<u64>,

/// 
    #[serde(rename = "InputRequests")]
    pub input_requests: Option<u64>,

/// 
    #[serde(rename = "OutputMegabytes")]
    pub output_megabytes: Option<u64>,

/// 
    #[serde(rename = "OutputRequests")]
    pub output_requests: Option<u64>,
}

impl MSFC_FC4STATISTICS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            control_requests: None,
            input_megabytes: None,
            input_requests: None,
            output_megabytes: None,
            output_requests: None,
        }
    }


    /// Sets the value of ControlRequests
    pub fn set_control_requests(&mut self, value: u64) {
        self.control_requests = Some(value);
    }

    /// Gets the value of ControlRequests
    pub fn get_control_requests(&self) -> Option<&u64> {
        self.control_requests.as_ref()
    }

    /// Sets the value of InputMegabytes
    pub fn set_input_megabytes(&mut self, value: u64) {
        self.input_megabytes = Some(value);
    }

    /// Gets the value of InputMegabytes
    pub fn get_input_megabytes(&self) -> Option<&u64> {
        self.input_megabytes.as_ref()
    }

    /// Sets the value of InputRequests
    pub fn set_input_requests(&mut self, value: u64) {
        self.input_requests = Some(value);
    }

    /// Gets the value of InputRequests
    pub fn get_input_requests(&self) -> Option<&u64> {
        self.input_requests.as_ref()
    }

    /// Sets the value of OutputMegabytes
    pub fn set_output_megabytes(&mut self, value: u64) {
        self.output_megabytes = Some(value);
    }

    /// Gets the value of OutputMegabytes
    pub fn get_output_megabytes(&self) -> Option<&u64> {
        self.output_megabytes.as_ref()
    }

    /// Sets the value of OutputRequests
    pub fn set_output_requests(&mut self, value: u64) {
        self.output_requests = Some(value);
    }

    /// Gets the value of OutputRequests
    pub fn get_output_requests(&self) -> Option<&u64> {
        self.output_requests.as_ref()
    }
}

