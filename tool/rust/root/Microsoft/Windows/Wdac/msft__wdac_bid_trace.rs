// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Wdac
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WdacBidTrace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WdacBidTrace {

/// 
    #[serde(rename = "BidTraceAdapter")]
    pub bid_trace_adapter: Option<String>,

/// 
    #[serde(rename = "IsDefined")]
    pub is_defined: Option<bool>,

/// 
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,

/// 
    #[serde(rename = "Mode")]
    pub mode: Option<u32>,

/// 
    #[serde(rename = "PathOrFolder")]
    pub path_or_folder: Option<String>,

/// 
    #[serde(rename = "Platform")]
    pub platform: Option<String>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,
}

impl MSFT_WdacBidTrace {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bid_trace_adapter: None,
            is_defined: None,
            is_enabled: None,
            mode: None,
            path_or_folder: None,
            platform: None,
            process_id: None,
        }
    }


    /// Sets the value of BidTraceAdapter
    pub fn set_bid_trace_adapter(&mut self, value: String) {
        self.bid_trace_adapter = Some(value);
    }

    /// Gets the value of BidTraceAdapter
    pub fn get_bid_trace_adapter(&self) -> Option<&String> {
        self.bid_trace_adapter.as_ref()
    }

    /// Sets the value of IsDefined
    pub fn set_is_defined(&mut self, value: bool) {
        self.is_defined = Some(value);
    }

    /// Gets the value of IsDefined
    pub fn get_is_defined(&self) -> Option<&bool> {
        self.is_defined.as_ref()
    }

    /// Sets the value of IsEnabled
    pub fn set_is_enabled(&mut self, value: bool) {
        self.is_enabled = Some(value);
    }

    /// Gets the value of IsEnabled
    pub fn get_is_enabled(&self) -> Option<&bool> {
        self.is_enabled.as_ref()
    }

    /// Sets the value of Mode
    pub fn set_mode(&mut self, value: u32) {
        self.mode = Some(value);
    }

    /// Gets the value of Mode
    pub fn get_mode(&self) -> Option<&u32> {
        self.mode.as_ref()
    }

    /// Sets the value of PathOrFolder
    pub fn set_path_or_folder(&mut self, value: String) {
        self.path_or_folder = Some(value);
    }

    /// Gets the value of PathOrFolder
    pub fn get_path_or_folder(&self) -> Option<&String> {
        self.path_or_folder.as_ref()
    }

    /// Sets the value of Platform
    pub fn set_platform(&mut self, value: String) {
        self.platform = Some(value);
    }

    /// Gets the value of Platform
    pub fn get_platform(&self) -> Option<&String> {
        self.platform.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }
}

