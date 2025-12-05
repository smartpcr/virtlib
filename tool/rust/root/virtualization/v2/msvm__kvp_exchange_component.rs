// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_KvpExchangeComponent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_KvpExchangeComponent {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "GuestExchangeItems")]
    pub guest_exchange_items: Vec<String>,

/// 
    #[serde(rename = "GuestIntrinsicExchangeItems")]
    pub guest_intrinsic_exchange_items: Vec<String>,

/// 
    #[serde(rename = "GuestIntrinsicExchangeItemsHypervisorOnly")]
    pub guest_intrinsic_exchange_items_hypervisor_only: Vec<String>,
}

impl Msvm_KvpExchangeComponent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            guest_exchange_items: Vec::new(),
            guest_intrinsic_exchange_items: Vec::new(),
            guest_intrinsic_exchange_items_hypervisor_only: Vec::new(),
        }
    }


    /// Sets the value of GuestExchangeItems
    pub fn set_guest_exchange_items(&mut self, value: Vec<String>) {
        self.guest_exchange_items = value;
    }

    /// Gets the value of GuestExchangeItems
    pub fn get_guest_exchange_items(&self) -> &Vec<String> {
        &self.guest_exchange_items
    }

    /// Sets the value of GuestIntrinsicExchangeItems
    pub fn set_guest_intrinsic_exchange_items(&mut self, value: Vec<String>) {
        self.guest_intrinsic_exchange_items = value;
    }

    /// Gets the value of GuestIntrinsicExchangeItems
    pub fn get_guest_intrinsic_exchange_items(&self) -> &Vec<String> {
        &self.guest_intrinsic_exchange_items
    }

    /// Sets the value of GuestIntrinsicExchangeItemsHypervisorOnly
    pub fn set_guest_intrinsic_exchange_items_hypervisor_only(&mut self, value: Vec<String>) {
        self.guest_intrinsic_exchange_items_hypervisor_only = value;
    }

    /// Gets the value of GuestIntrinsicExchangeItemsHypervisorOnly
    pub fn get_guest_intrinsic_exchange_items_hypervisor_only(&self) -> &Vec<String> {
        &self.guest_intrinsic_exchange_items_hypervisor_only
    }
}

