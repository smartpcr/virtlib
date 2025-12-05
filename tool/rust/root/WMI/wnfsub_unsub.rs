// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WNFSubUnsub struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WNFSubUnsub {
    #[serde(flatten)]
    pub base: WNFTrace,

/// 
    #[serde(rename = "Callback")]
    pub callback: Option<u32>,

/// 
    #[serde(rename = "DeliveryFlags")]
    pub delivery_flags: Option<u32>,

/// 
    #[serde(rename = "NameSub")]
    pub name_sub: Option<u32>,

/// 
    #[serde(rename = "RefCount")]
    pub ref_count: Option<u32>,

/// 
    #[serde(rename = "StateName")]
    pub state_name: Option<u64>,

/// 
    #[serde(rename = "Subscription")]
    pub subscription: Option<u32>,
}

impl WNFSubUnsub {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WNFTrace::new(),
            callback: None,
            delivery_flags: None,
            name_sub: None,
            ref_count: None,
            state_name: None,
            subscription: None,
        }
    }


    /// Sets the value of Callback
    pub fn set_callback(&mut self, value: u32) {
        self.callback = Some(value);
    }

    /// Gets the value of Callback
    pub fn get_callback(&self) -> Option<&u32> {
        self.callback.as_ref()
    }

    /// Sets the value of DeliveryFlags
    pub fn set_delivery_flags(&mut self, value: u32) {
        self.delivery_flags = Some(value);
    }

    /// Gets the value of DeliveryFlags
    pub fn get_delivery_flags(&self) -> Option<&u32> {
        self.delivery_flags.as_ref()
    }

    /// Sets the value of NameSub
    pub fn set_name_sub(&mut self, value: u32) {
        self.name_sub = Some(value);
    }

    /// Gets the value of NameSub
    pub fn get_name_sub(&self) -> Option<&u32> {
        self.name_sub.as_ref()
    }

    /// Sets the value of RefCount
    pub fn set_ref_count(&mut self, value: u32) {
        self.ref_count = Some(value);
    }

    /// Gets the value of RefCount
    pub fn get_ref_count(&self) -> Option<&u32> {
        self.ref_count.as_ref()
    }

    /// Sets the value of StateName
    pub fn set_state_name(&mut self, value: u64) {
        self.state_name = Some(value);
    }

    /// Gets the value of StateName
    pub fn get_state_name(&self) -> Option<&u64> {
        self.state_name.as_ref()
    }

    /// Sets the value of Subscription
    pub fn set_subscription(&mut self, value: u32) {
        self.subscription = Some(value);
    }

    /// Gets the value of Subscription
    pub fn get_subscription(&self) -> Option<&u32> {
        self.subscription.as_ref()
    }
}

