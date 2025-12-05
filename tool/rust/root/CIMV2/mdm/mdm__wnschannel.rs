// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WNSChannel struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WNSChannel {

/// 
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,

/// 
    #[serde(rename = "Channel")]
    pub channel: Option<String>,

/// 
    #[serde(rename = "ChannelStatus")]
    pub channel_status: Option<u32>,

/// 
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: Option<String>,

/// 
    #[serde(rename = "LastError")]
    pub last_error: Option<u32>,

/// 
    #[serde(rename = "UserSID")]
    pub user_sid: Option<String>,
}

impl MDM_WNSChannel {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_id: None,
            channel: None,
            channel_status: None,
            expiration_time: None,
            last_error: None,
            user_sid: None,
        }
    }


    /// Sets the value of AppId
    pub fn set_app_id(&mut self, value: String) {
        self.app_id = Some(value);
    }

    /// Gets the value of AppId
    pub fn get_app_id(&self) -> Option<&String> {
        self.app_id.as_ref()
    }

    /// Sets the value of Channel
    pub fn set_channel(&mut self, value: String) {
        self.channel = Some(value);
    }

    /// Gets the value of Channel
    pub fn get_channel(&self) -> Option<&String> {
        self.channel.as_ref()
    }

    /// Sets the value of ChannelStatus
    pub fn set_channel_status(&mut self, value: u32) {
        self.channel_status = Some(value);
    }

    /// Gets the value of ChannelStatus
    pub fn get_channel_status(&self) -> Option<&u32> {
        self.channel_status.as_ref()
    }

    /// Sets the value of ExpirationTime
    pub fn set_expiration_time(&mut self, value: String) {
        self.expiration_time = Some(value);
    }

    /// Gets the value of ExpirationTime
    pub fn get_expiration_time(&self) -> Option<&String> {
        self.expiration_time.as_ref()
    }

    /// Sets the value of LastError
    pub fn set_last_error(&mut self, value: u32) {
        self.last_error = Some(value);
    }

    /// Gets the value of LastError
    pub fn get_last_error(&self) -> Option<&u32> {
        self.last_error.as_ref()
    }

    /// Sets the value of UserSID
    pub fn set_user_sid(&mut self, value: String) {
        self.user_sid = Some(value);
    }

    /// Gets the value of UserSID
    pub fn get_user_sid(&self) -> Option<&String> {
        self.user_sid.as_ref()
    }
}

