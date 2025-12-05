// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_BluetoothRadio struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_BluetoothRadio {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "A2dpOffloadConnections")]
    pub a2dp_offload_connections: Option<u32>,

/// 
    #[serde(rename = "ACLflusheventsPersec")]
    pub aclflushevents_persec: Option<u32>,

/// 
    #[serde(rename = "ActiveLEAdvertisementMonitorscount")]
    pub active_leadvertisement_monitorscount: Option<u32>,

/// 
    #[serde(rename = "ClassicACLbytesreadPersec")]
    pub classic_aclbytesread_persec: Option<u32>,

/// 
    #[serde(rename = "ClassicACLbyteswrittenPersec")]
    pub classic_aclbyteswritten_persec: Option<u32>,

/// 
    #[serde(rename = "ClassicACLConnections")]
    pub classic_aclconnections: Option<u32>,

/// 
    #[serde(rename = "ClassicACLwritecredits")]
    pub classic_aclwritecredits: Option<u32>,

/// 
    #[serde(rename = "CreatedLEBIGCount")]
    pub created_lebigcount: Option<u32>,

/// 
    #[serde(rename = "EstablishedBIGSyncCount")]
    pub established_bigsync_count: Option<u32>,

/// 
    #[serde(rename = "EstablishedLECISCount")]
    pub established_leciscount: Option<u32>,

/// 
    #[serde(rename = "EstablishedLEPeriodicAdvertisingSyncCount")]
    pub established_leperiodic_advertising_sync_count: Option<u32>,

/// 
    #[serde(rename = "InquiryScanDutyCyclePercent")]
    pub inquiry_scan_duty_cycle_percent: Option<u32>,

/// 
    #[serde(rename = "InquiryScanInterval")]
    pub inquiry_scan_interval: Option<u32>,

/// 
    #[serde(rename = "InquiryScanWindow")]
    pub inquiry_scan_window: Option<u32>,

/// 
    #[serde(rename = "LEACLbytesreadPersec")]
    pub leaclbytesread_persec: Option<u32>,

/// 
    #[serde(rename = "LEACLbyteswrittenPersec")]
    pub leaclbyteswritten_persec: Option<u32>,

/// 
    #[serde(rename = "LEACLConnections")]
    pub leaclconnections: Option<u32>,

/// 
    #[serde(rename = "LEACLwritecredits")]
    pub leaclwritecredits: Option<u32>,

/// 
    #[serde(rename = "LEAdvertisingHandlesAllocated")]
    pub leadvertising_handles_allocated: Option<u32>,

/// 
    #[serde(rename = "LEAdvertisingSetsEnabled")]
    pub leadvertising_sets_enabled: Option<u32>,

/// 
    #[serde(rename = "LEBIGInfoReporteventsreceivedPersec")]
    pub lebiginfo_reporteventsreceived_persec: Option<u32>,

/// 
    #[serde(rename = "LEPeriodicAdvertisingReportbytesreceivedPersec")]
    pub leperiodic_advertising_reportbytesreceived_persec: Option<u32>,

/// 
    #[serde(rename = "LEPeriodicAdvertisingReporteventsreceivedPersec")]
    pub leperiodic_advertising_reporteventsreceived_persec: Option<u32>,

/// 
    #[serde(rename = "LEScanDutyCyclePercentCodedPhy")]
    pub lescan_duty_cycle_percent_coded_phy: Option<u32>,

/// 
    #[serde(rename = "LEScanDutyCyclePercentUncoded1MPhy")]
    pub lescan_duty_cycle_percent_uncoded1_mphy: Option<u32>,

/// 
    #[serde(rename = "LEScanIntervalCodedPhy")]
    pub lescan_interval_coded_phy: Option<u32>,

/// 
    #[serde(rename = "LEScanIntervalUncoded1MPhy")]
    pub lescan_interval_uncoded1_mphy: Option<u32>,

/// 
    #[serde(rename = "LEScanWindowCodedPhy")]
    pub lescan_window_coded_phy: Option<u32>,

/// 
    #[serde(rename = "LEScanWindowUncoded1MPhy")]
    pub lescan_window_uncoded1_mphy: Option<u32>,

/// 
    #[serde(rename = "PageScanDutyCyclePercent")]
    pub page_scan_duty_cycle_percent: Option<u32>,

/// 
    #[serde(rename = "PageScanInterval")]
    pub page_scan_interval: Option<u32>,

