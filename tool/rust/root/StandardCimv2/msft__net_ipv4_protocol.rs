// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIPv4Protocol struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIPv4Protocol {
    #[serde(flatten)]
    pub base: MSFT_NetBaseIPProtocol,

/// 
    #[serde(rename = "MinimumMtu")]
    pub minimum_mtu: Option<u32>,

/// 
    #[serde(rename = "MultipleArpAnnouncements")]
    pub multiple_arp_announcements: Option<u8>,
}

impl MSFT_NetIPv4Protocol {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetBaseIPProtocol::new(),
            minimum_mtu: None,
            multiple_arp_announcements: None,
        }
    }


    /// Sets the value of MinimumMtu
    pub fn set_minimum_mtu(&mut self, value: u32) {
        self.minimum_mtu = Some(value);
    }

    /// Gets the value of MinimumMtu
    pub fn get_minimum_mtu(&self) -> Option<&u32> {
        self.minimum_mtu.as_ref()
    }

    /// Sets the value of MultipleArpAnnouncements
    pub fn set_multiple_arp_announcements(&mut self, value: u8) {
        self.multiple_arp_announcements = Some(value);
    }

    /// Gets the value of MultipleArpAnnouncements
    pub fn get_multiple_arp_announcements(&self) -> Option<&u8> {
        self.multiple_arp_announcements.as_ref()
    }
}

