// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SMHBA_PROTOCOLSTATISTICS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SMHBA_PROTOCOLSTATISTICS {

/// 
    #[serde(rename = "ControlRequests")]
    pub control_requests: Option<i64>,

/// 
    #[serde(rename = "InputMegabytes")]
    pub input_megabytes: Option<i64>,

/// 
    #[serde(rename = "InputRequests")]
    pub input_requests: Option<i64>,

/// 
    #[serde(rename = "OutputMegabytes")]
    pub output_megabytes: Option<i64>,

/// 
    #[serde(rename = "OutputRequests")]
    pub output_requests: Option<i64>,

/// 
    #[serde(rename = "SecondsSinceLastReset")]
    pub seconds_since_last_reset: Option<i64>,
}

impl MS_SMHBA_PROTOCOLSTATISTICS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            control_requests: None,
            input_megabytes: None,
            input_requests: None,
            output_megabytes: None,
            output_requests: None,
            seconds_since_last_reset: None,
        }
    }


    /// Sets the value of ControlRequests
    pub fn set_control_requests(&mut self, value: i64) {
        self.control_requests = Some(value);
    }

    /// Gets the value of ControlRequests
    pub fn get_control_requests(&self) -> Option<&i64> {
        self.control_requests.as_ref()
    }

    /// Sets the value of InputMegabytes
    pub fn set_input_megabytes(&mut self, value: i64) {
        self.input_megabytes = Some(value);
    }

    /// Gets the value of InputMegabytes
    pub fn get_input_megabytes(&self) -> Option<&i64> {
        self.input_megabytes.as_ref()
    }

    /// Sets the value of InputRequests
    pub fn set_input_requests(&mut self, value: i64) {
        self.input_requests = Some(value);
    }

    /// Gets the value of InputRequests
    pub fn get_input_requests(&self) -> Option<&i64> {
        self.input_requests.as_ref()
    }

    /// Sets the value of OutputMegabytes
    pub fn set_output_megabytes(&mut self, value: i64) {
        self.output_megabytes = Some(value);
    }

    /// Gets the value of OutputMegabytes
    pub fn get_output_megabytes(&self) -> Option<&i64> {
        self.output_megabytes.as_ref()
    }

    /// Sets the value of OutputRequests
    pub fn set_output_requests(&mut self, value: i64) {
        self.output_requests = Some(value);
    }

    /// Gets the value of OutputRequests
    pub fn get_output_requests(&self) -> Option<&i64> {
        self.output_requests.as_ref()
    }

    /// Sets the value of SecondsSinceLastReset
    pub fn set_seconds_since_last_reset(&mut self, value: i64) {
        self.seconds_since_last_reset = Some(value);
    }

    /// Gets the value of SecondsSinceLastReset
    pub fn get_seconds_since_last_reset(&self) -> Option<&i64> {
        self.seconds_since_last_reset.as_ref()
    }
}

