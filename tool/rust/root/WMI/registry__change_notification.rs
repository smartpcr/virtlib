// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Registry_ChangeNotification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Registry_ChangeNotification {
    #[serde(flatten)]
    pub base: Registry,

/// 
    #[serde(rename = "KeyHandle")]
    pub key_handle: Option<u32>,

/// 
    #[serde(rename = "Notification")]
    pub notification: Option<u32>,

/// 
    #[serde(rename = "Primary")]
    pub primary: Option<u8>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u8>,

/// 
    #[serde(rename = "WatchSubtree")]
    pub watch_subtree: Option<u8>,
}

impl Registry_ChangeNotification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Registry::new(),
            key_handle: None,
            notification: None,
            primary: None,
            type: None,
            watch_subtree: None,
        }
    }


    /// Sets the value of KeyHandle
    pub fn set_key_handle(&mut self, value: u32) {
        self.key_handle = Some(value);
    }

    /// Gets the value of KeyHandle
    pub fn get_key_handle(&self) -> Option<&u32> {
        self.key_handle.as_ref()
    }

    /// Sets the value of Notification
    pub fn set_notification(&mut self, value: u32) {
        self.notification = Some(value);
    }

    /// Gets the value of Notification
    pub fn get_notification(&self) -> Option<&u32> {
        self.notification.as_ref()
    }

    /// Sets the value of Primary
    pub fn set_primary(&mut self, value: u8) {
        self.primary = Some(value);
    }

    /// Gets the value of Primary
    pub fn get_primary(&self) -> Option<&u8> {
        self.primary.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u8) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u8> {
        self.type.as_ref()
    }

    /// Sets the value of WatchSubtree
    pub fn set_watch_subtree(&mut self, value: u8) {
        self.watch_subtree = Some(value);
    }

    /// Gets the value of WatchSubtree
    pub fn get_watch_subtree(&self) -> Option<&u8> {
        self.watch_subtree.as_ref()
    }
}

