// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_RemoteApplication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_RemoteApplication {

/// 
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,

/// 
    #[serde(rename = "FeedUrl")]
    pub feed_url: Option<String>,
}

impl MDM_RemoteApplication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_id: None,
            feed_url: None,
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

    /// Sets the value of FeedUrl
    pub fn set_feed_url(&mut self, value: String) {
        self.feed_url = Some(value);
    }

    /// Gets the value of FeedUrl
    pub fn get_feed_url(&self) -> Option<&String> {
        self.feed_url.as_ref()
    }
}

