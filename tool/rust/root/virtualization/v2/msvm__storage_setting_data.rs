// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_StorageSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_StorageSettingData {
    #[serde(flatten)]
    pub base: Msvm_SystemComponentSettingData,

/// 
    #[serde(rename = "DisableInterruptBatching")]
    pub disable_interrupt_batching: Option<bool>,

/// 
    #[serde(rename = "ThreadCountPerChannel")]
    pub thread_count_per_channel: Option<StorageSettingData_ThreadCountPerChannel>,

/// 
    #[serde(rename = "VirtualProcessorsPerChannel")]
    pub virtual_processors_per_channel: Option<u16>,
}

impl Msvm_StorageSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_SystemComponentSettingData::new(),
            disable_interrupt_batching: None,
            thread_count_per_channel: None,
            virtual_processors_per_channel: None,
        }
    }


    /// Sets the value of DisableInterruptBatching
    pub fn set_disable_interrupt_batching(&mut self, value: bool) {
        self.disable_interrupt_batching = Some(value);
    }

    /// Gets the value of DisableInterruptBatching
    pub fn get_disable_interrupt_batching(&self) -> Option<&bool> {
        self.disable_interrupt_batching.as_ref()
    }

    /// Sets the value of ThreadCountPerChannel
    pub fn set_thread_count_per_channel(&mut self, value: StorageSettingData_ThreadCountPerChannel) {
        self.thread_count_per_channel = Some(value);
    }

    /// Gets the value of ThreadCountPerChannel
    pub fn get_thread_count_per_channel(&self) -> Option<&StorageSettingData_ThreadCountPerChannel> {
        self.thread_count_per_channel.as_ref()
    }

    /// Sets the value of VirtualProcessorsPerChannel
    pub fn set_virtual_processors_per_channel(&mut self, value: u16) {
        self.virtual_processors_per_channel = Some(value);
    }

    /// Gets the value of VirtualProcessorsPerChannel
    pub fn get_virtual_processors_per_channel(&self) -> Option<&u16> {
        self.virtual_processors_per_channel.as_ref()
    }
}

impl Msvm_StorageSettingData {
    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Msvm_VirtualSystemSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemSettingData")
    }

}

