// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Channel struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Channel {

/// The local endpoint for the channel.
    #[serde(rename = "LocalAddress")]
    pub local_address: Option<String>,

/// The remote address associated with the channel.
    #[serde(rename = "RemoteAddress")]
    pub remote_address: Option<String>,

/// A reference to the endpoint the channel connects to.
    #[serde(rename = "RemoteEndpoint")]
    pub remote_endpoint: Option<Endpoint>,

/// The current session Id, if any.
    #[serde(rename = "SessionId")]
    pub session_id: Option<String>,

/// The type of the channel.
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl Channel {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            local_address: None,
            remote_address: None,
            remote_endpoint: None,
            session_id: None,
            type: None,
        }
    }


    /// Sets the value of LocalAddress
    pub fn set_local_address(&mut self, value: String) {
        self.local_address = Some(value);
    }

    /// Gets the value of LocalAddress
    pub fn get_local_address(&self) -> Option<&String> {
        self.local_address.as_ref()
    }

    /// Sets the value of RemoteAddress
    pub fn set_remote_address(&mut self, value: String) {
        self.remote_address = Some(value);
    }

    /// Gets the value of RemoteAddress
    pub fn get_remote_address(&self) -> Option<&String> {
        self.remote_address.as_ref()
    }

    /// Sets the value of RemoteEndpoint
    pub fn set_remote_endpoint(&mut self, value: Endpoint) {
        self.remote_endpoint = Some(value);
    }

    /// Gets the value of RemoteEndpoint
    pub fn get_remote_endpoint(&self) -> Option<&Endpoint> {
        self.remote_endpoint.as_ref()
    }

    /// Sets the value of SessionId
    pub fn set_session_id(&mut self, value: String) {
        self.session_id = Some(value);
    }

    /// Gets the value of SessionId
    pub fn get_session_id(&self) -> Option<&String> {
        self.session_id.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }
}

