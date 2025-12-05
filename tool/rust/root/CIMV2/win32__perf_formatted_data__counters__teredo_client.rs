// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_TeredoClient struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_TeredoClient {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "InTeredoBubble")]
    pub in_teredo_bubble: Option<u32>,

/// 
    #[serde(rename = "InTeredoData")]
    pub in_teredo_data: Option<u64>,

/// 
    #[serde(rename = "InTeredoDataKernelMode")]
    pub in_teredo_data_kernel_mode: Option<u64>,

/// 
    #[serde(rename = "InTeredoDataUserMode")]
    pub in_teredo_data_user_mode: Option<u64>,

/// 
    #[serde(rename = "InTeredoInvalid")]
    pub in_teredo_invalid: Option<u32>,

/// 
    #[serde(rename = "InTeredoRouterAdvertisement")]
    pub in_teredo_router_advertisement: Option<u32>,

/// 
    #[serde(rename = "OutTeredoBubble")]
    pub out_teredo_bubble: Option<u32>,

/// 
    #[serde(rename = "OutTeredoData")]
    pub out_teredo_data: Option<u64>,

/// 
    #[serde(rename = "OutTeredoDataKernelMode")]
    pub out_teredo_data_kernel_mode: Option<u64>,

/// 
    #[serde(rename = "OutTeredoDataUserMode")]
    pub out_teredo_data_user_mode: Option<u64>,

/// 
    #[serde(rename = "OutTeredoRouterSolicitation")]
    pub out_teredo_router_solicitation: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_TeredoClient {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            in_teredo_bubble: None,
            in_teredo_data: None,
            in_teredo_data_kernel_mode: None,
            in_teredo_data_user_mode: None,
            in_teredo_invalid: None,
            in_teredo_router_advertisement: None,
            out_teredo_bubble: None,
            out_teredo_data: None,
            out_teredo_data_kernel_mode: None,
            out_teredo_data_user_mode: None,
            out_teredo_router_solicitation: None,
        }
    }


    /// Sets the value of InTeredoBubble
    pub fn set_in_teredo_bubble(&mut self, value: u32) {
        self.in_teredo_bubble = Some(value);
    }

    /// Gets the value of InTeredoBubble
    pub fn get_in_teredo_bubble(&self) -> Option<&u32> {
        self.in_teredo_bubble.as_ref()
    }

    /// Sets the value of InTeredoData
    pub fn set_in_teredo_data(&mut self, value: u64) {
        self.in_teredo_data = Some(value);
    }

    /// Gets the value of InTeredoData
    pub fn get_in_teredo_data(&self) -> Option<&u64> {
        self.in_teredo_data.as_ref()
    }

    /// Sets the value of InTeredoDataKernelMode
    pub fn set_in_teredo_data_kernel_mode(&mut self, value: u64) {
        self.in_teredo_data_kernel_mode = Some(value);
    }

    /// Gets the value of InTeredoDataKernelMode
    pub fn get_in_teredo_data_kernel_mode(&self) -> Option<&u64> {
        self.in_teredo_data_kernel_mode.as_ref()
    }

    /// Sets the value of InTeredoDataUserMode
    pub fn set_in_teredo_data_user_mode(&mut self, value: u64) {
        self.in_teredo_data_user_mode = Some(value);
    }

    /// Gets the value of InTeredoDataUserMode
    pub fn get_in_teredo_data_user_mode(&self) -> Option<&u64> {
        self.in_teredo_data_user_mode.as_ref()
    }

    /// Sets the value of InTeredoInvalid
    pub fn set_in_teredo_invalid(&mut self, value: u32) {
        self.in_teredo_invalid = Some(value);
    }

    /// Gets the value of InTeredoInvalid
    pub fn get_in_teredo_invalid(&self) -> Option<&u32> {
        self.in_teredo_invalid.as_ref()
    }

    /// Sets the value of InTeredoRouterAdvertisement
    pub fn set_in_teredo_router_advertisement(&mut self, value: u32) {
        self.in_teredo_router_advertisement = Some(value);
    }

    /// Gets the value of InTeredoRouterAdvertisement
    pub fn get_in_teredo_router_advertisement(&self) -> Option<&u32> {
        self.in_teredo_router_advertisement.as_ref()
    }

    /// Sets the value of OutTeredoBubble
    pub fn set_out_teredo_bubble(&mut self, value: u32) {
        self.out_teredo_bubble = Some(value);
    }

    /// Gets the value of OutTeredoBubble
    pub fn get_out_teredo_bubble(&self) -> Option<&u32> {
        self.out_teredo_bubble.as_ref()
    }

    /// Sets the value of OutTeredoData
    pub fn set_out_teredo_data(&mut self, value: u64) {
        self.out_teredo_data = Some(value);
    }

    /// Gets the value of OutTeredoData
    pub fn get_out_teredo_data(&self) -> Option<&u64> {
        self.out_teredo_data.as_ref()
    }

    /// Sets the value of OutTeredoDataKernelMode
    pub fn set_out_teredo_data_kernel_mode(&mut self, value: u64) {
        self.out_teredo_data_kernel_mode = Some(value);
    }

    /// Gets the value of OutTeredoDataKernelMode
    pub fn get_out_teredo_data_kernel_mode(&self) -> Option<&u64> {
        self.out_teredo_data_kernel_mode.as_ref()
    }

    /// Sets the value of OutTeredoDataUserMode
    pub fn set_out_teredo_data_user_mode(&mut self, value: u64) {
        self.out_teredo_data_user_mode = Some(value);
    }

    /// Gets the value of OutTeredoDataUserMode
    pub fn get_out_teredo_data_user_mode(&self) -> Option<&u64> {
        self.out_teredo_data_user_mode.as_ref()
    }

    /// Sets the value of OutTeredoRouterSolicitation
    pub fn set_out_teredo_router_solicitation(&mut self, value: u32) {
        self.out_teredo_router_solicitation = Some(value);
    }

    /// Gets the value of OutTeredoRouterSolicitation
    pub fn get_out_teredo_router_solicitation(&self) -> Option<&u32> {
        self.out_teredo_router_solicitation.as_ref()
    }
}

