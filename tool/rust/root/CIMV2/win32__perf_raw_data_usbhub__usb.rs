// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_usbhub_USB struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_usbhub_USB {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AvgBytesPerTransfer")]
    pub avg_bytes_per_transfer: Option<u64>,

/// 
    #[serde(rename = "AvgBytesPerTransfer_Base")]
    pub avg_bytes_per_transfer__base: Option<u32>,

/// 
    #[serde(rename = "AvgmslatencyforISOtransfers")]
    pub avgmslatencyfor_isotransfers: Option<u64>,

/// 
    #[serde(rename = "AvgmslatencyforISOtransfers_Base")]
    pub avgmslatencyfor_isotransfers__base: Option<u32>,

/// 
    #[serde(rename = "BulkBytesPerSec")]
    pub bulk_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "ControlDataBytesPerSec")]
    pub control_data_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "ControllerPCIInterruptsPerSec")]
    pub controller_pciinterrupts_per_sec: Option<u32>,

/// 
    #[serde(rename = "ControllerWorkSignalsPerSec")]
    pub controller_work_signals_per_sec: Option<u32>,

/// 
    #[serde(rename = "HostControllerAsyncCacheFlushCount")]
    pub host_controller_async_cache_flush_count: Option<u32>,

/// 
    #[serde(rename = "HostControllerAsyncIdle")]
    pub host_controller_async_idle: Option<u32>,

/// 
    #[serde(rename = "HostControllerIdle")]
    pub host_controller_idle: Option<u32>,

/// 
    #[serde(rename = "HostControllerPeriodicCacheFlushCount")]
    pub host_controller_periodic_cache_flush_count: Option<u32>,

/// 
    #[serde(rename = "HostControllerPeriodicIdle")]
    pub host_controller_periodic_idle: Option<u32>,

/// 
    #[serde(rename = "InterruptBytesPerSec")]
    pub interrupt_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "IsochronousBytesPerSec")]
    pub isochronous_bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "IsoPacketErrorsPerSec")]
    pub iso_packet_errors_per_sec: Option<u32>,

/// 
    #[serde(rename = "PercentTotalBandwidthUsedforInterrupt")]
    pub percent_total_bandwidth_usedfor_interrupt: Option<u32>,

/// 
    #[serde(rename = "PercentTotalBandwidthUsedforIso")]
    pub percent_total_bandwidth_usedfor_iso: Option<u32>,

/// 
    #[serde(rename = "TransferErrorsPerSec")]
    pub transfer_errors_per_sec: Option<u32>,
}

