// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterHardwareInfoSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterHardwareInfoSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "BusNumber")]
    pub bus_number: Option<u32>,

/// 
    #[serde(rename = "DeviceNumber")]
    pub device_number: Option<u32>,

/// 
    #[serde(rename = "Dma64BitSupported")]
    pub dma64_bit_supported: Option<bool>,

/// 
    #[serde(rename = "FunctionNumber")]
    pub function_number: Option<u32>,

/// 
    #[serde(rename = "LineBasedInterrupts")]
    pub line_based_interrupts: Option<bool>,

/// 
    #[serde(rename = "LineBasedInterruptSupported")]
    pub line_based_interrupt_supported: Option<bool>,

/// 
    #[serde(rename = "LocationInformationString")]
    pub location_information_string: Option<String>,

/// 
    #[serde(rename = "MaxInterruptMessages")]
    pub max_interrupt_messages: Option<u32>,

/// 
    #[serde(rename = "MsiEnabled")]
    pub msi_enabled: Option<bool>,

/// 
    #[serde(rename = "MsiInterruptSupported")]
    pub msi_interrupt_supported: Option<bool>,

/// 
    #[serde(rename = "MsiSupported")]
    pub msi_supported: Option<bool>,

/// 
    #[serde(rename = "MsiXEnabled")]
    pub msi_xenabled: Option<bool>,

/// 
    #[serde(rename = "MsiXInterruptSupported")]
    pub msi_xinterrupt_supported: Option<bool>,

/// 
    #[serde(rename = "MsixMessageAffinityArray")]
    pub msix_message_affinity_array: Vec<MSFT_NetAdapter_Group_Affinity>,

/// 
    #[serde(rename = "MsiXSupported")]
    pub msi_xsupported: Option<bool>,

/// 
    #[serde(rename = "NoInterrupt")]
    pub no_interrupt: Option<bool>,

/// 
    #[serde(rename = "NumaNode")]
    pub numa_node: Option<u16>,

/// 
    #[serde(rename = "NumMsiMessages")]
    pub num_msi_messages: Option<u32>,

/// 
    #[serde(rename = "NumMsixTableEntries")]
    pub num_msix_table_entries: Option<u32>,

/// 
    #[serde(rename = "PciCurrentSpeedAndMode")]
    pub pci_current_speed_and_mode: Option<u32>,

/// 
    #[serde(rename = "PciDeviceLabelID")]
    pub pci_device_label_id: Option<u32>,

/// 
    #[serde(rename = "PciDeviceLabelString")]
    pub pci_device_label_string: Option<String>,

/// 
    #[serde(rename = "PciDeviceType")]
    pub pci_device_type: Option<u32>,

/// 
    #[serde(rename = "PciExpressCurrentLinkSpeedEncoded")]
    pub pci_express_current_link_speed_encoded: Option<u32>,

/// 
    #[serde(rename = "PciExpressCurrentLinkWidth")]
    pub pci_express_current_link_width: Option<u32>,

/// 
    #[serde(rename = "PciExpressCurrentPayloadSize")]
    pub pci_express_current_payload_size: Option<u32>,

/// 
    #[serde(rename = "PciExpressMaxLinkSpeedEncoded")]
    pub pci_express_max_link_speed_encoded: Option<u32>,

/// 
    #[serde(rename = "PciExpressMaxLinkWidth")]
    pub pci_express_max_link_width: Option<u32>,

/// 
    #[serde(rename = "PciExpressMaxPayloadSize")]
    pub pci_express_max_payload_size: Option<u32>,

/// 
    #[serde(rename = "PciExpressMaxReadRequestSize")]
    pub pci_express_max_read_request_size: Option<u32>,

/// 
    #[serde(rename = "PciExpressVersion")]
    pub pci_express_version: Option<u32>,

/// 
    #[serde(rename = "PciXCurrentSpeedAndMode")]
    pub pci_xcurrent_speed_and_mode: Option<u32>,

/// 
    #[serde(rename = "S0WakeupSupported")]
    pub s0_wakeup_supported: Option<bool>,

/// 
    #[serde(rename = "SegmentNumber")]
    pub segment_number: Option<u32>,

/// 
    #[serde(rename = "SlotNumber")]
    pub slot_number: Option<u32>,

/// 
    #[serde(rename = "SriovSupport")]
    pub sriov_support: Option<u32>,
}

impl MSFT_NetAdapterHardwareInfoSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            bus_number: None,
            device_number: None,
            dma64_bit_supported: None,
            function_number: None,
            line_based_interrupts: None,
            line_based_interrupt_supported: None,
            location_information_string: None,
            max_interrupt_messages: None,
            msi_enabled: None,
            msi_interrupt_supported: None,
            msi_supported: None,
            msi_xenabled: None,
            msi_xinterrupt_supported: None,
            msix_message_affinity_array: Vec::new(),
            msi_xsupported: None,
            no_interrupt: None,
            numa_node: None,
            num_msi_messages: None,
            num_msix_table_entries: None,
            pci_current_speed_and_mode: None,
            pci_device_label_id: None,
            pci_device_label_string: None,
            pci_device_type: None,
            pci_express_current_link_speed_encoded: None,
            pci_express_current_link_width: None,
            pci_express_current_payload_size: None,
            pci_express_max_link_speed_encoded: None,
            pci_express_max_link_width: None,
            pci_express_max_payload_size: None,
            pci_express_max_read_request_size: None,
            pci_express_version: None,
            pci_xcurrent_speed_and_mode: None,
            s0_wakeup_supported: None,
            segment_number: None,
            slot_number: None,
            sriov_support: None,
        }
    }


    /// Sets the value of BusNumber
    pub fn set_bus_number(&mut self, value: u32) {
        self.bus_number = Some(value);
    }

    /// Gets the value of BusNumber
    pub fn get_bus_number(&self) -> Option<&u32> {
        self.bus_number.as_ref()
    }

    /// Sets the value of DeviceNumber
    pub fn set_device_number(&mut self, value: u32) {
        self.device_number = Some(value);
    }

    /// Gets the value of DeviceNumber
    pub fn get_device_number(&self) -> Option<&u32> {
        self.device_number.as_ref()
    }

    /// Sets the value of Dma64BitSupported
    pub fn set_dma64_bit_supported(&mut self, value: bool) {
        self.dma64_bit_supported = Some(value);
    }

    /// Gets the value of Dma64BitSupported
    pub fn get_dma64_bit_supported(&self) -> Option<&bool> {
        self.dma64_bit_supported.as_ref()
    }

    /// Sets the value of FunctionNumber
    pub fn set_function_number(&mut self, value: u32) {
        self.function_number = Some(value);
    }

    /// Gets the value of FunctionNumber
    pub fn get_function_number(&self) -> Option<&u32> {
        self.function_number.as_ref()
    }

    /// Sets the value of LineBasedInterrupts
    pub fn set_line_based_interrupts(&mut self, value: bool) {
        self.line_based_interrupts = Some(value);
    }

    /// Gets the value of LineBasedInterrupts
    pub fn get_line_based_interrupts(&self) -> Option<&bool> {
        self.line_based_interrupts.as_ref()
    }

    /// Sets the value of LineBasedInterruptSupported
    pub fn set_line_based_interrupt_supported(&mut self, value: bool) {
        self.line_based_interrupt_supported = Some(value);
    }

    /// Gets the value of LineBasedInterruptSupported
    pub fn get_line_based_interrupt_supported(&self) -> Option<&bool> {
        self.line_based_interrupt_supported.as_ref()
    }

    /// Sets the value of LocationInformationString
    pub fn set_location_information_string(&mut self, value: String) {
        self.location_information_string = Some(value);
    }

    /// Gets the value of LocationInformationString
    pub fn get_location_information_string(&self) -> Option<&String> {
        self.location_information_string.as_ref()
    }

    /// Sets the value of MaxInterruptMessages
    pub fn set_max_interrupt_messages(&mut self, value: u32) {
        self.max_interrupt_messages = Some(value);
    }

    /// Gets the value of MaxInterruptMessages
    pub fn get_max_interrupt_messages(&self) -> Option<&u32> {
        self.max_interrupt_messages.as_ref()
    }

    /// Sets the value of MsiEnabled
    pub fn set_msi_enabled(&mut self, value: bool) {
        self.msi_enabled = Some(value);
    }

    /// Gets the value of MsiEnabled
    pub fn get_msi_enabled(&self) -> Option<&bool> {
        self.msi_enabled.as_ref()
    }

    /// Sets the value of MsiInterruptSupported
    pub fn set_msi_interrupt_supported(&mut self, value: bool) {
        self.msi_interrupt_supported = Some(value);
    }

    /// Gets the value of MsiInterruptSupported
    pub fn get_msi_interrupt_supported(&self) -> Option<&bool> {
        self.msi_interrupt_supported.as_ref()
    }

    /// Sets the value of MsiSupported
    pub fn set_msi_supported(&mut self, value: bool) {
        self.msi_supported = Some(value);
    }

    /// Gets the value of MsiSupported
    pub fn get_msi_supported(&self) -> Option<&bool> {
        self.msi_supported.as_ref()
    }

    /// Sets the value of MsiXEnabled
    pub fn set_msi_xenabled(&mut self, value: bool) {
        self.msi_xenabled = Some(value);
    }

    /// Gets the value of MsiXEnabled
    pub fn get_msi_xenabled(&self) -> Option<&bool> {
        self.msi_xenabled.as_ref()
    }

    /// Sets the value of MsiXInterruptSupported
    pub fn set_msi_xinterrupt_supported(&mut self, value: bool) {
        self.msi_xinterrupt_supported = Some(value);
    }

    /// Gets the value of MsiXInterruptSupported
    pub fn get_msi_xinterrupt_supported(&self) -> Option<&bool> {
        self.msi_xinterrupt_supported.as_ref()
    }

    /// Sets the value of MsixMessageAffinityArray
    pub fn set_msix_message_affinity_array(&mut self, value: Vec<MSFT_NetAdapter_Group_Affinity>) {
        self.msix_message_affinity_array = value;
    }

    /// Gets the value of MsixMessageAffinityArray
    pub fn get_msix_message_affinity_array(&self) -> &Vec<MSFT_NetAdapter_Group_Affinity> {
        &self.msix_message_affinity_array
    }

    /// Sets the value of MsiXSupported
    pub fn set_msi_xsupported(&mut self, value: bool) {
        self.msi_xsupported = Some(value);
    }

    /// Gets the value of MsiXSupported
    pub fn get_msi_xsupported(&self) -> Option<&bool> {
        self.msi_xsupported.as_ref()
    }

    /// Sets the value of NoInterrupt
    pub fn set_no_interrupt(&mut self, value: bool) {
        self.no_interrupt = Some(value);
    }

    /// Gets the value of NoInterrupt
    pub fn get_no_interrupt(&self) -> Option<&bool> {
        self.no_interrupt.as_ref()
    }

    /// Sets the value of NumaNode
    pub fn set_numa_node(&mut self, value: u16) {
        self.numa_node = Some(value);
    }

    /// Gets the value of NumaNode
    pub fn get_numa_node(&self) -> Option<&u16> {
        self.numa_node.as_ref()
    }

    /// Sets the value of NumMsiMessages
    pub fn set_num_msi_messages(&mut self, value: u32) {
        self.num_msi_messages = Some(value);
    }

    /// Gets the value of NumMsiMessages
    pub fn get_num_msi_messages(&self) -> Option<&u32> {
        self.num_msi_messages.as_ref()
    }

    /// Sets the value of NumMsixTableEntries
    pub fn set_num_msix_table_entries(&mut self, value: u32) {
        self.num_msix_table_entries = Some(value);
    }

    /// Gets the value of NumMsixTableEntries
    pub fn get_num_msix_table_entries(&self) -> Option<&u32> {
        self.num_msix_table_entries.as_ref()
    }

    /// Sets the value of PciCurrentSpeedAndMode
    pub fn set_pci_current_speed_and_mode(&mut self, value: u32) {
        self.pci_current_speed_and_mode = Some(value);
    }

    /// Gets the value of PciCurrentSpeedAndMode
    pub fn get_pci_current_speed_and_mode(&self) -> Option<&u32> {
        self.pci_current_speed_and_mode.as_ref()
    }

    /// Sets the value of PciDeviceLabelID
    pub fn set_pci_device_label_id(&mut self, value: u32) {
        self.pci_device_label_id = Some(value);
    }

    /// Gets the value of PciDeviceLabelID
    pub fn get_pci_device_label_id(&self) -> Option<&u32> {
        self.pci_device_label_id.as_ref()
    }

    /// Sets the value of PciDeviceLabelString
    pub fn set_pci_device_label_string(&mut self, value: String) {
        self.pci_device_label_string = Some(value);
    }

    /// Gets the value of PciDeviceLabelString
    pub fn get_pci_device_label_string(&self) -> Option<&String> {
        self.pci_device_label_string.as_ref()
    }

    /// Sets the value of PciDeviceType
    pub fn set_pci_device_type(&mut self, value: u32) {
        self.pci_device_type = Some(value);
    }

    /// Gets the value of PciDeviceType
    pub fn get_pci_device_type(&self) -> Option<&u32> {
        self.pci_device_type.as_ref()
    }

    /// Sets the value of PciExpressCurrentLinkSpeedEncoded
    pub fn set_pci_express_current_link_speed_encoded(&mut self, value: u32) {
        self.pci_express_current_link_speed_encoded = Some(value);
    }

    /// Gets the value of PciExpressCurrentLinkSpeedEncoded
    pub fn get_pci_express_current_link_speed_encoded(&self) -> Option<&u32> {
        self.pci_express_current_link_speed_encoded.as_ref()
    }

    /// Sets the value of PciExpressCurrentLinkWidth
    pub fn set_pci_express_current_link_width(&mut self, value: u32) {
        self.pci_express_current_link_width = Some(value);
    }

    /// Gets the value of PciExpressCurrentLinkWidth
    pub fn get_pci_express_current_link_width(&self) -> Option<&u32> {
        self.pci_express_current_link_width.as_ref()
    }

    /// Sets the value of PciExpressCurrentPayloadSize
    pub fn set_pci_express_current_payload_size(&mut self, value: u32) {
        self.pci_express_current_payload_size = Some(value);
    }

    /// Gets the value of PciExpressCurrentPayloadSize
    pub fn get_pci_express_current_payload_size(&self) -> Option<&u32> {
        self.pci_express_current_payload_size.as_ref()
    }

    /// Sets the value of PciExpressMaxLinkSpeedEncoded
    pub fn set_pci_express_max_link_speed_encoded(&mut self, value: u32) {
        self.pci_express_max_link_speed_encoded = Some(value);
    }

    /// Gets the value of PciExpressMaxLinkSpeedEncoded
    pub fn get_pci_express_max_link_speed_encoded(&self) -> Option<&u32> {
        self.pci_express_max_link_speed_encoded.as_ref()
    }

    /// Sets the value of PciExpressMaxLinkWidth
    pub fn set_pci_express_max_link_width(&mut self, value: u32) {
        self.pci_express_max_link_width = Some(value);
    }

    /// Gets the value of PciExpressMaxLinkWidth
    pub fn get_pci_express_max_link_width(&self) -> Option<&u32> {
        self.pci_express_max_link_width.as_ref()
    }

    /// Sets the value of PciExpressMaxPayloadSize
    pub fn set_pci_express_max_payload_size(&mut self, value: u32) {
        self.pci_express_max_payload_size = Some(value);
    }

    /// Gets the value of PciExpressMaxPayloadSize
    pub fn get_pci_express_max_payload_size(&self) -> Option<&u32> {
        self.pci_express_max_payload_size.as_ref()
    }

    /// Sets the value of PciExpressMaxReadRequestSize
    pub fn set_pci_express_max_read_request_size(&mut self, value: u32) {
        self.pci_express_max_read_request_size = Some(value);
    }

    /// Gets the value of PciExpressMaxReadRequestSize
    pub fn get_pci_express_max_read_request_size(&self) -> Option<&u32> {
        self.pci_express_max_read_request_size.as_ref()
    }

    /// Sets the value of PciExpressVersion
    pub fn set_pci_express_version(&mut self, value: u32) {
        self.pci_express_version = Some(value);
    }

    /// Gets the value of PciExpressVersion
    pub fn get_pci_express_version(&self) -> Option<&u32> {
        self.pci_express_version.as_ref()
    }

    /// Sets the value of PciXCurrentSpeedAndMode
    pub fn set_pci_xcurrent_speed_and_mode(&mut self, value: u32) {
        self.pci_xcurrent_speed_and_mode = Some(value);
    }

    /// Gets the value of PciXCurrentSpeedAndMode
    pub fn get_pci_xcurrent_speed_and_mode(&self) -> Option<&u32> {
        self.pci_xcurrent_speed_and_mode.as_ref()
    }

    /// Sets the value of S0WakeupSupported
    pub fn set_s0_wakeup_supported(&mut self, value: bool) {
        self.s0_wakeup_supported = Some(value);
    }

    /// Gets the value of S0WakeupSupported
    pub fn get_s0_wakeup_supported(&self) -> Option<&bool> {
        self.s0_wakeup_supported.as_ref()
    }

    /// Sets the value of SegmentNumber
    pub fn set_segment_number(&mut self, value: u32) {
        self.segment_number = Some(value);
    }

    /// Gets the value of SegmentNumber
    pub fn get_segment_number(&self) -> Option<&u32> {
        self.segment_number.as_ref()
    }

    /// Sets the value of SlotNumber
    pub fn set_slot_number(&mut self, value: u32) {
        self.slot_number = Some(value);
    }

    /// Gets the value of SlotNumber
    pub fn get_slot_number(&self) -> Option<&u32> {
        self.slot_number.as_ref()
    }

    /// Sets the value of SriovSupport
    pub fn set_sriov_support(&mut self, value: u32) {
        self.sriov_support = Some(value);
    }

    /// Gets the value of SriovSupport
    pub fn get_sriov_support(&self) -> Option<&u32> {
        self.sriov_support.as_ref()
    }
}

