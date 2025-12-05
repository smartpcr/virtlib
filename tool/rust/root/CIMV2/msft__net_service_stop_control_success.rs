// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceStopControlSuccess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceStopControlSuccess {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

/// 
    #[serde(rename = "Control")]
    pub control: Option<String>,

/// 
    #[serde(rename = "Reason")]
    pub reason: Option<String>,

/// 
    #[serde(rename = "ReasonText")]
    pub reason_text: Option<String>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,

/// 
    #[serde(rename = "sid")]
    pub sid: Option<String>,
}

impl MSFT_NetServiceStopControlSuccess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            comment: None,
            control: None,
            reason: None,
            reason_text: None,
            service: None,
            sid: None,
        }
    }


    /// Sets the value of Comment
    pub fn set_comment(&mut self, value: String) {
        self.comment = Some(value);
    }

    /// Gets the value of Comment
    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    /// Sets the value of Control
    pub fn set_control(&mut self, value: String) {
        self.control = Some(value);
    }

    /// Gets the value of Control
    pub fn get_control(&self) -> Option<&String> {
        self.control.as_ref()
    }

    /// Sets the value of Reason
    pub fn set_reason(&mut self, value: String) {
        self.reason = Some(value);
    }

    /// Gets the value of Reason
    pub fn get_reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }

    /// Sets the value of ReasonText
    pub fn set_reason_text(&mut self, value: String) {
        self.reason_text = Some(value);
    }

    /// Gets the value of ReasonText
    pub fn get_reason_text(&self) -> Option<&String> {
        self.reason_text.as_ref()
    }

    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }

    /// Sets the value of sid
    pub fn set_sid(&mut self, value: String) {
        self.sid = Some(value);
    }

    /// Gets the value of sid
    pub fn get_sid(&self) -> Option<&String> {
        self.sid.as_ref()
    }
}

