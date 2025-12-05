// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_NETFramework_NETCLRMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_NETFramework_NETCLRMemory {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AllocatedBytesPersec")]
    pub allocated_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "FinalizationSurvivors")]
    pub finalization_survivors: Option<u64>,

/// 
    #[serde(rename = "Gen0heapsize")]
    pub gen0heapsize: Option<u64>,

/// 
    #[serde(rename = "Gen0PromotedBytesPerSec")]
    pub gen0_promoted_bytes_per_sec: Option<u64>,

/// 
    #[serde(rename = "Gen1heapsize")]
    pub gen1heapsize: Option<u64>,

/// 
    #[serde(rename = "Gen1PromotedBytesPerSec")]
    pub gen1_promoted_bytes_per_sec: Option<u64>,

/// 
    #[serde(rename = "Gen2heapsize")]
    pub gen2heapsize: Option<u64>,

/// 
    #[serde(rename = "LargeObjectHeapsize")]
    pub large_object_heapsize: Option<u64>,

/// 
    #[serde(rename = "NumberBytesinallHeaps")]
    pub number_bytesinall_heaps: Option<u64>,

/// 
    #[serde(rename = "NumberGCHandles")]
    pub number_gchandles: Option<u64>,

/// 
    #[serde(rename = "NumberGen0Collections")]
    pub number_gen0_collections: Option<u64>,

/// 
    #[serde(rename = "NumberGen1Collections")]
    pub number_gen1_collections: Option<u64>,

/// 
    #[serde(rename = "NumberGen2Collections")]
    pub number_gen2_collections: Option<u64>,

/// 
    #[serde(rename = "NumberInducedGC")]
    pub number_induced_gc: Option<u64>,

/// 
    #[serde(rename = "NumberofPinnedObjects")]
    pub numberof_pinned_objects: Option<u64>,

/// 
    #[serde(rename = "NumberofSinkBlocksinuse")]
    pub numberof_sink_blocksinuse: Option<u64>,

/// 
    #[serde(rename = "NumberTotalcommittedBytes")]
    pub number_totalcommitted_bytes: Option<u64>,

/// 
    #[serde(rename = "NumberTotalreservedBytes")]
    pub number_totalreserved_bytes: Option<u64>,

/// 
    #[serde(rename = "PercentTimeinGC")]
    pub percent_timein_gc: Option<u32>,

/// 
    #[serde(rename = "PercentTimeinGC_Base")]
    pub percent_timein_gc__base: Option<u32>,

/// 
    #[serde(rename = "ProcessID")]
    pub process_id: Option<u64>,

/// 
    #[serde(rename = "PromotedFinalizationMemoryfromGen0")]
    pub promoted_finalization_memoryfrom_gen0: Option<u64>,

/// 
    #[serde(rename = "PromotedMemoryfromGen0")]
    pub promoted_memoryfrom_gen0: Option<u64>,

/// 
    #[serde(rename = "PromotedMemoryfromGen1")]
    pub promoted_memoryfrom_gen1: Option<u64>,
}

