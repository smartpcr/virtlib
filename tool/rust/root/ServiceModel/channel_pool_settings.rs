// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ChannelPoolSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChannelPoolSettings {

/// The maximum time the connection can be idle before being disconnected. 
    #[serde(rename = "IdleTimeout")]
    pub idle_timeout: Option<String>,

/// The maximum time for a lease operation to complete before timing out.
    #[serde(rename = "LeaseTimeout")]
    pub lease_timeout: Option<String>,

/// The maximum number of outbound channels per endpoint.
    #[serde(rename = "MaxOutboundChannelsPerEndpoint")]
    pub max_outbound_channels_per_endpoint: Option<i32>,
}

impl ChannelPoolSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            idle_timeout: None,
            lease_timeout: None,
            max_outbound_channels_per_endpoint: None,
        }
    }


    /// Sets the value of IdleTimeout
    pub fn set_idle_timeout(&mut self, value: String) {
        self.idle_timeout = Some(value);
    }

    /// Gets the value of IdleTimeout
    pub fn get_idle_timeout(&self) -> Option<&String> {
        self.idle_timeout.as_ref()
    }

    /// Sets the value of LeaseTimeout
    pub fn set_lease_timeout(&mut self, value: String) {
        self.lease_timeout = Some(value);
    }

    /// Gets the value of LeaseTimeout
    pub fn get_lease_timeout(&self) -> Option<&String> {
        self.lease_timeout.as_ref()
    }

    /// Sets the value of MaxOutboundChannelsPerEndpoint
    pub fn set_max_outbound_channels_per_endpoint(&mut self, value: i32) {
        self.max_outbound_channels_per_endpoint = Some(value);
    }

    /// Gets the value of MaxOutboundChannelsPerEndpoint
    pub fn get_max_outbound_channels_per_endpoint(&self) -> Option<&i32> {
        self.max_outbound_channels_per_endpoint.as_ref()
    }
}

