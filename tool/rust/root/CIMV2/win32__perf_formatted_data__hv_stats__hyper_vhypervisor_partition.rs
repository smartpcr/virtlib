// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_HvStats_HyperVHypervisorPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_HvStats_HyperVHypervisorPartition {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AddressSpaces")]
    pub address_spaces: Option<u64>,

/// 
    #[serde(rename = "AttachedDevices")]
    pub attached_devices: Option<u64>,

/// 
    #[serde(rename = "CPPCRequestedPerformanceLevel")]
    pub cppcrequested_performance_level: Option<u64>,

/// 
    #[serde(rename = "DepositedPages")]
    pub deposited_pages: Option<u64>,

/// 
    #[serde(rename = "DeviceDMAErrors")]
    pub device_dmaerrors: Option<u64>,

/// 
    #[serde(rename = "DeviceInterruptErrors")]
    pub device_interrupt_errors: Option<u64>,

/// 
    #[serde(rename = "DeviceInterruptMappings")]
    pub device_interrupt_mappings: Option<u64>,

/// 
    #[serde(rename = "DeviceInterruptThrottleEvents")]
    pub device_interrupt_throttle_events: Option<u64>,

/// 
    #[serde(rename = "GPAPages")]
    pub gpapages: Option<u64>,

/// 
    #[serde(rename = "GPASpaceModificationsPersec")]
    pub gpaspace_modifications_persec: Option<u64>,

/// 
    #[serde(rename = "IOTLBFlushCost")]
    pub iotlbflush_cost: Option<u64>,

/// 
    #[serde(rename = "IOTLBFlushesPersec")]
    pub iotlbflushes_persec: Option<u64>,

/// 
    #[serde(rename = "NestedTLBFreeListSize")]
    pub nested_tlbfree_list_size: Option<u64>,

/// 
    #[serde(rename = "NestedTLBSize")]
    pub nested_tlbsize: Option<u64>,

/// 
    #[serde(rename = "NestedTLBTrimmedPagesPersec")]
    pub nested_tlbtrimmed_pages_persec: Option<u64>,

/// 
    #[serde(rename = "PagesRecombinedPersec")]
    pub pages_recombined_persec: Option<u64>,

/// 
    #[serde(rename = "PagesShatteredPersec")]
    pub pages_shattered_persec: Option<u64>,

/// 
    #[serde(rename = "RecommendedNestedTLBSize")]
    pub recommended_nested_tlbsize: Option<u64>,

/// 
    #[serde(rename = "RecommendedVirtualTLBSize")]
    pub recommended_virtual_tlbsize: Option<u64>,

/// 
    #[serde(rename = "SkippedTimerTicks")]
    pub skipped_timer_ticks: Option<u64>,

/// 
    #[serde(rename = "Value1Gdevicepages")]
    pub value1_gdevicepages: Option<u64>,

/// 
    #[serde(rename = "Value1GGPApages")]
    pub value1_ggpapages: Option<u64>,

/// 
    #[serde(rename = "Value2Mdevicepages")]
    pub value2_mdevicepages: Option<u64>,

/// 
    #[serde(rename = "Value2MGPApages")]
    pub value2_mgpapages: Option<u64>,

/// 
    #[serde(rename = "Value4Kdevicepages")]
    pub value4_kdevicepages: Option<u64>,

/// 
    #[serde(rename = "Value4KGPApages")]
    pub value4_kgpapages: Option<u64>,

/// 
    #[serde(rename = "VirtualProcessors")]
    pub virtual_processors: Option<u64>,

/// 
    #[serde(rename = "VirtualTLBFlushEntiresPersec")]
    pub virtual_tlbflush_entires_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualTLBPages")]
    pub virtual_tlbpages: Option<u64>,
}

