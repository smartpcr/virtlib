// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// NlServerAuth_End struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NlServerAuth_End {
    #[serde(flatten)]
    pub base: NlServerAuth,

/// Account
    #[serde(rename = "Account")]
    pub account: Option<String>,

/// Channel Type
    #[serde(rename = "ChannelType")]
    pub channel_type: Option<u32>,

/// Client
    #[serde(rename = "Client")]
    pub client: Option<String>,

/// Negotiated Flags
    #[serde(rename = "NegotiatedFlags")]
    pub negotiated_flags: Option<u32>,

/// Status
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl NlServerAuth_End {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: NlServerAuth::new(),
            account: None,
            channel_type: None,
            client: None,
            negotiated_flags: None,
            status: None,
        }
    }


    /// Sets the value of Account
    pub fn set_account(&mut self, value: String) {
        self.account = Some(value);
    }

    /// Gets the value of Account
    pub fn get_account(&self) -> Option<&String> {
        self.account.as_ref()
    }

    /// Sets the value of ChannelType
    pub fn set_channel_type(&mut self, value: u32) {
        self.channel_type = Some(value);
    }

    /// Gets the value of ChannelType
    pub fn get_channel_type(&self) -> Option<&u32> {
        self.channel_type.as_ref()
    }

    /// Sets the value of Client
    pub fn set_client(&mut self, value: String) {
        self.client = Some(value);
    }

    /// Gets the value of Client
    pub fn get_client(&self) -> Option<&String> {
        self.client.as_ref()
    }

    /// Sets the value of NegotiatedFlags
    pub fn set_negotiated_flags(&mut self, value: u32) {
        self.negotiated_flags = Some(value);
    }

    /// Gets the value of NegotiatedFlags
    pub fn get_negotiated_flags(&self) -> Option<&u32> {
        self.negotiated_flags.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }
}

