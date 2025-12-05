// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetEventPacketCaptureTarget struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetEventPacketCaptureTarget {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CaptureStatus")]
    pub capture_status: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,
}

impl MSFT_NetEventPacketCaptureTarget {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            capture_status: None,
            id: None,
            provider_name: None,
        }
    }


    /// Sets the value of CaptureStatus
    pub fn set_capture_status(&mut self, value: u32) {
        self.capture_status = Some(value);
    }

    /// Gets the value of CaptureStatus
    pub fn get_capture_status(&self) -> Option<&u32> {
        self.capture_status.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of ProviderName
    pub fn set_provider_name(&mut self, value: String) {
        self.provider_name = Some(value);
    }

    /// Gets the value of ProviderName
    pub fn get_provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }
}