impl Win32_PerfFormattedData_HvStats_HyperVHypervisorPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            address_spaces: None,
            attached_devices: None,
            cppcrequested_performance_level: None,
            deposited_pages: None,
            device_dmaerrors: None,
            device_interrupt_errors: None,
            device_interrupt_mappings: None,
            device_interrupt_throttle_events: None,
            gpapages: None,
            gpaspace_modifications_persec: None,
            iotlbflush_cost: None,
            iotlbflushes_persec: None,
            nested_tlbfree_list_size: None,
            nested_tlbsize: None,
            nested_tlbtrimmed_pages_persec: None,
            pages_recombined_persec: None,
            pages_shattered_persec: None,
            recommended_nested_tlbsize: None,
            recommended_virtual_tlbsize: None,
            skipped_timer_ticks: None,
            value1_gdevicepages: None,
            value1_ggpapages: None,
            value2_mdevicepages: None,
            value2_mgpapages: None,
            value4_kdevicepages: None,
            value4_kgpapages: None,
            virtual_processors: None,
            virtual_tlbflush_entires_persec: None,
            virtual_tlbpages: None,
        }
    }


    /// Sets the value of AddressSpaces
    pub fn set_address_spaces(&mut self, value: u64) {
        self.address_spaces = Some(value);
    }

    /// Gets the value of AddressSpaces
    pub fn get_address_spaces(&self) -> Option<&u64> {
        self.address_spaces.as_ref()
    }

    /// Sets the value of AttachedDevices
    pub fn set_attached_devices(&mut self, value: u64) {
        self.attached_devices = Some(value);
    }

    /// Gets the value of AttachedDevices
    pub fn get_attached_devices(&self) -> Option<&u64> {
        self.attached_devices.as_ref()
    }

    /// Sets the value of CPPCRequestedPerformanceLevel
    pub fn set_cppcrequested_performance_level(&mut self, value: u64) {
        self.cppcrequested_performance_level = Some(value);
    }

    /// Gets the value of CPPCRequestedPerformanceLevel
    pub fn get_cppcrequested_performance_level(&self) -> Option<&u64> {
        self.cppcrequested_performance_level.as_ref()
    }

    /// Sets the value of DepositedPages
    pub fn set_deposited_pages(&mut self, value: u64) {
        self.deposited_pages = Some(value);
    }

    /// Gets the value of DepositedPages
    pub fn get_deposited_pages(&self) -> Option<&u64> {
        self.deposited_pages.as_ref()
    }

    /// Sets the value of DeviceDMAErrors
    pub fn set_device_dmaerrors(&mut self, value: u64) {
        self.device_dmaerrors = Some(value);
    }

    /// Gets the value of DeviceDMAErrors
    pub fn get_device_dmaerrors(&self) -> Option<&u64> {
        self.device_dmaerrors.as_ref()
    }

    /// Sets the value of DeviceInterruptErrors
    pub fn set_device_interrupt_errors(&mut self, value: u64) {
        self.device_interrupt_errors = Some(value);
    }

    /// Gets the value of DeviceInterruptErrors
    pub fn get_device_interrupt_errors(&self) -> Option<&u64> {
        self.device_interrupt_errors.as_ref()
    }

    /// Sets the value of DeviceInterruptMappings
    pub fn set_device_interrupt_mappings(&mut self, value: u64) {
        self.device_interrupt_mappings = Some(value);
    }

    /// Gets the value of DeviceInterruptMappings
    pub fn get_device_interrupt_mappings(&self) -> Option<&u64> {
        self.device_interrupt_mappings.as_ref()
    }

    /// Sets the value of DeviceInterruptThrottleEvents
    pub fn set_device_interrupt_throttle_events(&mut self, value: u64) {
        self.device_interrupt_throttle_events = Some(value);
    }

    /// Gets the value of DeviceInterruptThrottleEvents
    pub fn get_device_interrupt_throttle_events(&self) -> Option<&u64> {
        self.device_interrupt_throttle_events.as_ref()
    }

    /// Sets the value of GPAPages
    pub fn set_gpapages(&mut self, value: u64) {
        self.gpapages = Some(value);
    }

    /// Gets the value of GPAPages
    pub fn get_gpapages(&self) -> Option<&u64> {
        self.gpapages.as_ref()
    }

    /// Sets the value of GPASpaceModificationsPersec
    pub fn set_gpaspace_modifications_persec(&mut self, value: u64) {
        self.gpaspace_modifications_persec = Some(value);
    }

    /// Gets the value of GPASpaceModificationsPersec
    pub fn get_gpaspace_modifications_persec(&self) -> Option<&u64> {
        self.gpaspace_modifications_persec.as_ref()
    }

    /// Sets the value of IOTLBFlushCost
    pub fn set_iotlbflush_cost(&mut self, value: u64) {
        self.iotlbflush_cost = Some(value);
    }

    /// Gets the value of IOTLBFlushCost
    pub fn get_iotlbflush_cost(&self) -> Option<&u64> {
        self.iotlbflush_cost.as_ref()
    }

    /// Sets the value of IOTLBFlushesPersec
    pub fn set_iotlbflushes_persec(&mut self, value: u64) {
        self.iotlbflushes_persec = Some(value);
    }

    /// Gets the value of IOTLBFlushesPersec
    pub fn get_iotlbflushes_persec(&self) -> Option<&u64> {
        self.iotlbflushes_persec.as_ref()
    }

    /// Sets the value of NestedTLBFreeListSize
    pub fn set_nested_tlbfree_list_size(&mut self, value: u64) {
        self.nested_tlbfree_list_size = Some(value);
    }

    /// Gets the value of NestedTLBFreeListSize
    pub fn get_nested_tlbfree_list_size(&self) -> Option<&u64> {
        self.nested_tlbfree_list_size.as_ref()
    }

    /// Sets the value of NestedTLBSize
    pub fn set_nested_tlbsize(&mut self, value: u64) {
        self.nested_tlbsize = Some(value);
    }

    /// Gets the value of NestedTLBSize
    pub fn get_nested_tlbsize(&self) -> Option<&u64> {
        self.nested_tlbsize.as_ref()
    }

    /// Sets the value of NestedTLBTrimmedPagesPersec
    pub fn set_nested_tlbtrimmed_pages_persec(&mut self, value: u64) {
        self.nested_tlbtrimmed_pages_persec = Some(value);
    }

    /// Gets the value of NestedTLBTrimmedPagesPersec
    pub fn get_nested_tlbtrimmed_pages_persec(&self) -> Option<&u64> {
        self.nested_tlbtrimmed_pages_persec.as_ref()
    }

    /// Sets the value of PagesRecombinedPersec
    pub fn set_pages_recombined_persec(&mut self, value: u64) {
        self.pages_recombined_persec = Some(value);
    }

    /// Gets the value of PagesRecombinedPersec
    pub fn get_pages_recombined_persec(&self) -> Option<&u64> {
        self.pages_recombined_persec.as_ref()
    }

    /// Sets the value of PagesShatteredPersec
    pub fn set_pages_shattered_persec(&mut self, value: u64) {
        self.pages_shattered_persec = Some(value);
    }

    /// Gets the value of PagesShatteredPersec
    pub fn get_pages_shattered_persec(&self) -> Option<&u64> {
        self.pages_shattered_persec.as_ref()
    }

    /// Sets the value of RecommendedNestedTLBSize
    pub fn set_recommended_nested_tlbsize(&mut self, value: u64) {
        self.recommended_nested_tlbsize = Some(value);
    }

    /// Gets the value of RecommendedNestedTLBSize
    pub fn get_recommended_nested_tlbsize(&self) -> Option<&u64> {
        self.recommended_nested_tlbsize.as_ref()
    }

    /// Sets the value of RecommendedVirtualTLBSize
    pub fn set_recommended_virtual_tlbsize(&mut self, value: u64) {
        self.recommended_virtual_tlbsize = Some(value);
    }

    /// Gets the value of RecommendedVirtualTLBSize
    pub fn get_recommended_virtual_tlbsize(&self) -> Option<&u64> {
        self.recommended_virtual_tlbsize.as_ref()
    }

    /// Sets the value of SkippedTimerTicks
    pub fn set_skipped_timer_ticks(&mut self, value: u64) {
        self.skipped_timer_ticks = Some(value);
    }

    /// Gets the value of SkippedTimerTicks
    pub fn get_skipped_timer_ticks(&self) -> Option<&u64> {
        self.skipped_timer_ticks.as_ref()
    }

    /// Sets the value of Value1Gdevicepages
    pub fn set_value1_gdevicepages(&mut self, value: u64) {
        self.value1_gdevicepages = Some(value);
    }

    /// Gets the value of Value1Gdevicepages
    pub fn get_value1_gdevicepages(&self) -> Option<&u64> {
        self.value1_gdevicepages.as_ref()
    }

    /// Sets the value of Value1GGPApages
    pub fn set_value1_ggpapages(&mut self, value: u64) {
        self.value1_ggpapages = Some(value);
    }

    /// Gets the value of Value1GGPApages
    pub fn get_value1_ggpapages(&self) -> Option<&u64> {
        self.value1_ggpapages.as_ref()
    }

    /// Sets the value of Value2Mdevicepages
    pub fn set_value2_mdevicepages(&mut self, value: u64) {
        self.value2_mdevicepages = Some(value);
    }

    /// Gets the value of Value2Mdevicepages
    pub fn get_value2_mdevicepages(&self) -> Option<&u64> {
        self.value2_mdevicepages.as_ref()
    }

    /// Sets the value of Value2MGPApages
    pub fn set_value2_mgpapages(&mut self, value: u64) {
        self.value2_mgpapages = Some(value);
    }

    /// Gets the value of Value2MGPApages
    pub fn get_value2_mgpapages(&self) -> Option<&u64> {
        self.value2_mgpapages.as_ref()
    }

    /// Sets the value of Value4Kdevicepages
    pub fn set_value4_kdevicepages(&mut self, value: u64) {
        self.value4_kdevicepages = Some(value);
    }

    /// Gets the value of Value4Kdevicepages
    pub fn get_value4_kdevicepages(&self) -> Option<&u64> {
        self.value4_kdevicepages.as_ref()
    }

    /// Sets the value of Value4KGPApages
    pub fn set_value4_kgpapages(&mut self, value: u64) {
        self.value4_kgpapages = Some(value);
    }

    /// Gets the value of Value4KGPApages
    pub fn get_value4_kgpapages(&self) -> Option<&u64> {
        self.value4_kgpapages.as_ref()
    }

    /// Sets the value of VirtualProcessors
    pub fn set_virtual_processors(&mut self, value: u64) {
        self.virtual_processors = Some(value);
    }

    /// Gets the value of VirtualProcessors
    pub fn get_virtual_processors(&self) -> Option<&u64> {
        self.virtual_processors.as_ref()
    }

    /// Sets the value of VirtualTLBFlushEntiresPersec
    pub fn set_virtual_tlbflush_entires_persec(&mut self, value: u64) {
        self.virtual_tlbflush_entires_persec = Some(value);
    }

    /// Gets the value of VirtualTLBFlushEntiresPersec
    pub fn get_virtual_tlbflush_entires_persec(&self) -> Option<&u64> {
        self.virtual_tlbflush_entires_persec.as_ref()
    }

    /// Sets the value of VirtualTLBPages
    pub fn set_virtual_tlbpages(&mut self, value: u64) {
        self.virtual_tlbpages = Some(value);
    }

    /// Gets the value of VirtualTLBPages
    pub fn get_virtual_tlbpages(&self) -> Option<&u64> {
        self.virtual_tlbpages.as_ref()
    }
}

