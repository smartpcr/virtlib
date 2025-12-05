// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SMHBA_FC_PHY struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SMHBA_FC_PHY {

/// 
    #[serde(rename = "MaxFrameSize")]
    pub max_frame_size: Option<u32>,

/// 
    #[serde(rename = "PhySpeed")]
    pub phy_speed: Option<u32>,

/// 
    #[serde(rename = "PhySupportSpeed")]
    pub phy_support_speed: Option<u32>,

/// 
    #[serde(rename = "PhyType")]
    pub phy_type: Option<u8>,
}

impl MS_SMHBA_FC_PHY {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            max_frame_size: None,
            phy_speed: None,
            phy_support_speed: None,
            phy_type: None,
        }
    }


    /// Sets the value of MaxFrameSize
    pub fn set_max_frame_size(&mut self, value: u32) {
        self.max_frame_size = Some(value);
    }

    /// Gets the value of MaxFrameSize
    pub fn get_max_frame_size(&self) -> Option<&u32> {
        self.max_frame_size.as_ref()
    }

    /// Sets the value of PhySpeed
    pub fn set_phy_speed(&mut self, value: u32) {
        self.phy_speed = Some(value);
    }

    /// Gets the value of PhySpeed
    pub fn get_phy_speed(&self) -> Option<&u32> {
        self.phy_speed.as_ref()
    }

    /// Sets the value of PhySupportSpeed
    pub fn set_phy_support_speed(&mut self, value: u32) {
        self.phy_support_speed = Some(value);
    }

    /// Gets the value of PhySupportSpeed
    pub fn get_phy_support_speed(&self) -> Option<&u32> {
        self.phy_support_speed.as_ref()
    }

    /// Sets the value of PhyType
    pub fn set_phy_type(&mut self, value: u8) {
        self.phy_type = Some(value);
    }

    /// Gets the value of PhyType
    pub fn get_phy_type(&self) -> Option<&u8> {
        self.phy_type.as_ref()
    }
}

