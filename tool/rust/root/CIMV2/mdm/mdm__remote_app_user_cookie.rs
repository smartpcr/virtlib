// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_RemoteAppUserCookie struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_RemoteAppUserCookie {

/// 
    #[serde(rename = "Cookie")]
    pub cookie: Option<String>,

/// 
    #[serde(rename = "CookieHash")]
    pub cookie_hash: Option<String>,

/// 
    #[serde(rename = "FeedUrl")]
    pub feed_url: Option<String>,
}

impl MDM_RemoteAppUserCookie {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cookie: None,
            cookie_hash: None,
            feed_url: None,
        }
    }


    /// Sets the value of Cookie
    pub fn set_cookie(&mut self, value: String) {
        self.cookie = Some(value);
    }

    /// Gets the value of Cookie
    pub fn get_cookie(&self) -> Option<&String> {
        self.cookie.as_ref()
    }

    /// Sets the value of CookieHash
    pub fn set_cookie_hash(&mut self, value: String) {
        self.cookie_hash = Some(value);
    }

    /// Gets the value of CookieHash
    pub fn get_cookie_hash(&self) -> Option<&String> {
        self.cookie_hash.as_ref()
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

