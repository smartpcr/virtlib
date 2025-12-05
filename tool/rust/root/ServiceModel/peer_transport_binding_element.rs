// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PeerTransportBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerTransportBindingElement {
    #[serde(flatten)]
    pub base: TransportBindingElement,

/// The IP address on which the peer node will listen for messages.
    #[serde(rename = "ListenIPAddress")]
    pub listen_ipaddress: Option<String>,

/// The network interface port on which this binding will process peer channel messages.
    #[serde(rename = "Port")]
    pub port: Option<i32>,

/// Peer transport security settings.
    #[serde(rename = "Security")]
    pub security: Option<PeerSecuritySettings>,
}

impl PeerTransportBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: TransportBindingElement::new(),
            listen_ipaddress: None,
            port: None,
            security: None,
        }
    }


    /// Sets the value of ListenIPAddress
    pub fn set_listen_ipaddress(&mut self, value: String) {
        self.listen_ipaddress = Some(value);
    }

    /// Gets the value of ListenIPAddress
    pub fn get_listen_ipaddress(&self) -> Option<&String> {
        self.listen_ipaddress.as_ref()
    }

    /// Sets the value of Port
    pub fn set_port(&mut self, value: i32) {
        self.port = Some(value);
    }

    /// Gets the value of Port
    pub fn get_port(&self) -> Option<&i32> {
        self.port.as_ref()
    }

    /// Sets the value of Security
    pub fn set_security(&mut self, value: PeerSecuritySettings) {
        self.security = Some(value);
    }

    /// Gets the value of Security
    pub fn get_security(&self) -> Option<&PeerSecuritySettings> {
        self.security.as_ref()
    }
}