impl Win32_PerfRawData_usbhub_USB {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            avg_bytes_per_transfer: None,
            avg_bytes_per_transfer__base: None,
            avgmslatencyfor_isotransfers: None,
            avgmslatencyfor_isotransfers__base: None,
            bulk_bytes_per_sec: None,
            control_data_bytes_per_sec: None,
            controller_pciinterrupts_per_sec: None,
            controller_work_signals_per_sec: None,
            host_controller_async_cache_flush_count: None,
            host_controller_async_idle: None,
            host_controller_idle: None,
            host_controller_periodic_cache_flush_count: None,
            host_controller_periodic_idle: None,
            interrupt_bytes_per_sec: None,
            isochronous_bytes_per_sec: None,
            iso_packet_errors_per_sec: None,
            percent_total_bandwidth_usedfor_interrupt: None,
            percent_total_bandwidth_usedfor_iso: None,
            transfer_errors_per_sec: None,
        }
    }


    /// Sets the value of AvgBytesPerTransfer
    pub fn set_avg_bytes_per_transfer(&mut self, value: u64) {
        self.avg_bytes_per_transfer = Some(value);
    }

    /// Gets the value of AvgBytesPerTransfer
    pub fn get_avg_bytes_per_transfer(&self) -> Option<&u64> {
        self.avg_bytes_per_transfer.as_ref()
    }

    /// Sets the value of AvgBytesPerTransfer_Base
    pub fn set_avg_bytes_per_transfer__base(&mut self, value: u32) {
        self.avg_bytes_per_transfer__base = Some(value);
    }

    /// Gets the value of AvgBytesPerTransfer_Base
    pub fn get_avg_bytes_per_transfer__base(&self) -> Option<&u32> {
        self.avg_bytes_per_transfer__base.as_ref()
    }

    /// Sets the value of AvgmslatencyforISOtransfers
    pub fn set_avgmslatencyfor_isotransfers(&mut self, value: u64) {
        self.avgmslatencyfor_isotransfers = Some(value);
    }

    /// Gets the value of AvgmslatencyforISOtransfers
    pub fn get_avgmslatencyfor_isotransfers(&self) -> Option<&u64> {
        self.avgmslatencyfor_isotransfers.as_ref()
    }

    /// Sets the value of AvgmslatencyforISOtransfers_Base
    pub fn set_avgmslatencyfor_isotransfers__base(&mut self, value: u32) {
        self.avgmslatencyfor_isotransfers__base = Some(value);
    }

    /// Gets the value of AvgmslatencyforISOtransfers_Base
    pub fn get_avgmslatencyfor_isotransfers__base(&self) -> Option<&u32> {
        self.avgmslatencyfor_isotransfers__base.as_ref()
    }

    /// Sets the value of BulkBytesPerSec
    pub fn set_bulk_bytes_per_sec(&mut self, value: u32) {
        self.bulk_bytes_per_sec = Some(value);
    }

    /// Gets the value of BulkBytesPerSec
    pub fn get_bulk_bytes_per_sec(&self) -> Option<&u32> {
        self.bulk_bytes_per_sec.as_ref()
    }

    /// Sets the value of ControlDataBytesPerSec
    pub fn set_control_data_bytes_per_sec(&mut self, value: u32) {
        self.control_data_bytes_per_sec = Some(value);
    }

    /// Gets the value of ControlDataBytesPerSec
    pub fn get_control_data_bytes_per_sec(&self) -> Option<&u32> {
        self.control_data_bytes_per_sec.as_ref()
    }

    /// Sets the value of ControllerPCIInterruptsPerSec
    pub fn set_controller_pciinterrupts_per_sec(&mut self, value: u32) {
        self.controller_pciinterrupts_per_sec = Some(value);
    }

    /// Gets the value of ControllerPCIInterruptsPerSec
    pub fn get_controller_pciinterrupts_per_sec(&self) -> Option<&u32> {
        self.controller_pciinterrupts_per_sec.as_ref()
    }

    /// Sets the value of ControllerWorkSignalsPerSec
    pub fn set_controller_work_signals_per_sec(&mut self, value: u32) {
        self.controller_work_signals_per_sec = Some(value);
    }

    /// Gets the value of ControllerWorkSignalsPerSec
    pub fn get_controller_work_signals_per_sec(&self) -> Option<&u32> {
        self.controller_work_signals_per_sec.as_ref()
    }

    /// Sets the value of HostControllerAsyncCacheFlushCount
    pub fn set_host_controller_async_cache_flush_count(&mut self, value: u32) {
        self.host_controller_async_cache_flush_count = Some(value);
    }

    /// Gets the value of HostControllerAsyncCacheFlushCount
    pub fn get_host_controller_async_cache_flush_count(&self) -> Option<&u32> {
        self.host_controller_async_cache_flush_count.as_ref()
    }

    /// Sets the value of HostControllerAsyncIdle
    pub fn set_host_controller_async_idle(&mut self, value: u32) {
        self.host_controller_async_idle = Some(value);
    }

    /// Gets the value of HostControllerAsyncIdle
    pub fn get_host_controller_async_idle(&self) -> Option<&u32> {
        self.host_controller_async_idle.as_ref()
    }

    /// Sets the value of HostControllerIdle
    pub fn set_host_controller_idle(&mut self, value: u32) {
        self.host_controller_idle = Some(value);
    }

    /// Gets the value of HostControllerIdle
    pub fn get_host_controller_idle(&self) -> Option<&u32> {
        self.host_controller_idle.as_ref()
    }

    /// Sets the value of HostControllerPeriodicCacheFlushCount
    pub fn set_host_controller_periodic_cache_flush_count(&mut self, value: u32) {
        self.host_controller_periodic_cache_flush_count = Some(value);
    }

    /// Gets the value of HostControllerPeriodicCacheFlushCount
    pub fn get_host_controller_periodic_cache_flush_count(&self) -> Option<&u32> {
        self.host_controller_periodic_cache_flush_count.as_ref()
    }

    /// Sets the value of HostControllerPeriodicIdle
    pub fn set_host_controller_periodic_idle(&mut self, value: u32) {
        self.host_controller_periodic_idle = Some(value);
    }

    /// Gets the value of HostControllerPeriodicIdle
    pub fn get_host_controller_periodic_idle(&self) -> Option<&u32> {
        self.host_controller_periodic_idle.as_ref()
    }

    /// Sets the value of InterruptBytesPerSec
    pub fn set_interrupt_bytes_per_sec(&mut self, value: u32) {
        self.interrupt_bytes_per_sec = Some(value);
    }

    /// Gets the value of InterruptBytesPerSec
    pub fn get_interrupt_bytes_per_sec(&self) -> Option<&u32> {
        self.interrupt_bytes_per_sec.as_ref()
    }

    /// Sets the value of IsochronousBytesPerSec
    pub fn set_isochronous_bytes_per_sec(&mut self, value: u32) {
        self.isochronous_bytes_per_sec = Some(value);
    }

    /// Gets the value of IsochronousBytesPerSec
    pub fn get_isochronous_bytes_per_sec(&self) -> Option<&u32> {
        self.isochronous_bytes_per_sec.as_ref()
    }

    /// Sets the value of IsoPacketErrorsPerSec
    pub fn set_iso_packet_errors_per_sec(&mut self, value: u32) {
        self.iso_packet_errors_per_sec = Some(value);
    }

    /// Gets the value of IsoPacketErrorsPerSec
    pub fn get_iso_packet_errors_per_sec(&self) -> Option<&u32> {
        self.iso_packet_errors_per_sec.as_ref()
    }

    /// Sets the value of PercentTotalBandwidthUsedforInterrupt
    pub fn set_percent_total_bandwidth_usedfor_interrupt(&mut self, value: u32) {
        self.percent_total_bandwidth_usedfor_interrupt = Some(value);
    }

    /// Gets the value of PercentTotalBandwidthUsedforInterrupt
    pub fn get_percent_total_bandwidth_usedfor_interrupt(&self) -> Option<&u32> {
        self.percent_total_bandwidth_usedfor_interrupt.as_ref()
    }

    /// Sets the value of PercentTotalBandwidthUsedforIso
    pub fn set_percent_total_bandwidth_usedfor_iso(&mut self, value: u32) {
        self.percent_total_bandwidth_usedfor_iso = Some(value);
    }

    /// Gets the value of PercentTotalBandwidthUsedforIso
    pub fn get_percent_total_bandwidth_usedfor_iso(&self) -> Option<&u32> {
        self.percent_total_bandwidth_usedfor_iso.as_ref()
    }

    /// Sets the value of TransferErrorsPerSec
    pub fn set_transfer_errors_per_sec(&mut self, value: u32) {
        self.transfer_errors_per_sec = Some(value);
    }

    /// Gets the value of TransferErrorsPerSec
    pub fn get_transfer_errors_per_sec(&self) -> Option<&u32> {
        self.transfer_errors_per_sec.as_ref()
    }
}

