// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PeerSecuritySettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerSecuritySettings {

/// Whether message-level and transport-level security are used by an endpoint configured with the binding.
    #[serde(rename = "Mode")]
    pub mode: Option<String>,

/// Transport security settings.
    #[serde(rename = "Transport")]
    pub transport: Option<PeerTransportSecuritySettings>,
}

impl PeerSecuritySettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            mode: None,
            transport: None,
        }
    }


    /// Sets the value of Mode
    pub fn set_mode(&mut self, value: String) {
        self.mode = Some(value);
    }

    /// Gets the value of Mode
    pub fn get_mode(&self) -> Option<&String> {
        self.mode.as_ref()
    }

    /// Sets the value of Transport
    pub fn set_transport(&mut self, value: PeerTransportSecuritySettings) {
        self.transport = Some(value);
    }

    /// Gets the value of Transport
    pub fn get_transport(&self) -> Option<&PeerTransportSecuritySettings> {
        self.transport.as_ref()
    }
}