impl Win32_PerfRawData_NETFramework_NETCLRMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            allocated_bytes_persec: None,
            finalization_survivors: None,
            gen0heapsize: None,
            gen0_promoted_bytes_per_sec: None,
            gen1heapsize: None,
            gen1_promoted_bytes_per_sec: None,
            gen2heapsize: None,
            large_object_heapsize: None,
            number_bytesinall_heaps: None,
            number_gchandles: None,
            number_gen0_collections: None,
            number_gen1_collections: None,
            number_gen2_collections: None,
            number_induced_gc: None,
            numberof_pinned_objects: None,
            numberof_sink_blocksinuse: None,
            number_totalcommitted_bytes: None,
            number_totalreserved_bytes: None,
            percent_timein_gc: None,
            percent_timein_gc__base: None,
            process_id: None,
            promoted_finalization_memoryfrom_gen0: None,
            promoted_memoryfrom_gen0: None,
            promoted_memoryfrom_gen1: None,
        }
    }


    /// Sets the value of AllocatedBytesPersec
    pub fn set_allocated_bytes_persec(&mut self, value: u64) {
        self.allocated_bytes_persec = Some(value);
    }

    /// Gets the value of AllocatedBytesPersec
    pub fn get_allocated_bytes_persec(&self) -> Option<&u64> {
        self.allocated_bytes_persec.as_ref()
    }

    /// Sets the value of FinalizationSurvivors
    pub fn set_finalization_survivors(&mut self, value: u64) {
        self.finalization_survivors = Some(value);
    }

    /// Gets the value of FinalizationSurvivors
    pub fn get_finalization_survivors(&self) -> Option<&u64> {
        self.finalization_survivors.as_ref()
    }

    /// Sets the value of Gen0heapsize
    pub fn set_gen0heapsize(&mut self, value: u64) {
        self.gen0heapsize = Some(value);
    }

    /// Gets the value of Gen0heapsize
    pub fn get_gen0heapsize(&self) -> Option<&u64> {
        self.gen0heapsize.as_ref()
    }

    /// Sets the value of Gen0PromotedBytesPerSec
    pub fn set_gen0_promoted_bytes_per_sec(&mut self, value: u64) {
        self.gen0_promoted_bytes_per_sec = Some(value);
    }

    /// Gets the value of Gen0PromotedBytesPerSec
    pub fn get_gen0_promoted_bytes_per_sec(&self) -> Option<&u64> {
        self.gen0_promoted_bytes_per_sec.as_ref()
    }

    /// Sets the value of Gen1heapsize
    pub fn set_gen1heapsize(&mut self, value: u64) {
        self.gen1heapsize = Some(value);
    }

    /// Gets the value of Gen1heapsize
    pub fn get_gen1heapsize(&self) -> Option<&u64> {
        self.gen1heapsize.as_ref()
    }

    /// Sets the value of Gen1PromotedBytesPerSec
    pub fn set_gen1_promoted_bytes_per_sec(&mut self, value: u64) {
        self.gen1_promoted_bytes_per_sec = Some(value);
    }

    /// Gets the value of Gen1PromotedBytesPerSec
    pub fn get_gen1_promoted_bytes_per_sec(&self) -> Option<&u64> {
        self.gen1_promoted_bytes_per_sec.as_ref()
    }

    /// Sets the value of Gen2heapsize
    pub fn set_gen2heapsize(&mut self, value: u64) {
        self.gen2heapsize = Some(value);
    }

    /// Gets the value of Gen2heapsize
    pub fn get_gen2heapsize(&self) -> Option<&u64> {
        self.gen2heapsize.as_ref()
    }

    /// Sets the value of LargeObjectHeapsize
    pub fn set_large_object_heapsize(&mut self, value: u64) {
        self.large_object_heapsize = Some(value);
    }

    /// Gets the value of LargeObjectHeapsize
    pub fn get_large_object_heapsize(&self) -> Option<&u64> {
        self.large_object_heapsize.as_ref()
    }

    /// Sets the value of NumberBytesinallHeaps
    pub fn set_number_bytesinall_heaps(&mut self, value: u64) {
        self.number_bytesinall_heaps = Some(value);
    }

    /// Gets the value of NumberBytesinallHeaps
    pub fn get_number_bytesinall_heaps(&self) -> Option<&u64> {
        self.number_bytesinall_heaps.as_ref()
    }

    /// Sets the value of NumberGCHandles
    pub fn set_number_gchandles(&mut self, value: u64) {
        self.number_gchandles = Some(value);
    }

    /// Gets the value of NumberGCHandles
    pub fn get_number_gchandles(&self) -> Option<&u64> {
        self.number_gchandles.as_ref()
    }

    /// Sets the value of NumberGen0Collections
    pub fn set_number_gen0_collections(&mut self, value: u64) {
        self.number_gen0_collections = Some(value);
    }

    /// Gets the value of NumberGen0Collections
    pub fn get_number_gen0_collections(&self) -> Option<&u64> {
        self.number_gen0_collections.as_ref()
    }

    /// Sets the value of NumberGen1Collections
    pub fn set_number_gen1_collections(&mut self, value: u64) {
        self.number_gen1_collections = Some(value);
    }

    /// Gets the value of NumberGen1Collections
    pub fn get_number_gen1_collections(&self) -> Option<&u64> {
        self.number_gen1_collections.as_ref()
    }

    /// Sets the value of NumberGen2Collections
    pub fn set_number_gen2_collections(&mut self, value: u64) {
        self.number_gen2_collections = Some(value);
    }

    /// Gets the value of NumberGen2Collections
    pub fn get_number_gen2_collections(&self) -> Option<&u64> {
        self.number_gen2_collections.as_ref()
    }

    /// Sets the value of NumberInducedGC
    pub fn set_number_induced_gc(&mut self, value: u64) {
        self.number_induced_gc = Some(value);
    }

    /// Gets the value of NumberInducedGC
    pub fn get_number_induced_gc(&self) -> Option<&u64> {
        self.number_induced_gc.as_ref()
    }

    /// Sets the value of NumberofPinnedObjects
    pub fn set_numberof_pinned_objects(&mut self, value: u64) {
        self.numberof_pinned_objects = Some(value);
    }

    /// Gets the value of NumberofPinnedObjects
    pub fn get_numberof_pinned_objects(&self) -> Option<&u64> {
        self.numberof_pinned_objects.as_ref()
    }

    /// Sets the value of NumberofSinkBlocksinuse
    pub fn set_numberof_sink_blocksinuse(&mut self, value: u64) {
        self.numberof_sink_blocksinuse = Some(value);
    }

    /// Gets the value of NumberofSinkBlocksinuse
    pub fn get_numberof_sink_blocksinuse(&self) -> Option<&u64> {
        self.numberof_sink_blocksinuse.as_ref()
    }

    /// Sets the value of NumberTotalcommittedBytes
    pub fn set_number_totalcommitted_bytes(&mut self, value: u64) {
        self.number_totalcommitted_bytes = Some(value);
    }

    /// Gets the value of NumberTotalcommittedBytes
    pub fn get_number_totalcommitted_bytes(&self) -> Option<&u64> {
        self.number_totalcommitted_bytes.as_ref()
    }

    /// Sets the value of NumberTotalreservedBytes
    pub fn set_number_totalreserved_bytes(&mut self, value: u64) {
        self.number_totalreserved_bytes = Some(value);
    }

    /// Gets the value of NumberTotalreservedBytes
    pub fn get_number_totalreserved_bytes(&self) -> Option<&u64> {
        self.number_totalreserved_bytes.as_ref()
    }

    /// Sets the value of PercentTimeinGC
    pub fn set_percent_timein_gc(&mut self, value: u32) {
        self.percent_timein_gc = Some(value);
    }

    /// Gets the value of PercentTimeinGC
    pub fn get_percent_timein_gc(&self) -> Option<&u32> {
        self.percent_timein_gc.as_ref()
    }

    /// Sets the value of PercentTimeinGC_Base
    pub fn set_percent_timein_gc__base(&mut self, value: u32) {
        self.percent_timein_gc__base = Some(value);
    }

    /// Gets the value of PercentTimeinGC_Base
    pub fn get_percent_timein_gc__base(&self) -> Option<&u32> {
        self.percent_timein_gc__base.as_ref()
    }

    /// Sets the value of ProcessID
    pub fn set_process_id(&mut self, value: u64) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessID
    pub fn get_process_id(&self) -> Option<&u64> {
        self.process_id.as_ref()
    }

    /// Sets the value of PromotedFinalizationMemoryfromGen0
    pub fn set_promoted_finalization_memoryfrom_gen0(&mut self, value: u64) {
        self.promoted_finalization_memoryfrom_gen0 = Some(value);
    }

    /// Gets the value of PromotedFinalizationMemoryfromGen0
    pub fn get_promoted_finalization_memoryfrom_gen0(&self) -> Option<&u64> {
        self.promoted_finalization_memoryfrom_gen0.as_ref()
    }

    /// Sets the value of PromotedMemoryfromGen0
    pub fn set_promoted_memoryfrom_gen0(&mut self, value: u64) {
        self.promoted_memoryfrom_gen0 = Some(value);
    }

    /// Gets the value of PromotedMemoryfromGen0
    pub fn get_promoted_memoryfrom_gen0(&self) -> Option<&u64> {
        self.promoted_memoryfrom_gen0.as_ref()
    }

    /// Sets the value of PromotedMemoryfromGen1
    pub fn set_promoted_memoryfrom_gen1(&mut self, value: u64) {
        self.promoted_memoryfrom_gen1 = Some(value);
    }

    /// Gets the value of PromotedMemoryfromGen1
    pub fn get_promoted_memoryfrom_gen1(&self) -> Option<&u64> {
        self.promoted_memoryfrom_gen1.as_ref()
    }
}