/// 
    #[serde(rename = "PageScanWindow")]
    pub page_scan_window: Option<u32>,

/// 
    #[serde(rename = "ReportedLEMonitoredDevices")]
    pub reported_lemonitored_devices: Option<u32>,

/// 
    #[serde(rename = "SCObytesreadPersec")]
    pub scobytesread_persec: Option<u32>,

/// 
    #[serde(rename = "SCObyteswrittenPersec")]
    pub scobyteswritten_persec: Option<u32>,

/// 
    #[serde(rename = "SCOConnections")]
    pub scoconnections: Option<u32>,

/// 
    #[serde(rename = "SidebandSCOConnections")]
    pub sideband_scoconnections: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_BluetoothRadio {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            a2dp_offload_connections: None,
            aclflushevents_persec: None,
            active_leadvertisement_monitorscount: None,
            classic_aclbytesread_persec: None,
            classic_aclbyteswritten_persec: None,
            classic_aclconnections: None,
            classic_aclwritecredits: None,
            created_lebigcount: None,
            established_bigsync_count: None,
            established_leciscount: None,
            established_leperiodic_advertising_sync_count: None,
            inquiry_scan_duty_cycle_percent: None,
            inquiry_scan_interval: None,
            inquiry_scan_window: None,
            leaclbytesread_persec: None,
            leaclbyteswritten_persec: None,
            leaclconnections: None,
            leaclwritecredits: None,
            leadvertising_handles_allocated: None,
            leadvertising_sets_enabled: None,
            lebiginfo_reporteventsreceived_persec: None,
            leperiodic_advertising_reportbytesreceived_persec: None,
            leperiodic_advertising_reporteventsreceived_persec: None,
            lescan_duty_cycle_percent_coded_phy: None,
            lescan_duty_cycle_percent_uncoded1_mphy: None,
            lescan_interval_coded_phy: None,
            lescan_interval_uncoded1_mphy: None,
            lescan_window_coded_phy: None,
            lescan_window_uncoded1_mphy: None,
            page_scan_duty_cycle_percent: None,
            page_scan_interval: None,
            page_scan_window: None,
            reported_lemonitored_devices: None,
            scobytesread_persec: None,
            scobyteswritten_persec: None,
            scoconnections: None,
            sideband_scoconnections: None,
        }
    }


    /// Sets the value of A2dpOffloadConnections
    pub fn set_a2dp_offload_connections(&mut self, value: u32) {
        self.a2dp_offload_connections = Some(value);
    }

    /// Gets the value of A2dpOffloadConnections
    pub fn get_a2dp_offload_connections(&self) -> Option<&u32> {
        self.a2dp_offload_connections.as_ref()
    }

    /// Sets the value of ACLflusheventsPersec
    pub fn set_aclflushevents_persec(&mut self, value: u32) {
        self.aclflushevents_persec = Some(value);
    }

    /// Gets the value of ACLflusheventsPersec
    pub fn get_aclflushevents_persec(&self) -> Option<&u32> {
        self.aclflushevents_persec.as_ref()
    }

    /// Sets the value of ActiveLEAdvertisementMonitorscount
    pub fn set_active_leadvertisement_monitorscount(&mut self, value: u32) {
        self.active_leadvertisement_monitorscount = Some(value);
    }

    /// Gets the value of ActiveLEAdvertisementMonitorscount
    pub fn get_active_leadvertisement_monitorscount(&self) -> Option<&u32> {
        self.active_leadvertisement_monitorscount.as_ref()
    }

    /// Sets the value of ClassicACLbytesreadPersec
    pub fn set_classic_aclbytesread_persec(&mut self, value: u32) {
        self.classic_aclbytesread_persec = Some(value);
    }

    /// Gets the value of ClassicACLbytesreadPersec
    pub fn get_classic_aclbytesread_persec(&self) -> Option<&u32> {
        self.classic_aclbytesread_persec.as_ref()
    }

    /// Sets the value of ClassicACLbyteswrittenPersec
    pub fn set_classic_aclbyteswritten_persec(&mut self, value: u32) {
        self.classic_aclbyteswritten_persec = Some(value);
    }

    /// Gets the value of ClassicACLbyteswrittenPersec
    pub fn get_classic_aclbyteswritten_persec(&self) -> Option<&u32> {
        self.classic_aclbyteswritten_persec.as_ref()
    }

    /// Sets the value of ClassicACLConnections
    pub fn set_classic_aclconnections(&mut self, value: u32) {
        self.classic_aclconnections = Some(value);
    }

    /// Gets the value of ClassicACLConnections
    pub fn get_classic_aclconnections(&self) -> Option<&u32> {
        self.classic_aclconnections.as_ref()
    }

    /// Sets the value of ClassicACLwritecredits
    pub fn set_classic_aclwritecredits(&mut self, value: u32) {
        self.classic_aclwritecredits = Some(value);
    }

    /// Gets the value of ClassicACLwritecredits
    pub fn get_classic_aclwritecredits(&self) -> Option<&u32> {
        self.classic_aclwritecredits.as_ref()
    }

    /// Sets the value of CreatedLEBIGCount
    pub fn set_created_lebigcount(&mut self, value: u32) {
        self.created_lebigcount = Some(value);
    }

    /// Gets the value of CreatedLEBIGCount
    pub fn get_created_lebigcount(&self) -> Option<&u32> {
        self.created_lebigcount.as_ref()
    }

    /// Sets the value of EstablishedBIGSyncCount
    pub fn set_established_bigsync_count(&mut self, value: u32) {
        self.established_bigsync_count = Some(value);
    }

    /// Gets the value of EstablishedBIGSyncCount
    pub fn get_established_bigsync_count(&self) -> Option<&u32> {
        self.established_bigsync_count.as_ref()
    }

    /// Sets the value of EstablishedLECISCount
    pub fn set_established_leciscount(&mut self, value: u32) {
        self.established_leciscount = Some(value);
    }

    /// Gets the value of EstablishedLECISCount
    pub fn get_established_leciscount(&self) -> Option<&u32> {
        self.established_leciscount.as_ref()
    }

    /// Sets the value of EstablishedLEPeriodicAdvertisingSyncCount
    pub fn set_established_leperiodic_advertising_sync_count(&mut self, value: u32) {
        self.established_leperiodic_advertising_sync_count = Some(value);
    }

    /// Gets the value of EstablishedLEPeriodicAdvertisingSyncCount
    pub fn get_established_leperiodic_advertising_sync_count(&self) -> Option<&u32> {
        self.established_leperiodic_advertising_sync_count.as_ref()
    }

    /// Sets the value of InquiryScanDutyCyclePercent
    pub fn set_inquiry_scan_duty_cycle_percent(&mut self, value: u32) {
        self.inquiry_scan_duty_cycle_percent = Some(value);
    }

    /// Gets the value of InquiryScanDutyCyclePercent
    pub fn get_inquiry_scan_duty_cycle_percent(&self) -> Option<&u32> {
        self.inquiry_scan_duty_cycle_percent.as_ref()
    }

    /// Sets the value of InquiryScanInterval
    pub fn set_inquiry_scan_interval(&mut self, value: u32) {
        self.inquiry_scan_interval = Some(value);
    }

    /// Gets the value of InquiryScanInterval
    pub fn get_inquiry_scan_interval(&self) -> Option<&u32> {
        self.inquiry_scan_interval.as_ref()
    }

    /// Sets the value of InquiryScanWindow
    pub fn set_inquiry_scan_window(&mut self, value: u32) {
        self.inquiry_scan_window = Some(value);
    }

    /// Gets the value of InquiryScanWindow
    pub fn get_inquiry_scan_window(&self) -> Option<&u32> {
        self.inquiry_scan_window.as_ref()
    }

    /// Sets the value of LEACLbytesreadPersec
    pub fn set_leaclbytesread_persec(&mut self, value: u32) {
        self.leaclbytesread_persec = Some(value);
    }

    /// Gets the value of LEACLbytesreadPersec
    pub fn get_leaclbytesread_persec(&self) -> Option<&u32> {
        self.leaclbytesread_persec.as_ref()
    }

    /// Sets the value of LEACLbyteswrittenPersec
    pub fn set_leaclbyteswritten_persec(&mut self, value: u32) {
        self.leaclbyteswritten_persec = Some(value);
    }

    /// Gets the value of LEACLbyteswrittenPersec
    pub fn get_leaclbyteswritten_persec(&self) -> Option<&u32> {
        self.leaclbyteswritten_persec.as_ref()
    }

    /// Sets the value of LEACLConnections
    pub fn set_leaclconnections(&mut self, value: u32) {
        self.leaclconnections = Some(value);
    }

    /// Gets the value of LEACLConnections
    pub fn get_leaclconnections(&self) -> Option<&u32> {
        self.leaclconnections.as_ref()
    }

    /// Sets the value of LEACLwritecredits
    pub fn set_leaclwritecredits(&mut self, value: u32) {
        self.leaclwritecredits = Some(value);
    }

    /// Gets the value of LEACLwritecredits
    pub fn get_leaclwritecredits(&self) -> Option<&u32> {
        self.leaclwritecredits.as_ref()
    }

    /// Sets the value of LEAdvertisingHandlesAllocated
    pub fn set_leadvertising_handles_allocated(&mut self, value: u32) {
        self.leadvertising_handles_allocated = Some(value);
    }

    /// Gets the value of LEAdvertisingHandlesAllocated
    pub fn get_leadvertising_handles_allocated(&self) -> Option<&u32> {
        self.leadvertising_handles_allocated.as_ref()
    }

    /// Sets the value of LEAdvertisingSetsEnabled
    pub fn set_leadvertising_sets_enabled(&mut self, value: u32) {
        self.leadvertising_sets_enabled = Some(value);
    }

    /// Gets the value of LEAdvertisingSetsEnabled
    pub fn get_leadvertising_sets_enabled(&self) -> Option<&u32> {
        self.leadvertising_sets_enabled.as_ref()
    }

    /// Sets the value of LEBIGInfoReporteventsreceivedPersec
    pub fn set_lebiginfo_reporteventsreceived_persec(&mut self, value: u32) {
        self.lebiginfo_reporteventsreceived_persec = Some(value);
    }

    /// Gets the value of LEBIGInfoReporteventsreceivedPersec
    pub fn get_lebiginfo_reporteventsreceived_persec(&self) -> Option<&u32> {
        self.lebiginfo_reporteventsreceived_persec.as_ref()
    }

    /// Sets the value of LEPeriodicAdvertisingReportbytesreceivedPersec
    pub fn set_leperiodic_advertising_reportbytesreceived_persec(&mut self, value: u32) {
        self.leperiodic_advertising_reportbytesreceived_persec = Some(value);
    }

    /// Gets the value of LEPeriodicAdvertisingReportbytesreceivedPersec
    pub fn get_leperiodic_advertising_reportbytesreceived_persec(&self) -> Option<&u32> {
        self.leperiodic_advertising_reportbytesreceived_persec.as_ref()
    }

    /// Sets the value of LEPeriodicAdvertisingReporteventsreceivedPersec
    pub fn set_leperiodic_advertising_reporteventsreceived_persec(&mut self, value: u32) {
        self.leperiodic_advertising_reporteventsreceived_persec = Some(value);
    }

    /// Gets the value of LEPeriodicAdvertisingReporteventsreceivedPersec
    pub fn get_leperiodic_advertising_reporteventsreceived_persec(&self) -> Option<&u32> {
        self.leperiodic_advertising_reporteventsreceived_persec.as_ref()
    }

    /// Sets the value of LEScanDutyCyclePercentCodedPhy
    pub fn set_lescan_duty_cycle_percent_coded_phy(&mut self, value: u32) {
        self.lescan_duty_cycle_percent_coded_phy = Some(value);
    }

    /// Gets the value of LEScanDutyCyclePercentCodedPhy
    pub fn get_lescan_duty_cycle_percent_coded_phy(&self) -> Option<&u32> {
        self.lescan_duty_cycle_percent_coded_phy.as_ref()
    }

    /// Sets the value of LEScanDutyCyclePercentUncoded1MPhy
    pub fn set_lescan_duty_cycle_percent_uncoded1_mphy(&mut self, value: u32) {
        self.lescan_duty_cycle_percent_uncoded1_mphy = Some(value);
    }

    /// Gets the value of LEScanDutyCyclePercentUncoded1MPhy
    pub fn get_lescan_duty_cycle_percent_uncoded1_mphy(&self) -> Option<&u32> {
        self.lescan_duty_cycle_percent_uncoded1_mphy.as_ref()
    }

    /// Sets the value of LEScanIntervalCodedPhy
    pub fn set_lescan_interval_coded_phy(&mut self, value: u32) {
        self.lescan_interval_coded_phy = Some(value);
    }

    /// Gets the value of LEScanIntervalCodedPhy
    pub fn get_lescan_interval_coded_phy(&self) -> Option<&u32> {
        self.lescan_interval_coded_phy.as_ref()
    }

    /// Sets the value of LEScanIntervalUncoded1MPhy
    pub fn set_lescan_interval_uncoded1_mphy(&mut self, value: u32) {
        self.lescan_interval_uncoded1_mphy = Some(value);
    }

    /// Gets the value of LEScanIntervalUncoded1MPhy
    pub fn get_lescan_interval_uncoded1_mphy(&self) -> Option<&u32> {
        self.lescan_interval_uncoded1_mphy.as_ref()
    }

    /// Sets the value of LEScanWindowCodedPhy
    pub fn set_lescan_window_coded_phy(&mut self, value: u32) {
        self.lescan_window_coded_phy = Some(value);
    }

    /// Gets the value of LEScanWindowCodedPhy
    pub fn get_lescan_window_coded_phy(&self) -> Option<&u32> {
        self.lescan_window_coded_phy.as_ref()
    }

    /// Sets the value of LEScanWindowUncoded1MPhy
    pub fn set_lescan_window_uncoded1_mphy(&mut self, value: u32) {
        self.lescan_window_uncoded1_mphy = Some(value);
    }

    /// Gets the value of LEScanWindowUncoded1MPhy
    pub fn get_lescan_window_uncoded1_mphy(&self) -> Option<&u32> {
        self.lescan_window_uncoded1_mphy.as_ref()
    }

    /// Sets the value of PageScanDutyCyclePercent
    pub fn set_page_scan_duty_cycle_percent(&mut self, value: u32) {
        self.page_scan_duty_cycle_percent = Some(value);
    }

    /// Gets the value of PageScanDutyCyclePercent
    pub fn get_page_scan_duty_cycle_percent(&self) -> Option<&u32> {
        self.page_scan_duty_cycle_percent.as_ref()
    }

    /// Sets the value of PageScanInterval
    pub fn set_page_scan_interval(&mut self, value: u32) {
        self.page_scan_interval = Some(value);
    }

    /// Gets the value of PageScanInterval
    pub fn get_page_scan_interval(&self) -> Option<&u32> {
        self.page_scan_interval.as_ref()
    }

    /// Sets the value of PageScanWindow
    pub fn set_page_scan_window(&mut self, value: u32) {
        self.page_scan_window = Some(value);
    }

    /// Gets the value of PageScanWindow
    pub fn get_page_scan_window(&self) -> Option<&u32> {
        self.page_scan_window.as_ref()
    }

    /// Sets the value of ReportedLEMonitoredDevices
    pub fn set_reported_lemonitored_devices(&mut self, value: u32) {
        self.reported_lemonitored_devices = Some(value);
    }

    /// Gets the value of ReportedLEMonitoredDevices
    pub fn get_reported_lemonitored_devices(&self) -> Option<&u32> {
        self.reported_lemonitored_devices.as_ref()
    }

    /// Sets the value of SCObytesreadPersec
    pub fn set_scobytesread_persec(&mut self, value: u32) {
        self.scobytesread_persec = Some(value);
    }

    /// Gets the value of SCObytesreadPersec
    pub fn get_scobytesread_persec(&self) -> Option<&u32> {
        self.scobytesread_persec.as_ref()
    }

    /// Sets the value of SCObyteswrittenPersec
    pub fn set_scobyteswritten_persec(&mut self, value: u32) {
        self.scobyteswritten_persec = Some(value);
    }

    /// Gets the value of SCObyteswrittenPersec
    pub fn get_scobyteswritten_persec(&self) -> Option<&u32> {
        self.scobyteswritten_persec.as_ref()
    }

    /// Sets the value of SCOConnections
    pub fn set_scoconnections(&mut self, value: u32) {
        self.scoconnections = Some(value);
    }

    /// Gets the value of SCOConnections
    pub fn get_scoconnections(&self) -> Option<&u32> {
        self.scoconnections.as_ref()
    }

    /// Sets the value of SidebandSCOConnections
    pub fn set_sideband_scoconnections(&mut self, value: u32) {
        self.sideband_scoconnections = Some(value);
    }

    /// Gets the value of SidebandSCOConnections
    pub fn get_sideband_scoconnections(&self) -> Option<&u32> {
        self.sideband_scoconnections.as_ref()
    }
}

