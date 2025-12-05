// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Header_Extension_V1_TypeGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Header_Extension_V1_TypeGroup {
    #[serde(flatten)]
    pub base: EventTraceEvent_V1,

/// 
    #[serde(rename = "GroupMask1")]
    pub group_mask1: Option<u32>,

/// 
    #[serde(rename = "GroupMask2")]
    pub group_mask2: Option<u32>,

/// 
    #[serde(rename = "GroupMask3")]
    pub group_mask3: Option<u32>,

/// 
    #[serde(rename = "GroupMask4")]
    pub group_mask4: Option<u32>,

/// 
    #[serde(rename = "GroupMask5")]
    pub group_mask5: Option<u32>,

/// 
    #[serde(rename = "GroupMask6")]
    pub group_mask6: Option<u32>,

/// 
    #[serde(rename = "GroupMask7")]
    pub group_mask7: Option<u32>,

/// 
    #[serde(rename = "GroupMask8")]
    pub group_mask8: Option<u32>,
}

impl Header_Extension_V1_TypeGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTraceEvent_V1::new(),
            group_mask1: None,
            group_mask2: None,
            group_mask3: None,
            group_mask4: None,
            group_mask5: None,
            group_mask6: None,
            group_mask7: None,
            group_mask8: None,
        }
    }


    /// Sets the value of GroupMask1
    pub fn set_group_mask1(&mut self, value: u32) {
        self.group_mask1 = Some(value);
    }

    /// Gets the value of GroupMask1
    pub fn get_group_mask1(&self) -> Option<&u32> {
        self.group_mask1.as_ref()
    }

    /// Sets the value of GroupMask2
    pub fn set_group_mask2(&mut self, value: u32) {
        self.group_mask2 = Some(value);
    }

    /// Gets the value of GroupMask2
    pub fn get_group_mask2(&self) -> Option<&u32> {
        self.group_mask2.as_ref()
    }

    /// Sets the value of GroupMask3
    pub fn set_group_mask3(&mut self, value: u32) {
        self.group_mask3 = Some(value);
    }

    /// Gets the value of GroupMask3
    pub fn get_group_mask3(&self) -> Option<&u32> {
        self.group_mask3.as_ref()
    }

    /// Sets the value of GroupMask4
    pub fn set_group_mask4(&mut self, value: u32) {
        self.group_mask4 = Some(value);
    }

    /// Gets the value of GroupMask4
    pub fn get_group_mask4(&self) -> Option<&u32> {
        self.group_mask4.as_ref()
    }

    /// Sets the value of GroupMask5
    pub fn set_group_mask5(&mut self, value: u32) {
        self.group_mask5 = Some(value);
    }

    /// Gets the value of GroupMask5
    pub fn get_group_mask5(&self) -> Option<&u32> {
        self.group_mask5.as_ref()
    }

    /// Sets the value of GroupMask6
    pub fn set_group_mask6(&mut self, value: u32) {
        self.group_mask6 = Some(value);
    }

    /// Gets the value of GroupMask6
    pub fn get_group_mask6(&self) -> Option<&u32> {
        self.group_mask6.as_ref()
    }

    /// Sets the value of GroupMask7
    pub fn set_group_mask7(&mut self, value: u32) {
        self.group_mask7 = Some(value);
    }

    /// Gets the value of GroupMask7
    pub fn get_group_mask7(&self) -> Option<&u32> {
        self.group_mask7.as_ref()
    }

    /// Sets the value of GroupMask8
    pub fn set_group_mask8(&mut self, value: u32) {
        self.group_mask8 = Some(value);
    }

    /// Gets the value of GroupMask8
    pub fn get_group_mask8(&self) -> Option<&u32> {
        self.group_mask8.as_ref()
    }
}

