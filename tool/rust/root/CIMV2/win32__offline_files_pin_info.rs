// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesPinInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesPinInfo {

/// 
    #[serde(rename = "Pinned")]
    pub pinned: Option<bool>,

/// 
    #[serde(rename = "PinnedForComputer")]
    pub pinned_for_computer: Option<u32>,

/// 
    #[serde(rename = "PinnedForFolderRedirection")]
    pub pinned_for_folder_redirection: Option<u32>,

/// 
    #[serde(rename = "PinnedForUser")]
    pub pinned_for_user: Option<u32>,

/// 
    #[serde(rename = "PinnedForUserByPolicy")]
    pub pinned_for_user_by_policy: Option<u32>,
}

impl Win32_OfflineFilesPinInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            pinned: None,
            pinned_for_computer: None,
            pinned_for_folder_redirection: None,
            pinned_for_user: None,
            pinned_for_user_by_policy: None,
        }
    }


    /// Sets the value of Pinned
    pub fn set_pinned(&mut self, value: bool) {
        self.pinned = Some(value);
    }

    /// Gets the value of Pinned
    pub fn get_pinned(&self) -> Option<&bool> {
        self.pinned.as_ref()
    }

    /// Sets the value of PinnedForComputer
    pub fn set_pinned_for_computer(&mut self, value: u32) {
        self.pinned_for_computer = Some(value);
    }

    /// Gets the value of PinnedForComputer
    pub fn get_pinned_for_computer(&self) -> Option<&u32> {
        self.pinned_for_computer.as_ref()
    }

    /// Sets the value of PinnedForFolderRedirection
    pub fn set_pinned_for_folder_redirection(&mut self, value: u32) {
        self.pinned_for_folder_redirection = Some(value);
    }

    /// Gets the value of PinnedForFolderRedirection
    pub fn get_pinned_for_folder_redirection(&self) -> Option<&u32> {
        self.pinned_for_folder_redirection.as_ref()
    }

    /// Sets the value of PinnedForUser
    pub fn set_pinned_for_user(&mut self, value: u32) {
        self.pinned_for_user = Some(value);
    }

    /// Gets the value of PinnedForUser
    pub fn get_pinned_for_user(&self) -> Option<&u32> {
        self.pinned_for_user.as_ref()
    }

    /// Sets the value of PinnedForUserByPolicy
    pub fn set_pinned_for_user_by_policy(&mut self, value: u32) {
        self.pinned_for_user_by_policy = Some(value);
    }

    /// Gets the value of PinnedForUserByPolicy
    pub fn get_pinned_for_user_by_policy(&self) -> Option<&u32> {
        self.pinned_for_user_by_policy.as_ref()
    }
}

