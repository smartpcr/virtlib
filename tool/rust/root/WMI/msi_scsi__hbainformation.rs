// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_HBAInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_HBAInformation {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// A text string set by the manufacturer describing the Asic version
    #[serde(rename = "AsicVersion")]
    pub asic_version: Option<String>,

/// TRUE if Bi-directionsal SCSI comamnd supported
    #[serde(rename = "BiDiScsiCommands")]
    pub bi_di_scsi_commands: Option<bool>,

/// TRUE if the adapter caches are valid
    #[serde(rename = "CacheValid")]
    pub cache_valid: Option<bool>,

/// A text string specifying the name of the driver for the adapter
    #[serde(rename = "DriverName")]
    pub driver_name: Option<String>,

/// A text string set by the manufacturer describing the firmware version of adapter
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// **typedef** Bit flags that indicate various functionality supported
    #[serde(rename = "FunctionalitySupported")]
    pub functionality_supported: Option<u32>,

/// This is the GUID value last set by the SetGenerationalGuid method in the MSiSCSI_Operations class.
    #[serde(rename = "GenerationalGuid")]
    pub generational_guid: Vec<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// TRUE if TCP/IP traffic is integrated with the Windows networking TCP/IP stack via a software only initiator. An adapter with its own TCP/IP stack would set this to FALSE.
    #[serde(rename = "IntegratedTCPIP")]
    pub integrated_tcpip: Option<bool>,

/// Maxumum CDB length supported by the adapter
    #[serde(rename = "MaxCDBLength")]
    pub max_cdblength: Option<u32>,

/// TRUE if this adapter is a multifunction device, that is it also exposes a netcard interface
    #[serde(rename = "MultifunctionDevice")]
    pub multifunction_device: Option<bool>,

/// Number of ports (or TCP/IP addresses) on the adapter
    #[serde(rename = "NumberOfPorts")]
    pub number_of_ports: Option<u32>,

/// A text string set by the manufacturer describing the option rom version of adapter
    #[serde(rename = "OptionRomVersion")]
    pub option_rom_version: Option<String>,

/// If TRUE the iSCSI Initiator service will perform any DNS lookup and pass binary IP addresses to the adapter; the adapter must be on the same network as the Windows TCP/IP stack. If FALSE then DNS must be available on adapter.
    #[serde(rename = "RequiresBinaryIpAddresses")]
    pub requires_binary_ip_addresses: Option<bool>,

/// A text string set by the manufacturer describing the serial number of adapter
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// **typedef** Current status of adapter
    #[serde(rename = "Status")]
    pub status: Option<HBAInformation_Status>,

/// Id that is globally unique for all instances of iSCSI initiators. Use the address of the Adapter Extension or another address owned by the device driver.
    #[serde(rename = "UniqueAdapterId")]
    pub unique_adapter_id: Option<u64>,

/// A text string describing the manufacturer of adapter
    #[serde(rename = "VendorID")]
    pub vendor_id: Option<String>,

/// A text string set by the manufacturer describing the model of adapter
    #[serde(rename = "VendorModel")]
    pub vendor_model: Option<String>,

/// A text string set by the manufacturer describing the version of adapter
    #[serde(rename = "VendorVersion")]
    pub vendor_version: Option<String>,

/// Maximum version number of the iSCSI spec supported by adapter
    #[serde(rename = "VersionMax")]
    pub version_max: Option<u8>,

/// Minimum version number of the iScsi spec supported by adapter
    #[serde(rename = "VersionMin")]
    pub version_min: Option<u8>,
}

impl MSiSCSI_HBAInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            asic_version: None,
            bi_di_scsi_commands: None,
            cache_valid: None,
            driver_name: None,
            firmware_version: None,
            functionality_supported: None,
            generational_guid: Vec::new(),
            instance_name: None,
            integrated_tcpip: None,
            max_cdblength: None,
            multifunction_device: None,
            number_of_ports: None,
            option_rom_version: None,
            requires_binary_ip_addresses: None,
            serial_number: None,
            status: None,
            unique_adapter_id: None,
            vendor_id: None,
            vendor_model: None,
            vendor_version: None,
            version_max: None,
            version_min: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of AsicVersion
    pub fn set_asic_version(&mut self, value: String) {
        self.asic_version = Some(value);
    }

    /// Gets the value of AsicVersion
    pub fn get_asic_version(&self) -> Option<&String> {
        self.asic_version.as_ref()
    }

    /// Sets the value of BiDiScsiCommands
    pub fn set_bi_di_scsi_commands(&mut self, value: bool) {
        self.bi_di_scsi_commands = Some(value);
    }

    /// Gets the value of BiDiScsiCommands
    pub fn get_bi_di_scsi_commands(&self) -> Option<&bool> {
        self.bi_di_scsi_commands.as_ref()
    }

    /// Sets the value of CacheValid
    pub fn set_cache_valid(&mut self, value: bool) {
        self.cache_valid = Some(value);
    }

    /// Gets the value of CacheValid
    pub fn get_cache_valid(&self) -> Option<&bool> {
        self.cache_valid.as_ref()
    }

    /// Sets the value of DriverName
    pub fn set_driver_name(&mut self, value: String) {
        self.driver_name = Some(value);
    }

    /// Gets the value of DriverName
    pub fn get_driver_name(&self) -> Option<&String> {
        self.driver_name.as_ref()
    }

    /// Sets the value of FirmwareVersion
    pub fn set_firmware_version(&mut self, value: String) {
        self.firmware_version = Some(value);
    }

    /// Gets the value of FirmwareVersion
    pub fn get_firmware_version(&self) -> Option<&String> {
        self.firmware_version.as_ref()
    }

    /// Sets the value of FunctionalitySupported
    pub fn set_functionality_supported(&mut self, value: u32) {
        self.functionality_supported = Some(value);
    }

    /// Gets the value of FunctionalitySupported
    pub fn get_functionality_supported(&self) -> Option<&u32> {
        self.functionality_supported.as_ref()
    }

    /// Sets the value of GenerationalGuid
    pub fn set_generational_guid(&mut self, value: Vec<u8>) {
        self.generational_guid = value;
    }

    /// Gets the value of GenerationalGuid
    pub fn get_generational_guid(&self) -> &Vec<u8> {
        &self.generational_guid
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of IntegratedTCPIP
    pub fn set_integrated_tcpip(&mut self, value: bool) {
        self.integrated_tcpip = Some(value);
    }

    /// Gets the value of IntegratedTCPIP
    pub fn get_integrated_tcpip(&self) -> Option<&bool> {
        self.integrated_tcpip.as_ref()
    }

    /// Sets the value of MaxCDBLength
    pub fn set_max_cdblength(&mut self, value: u32) {
        self.max_cdblength = Some(value);
    }

    /// Gets the value of MaxCDBLength
    pub fn get_max_cdblength(&self) -> Option<&u32> {
        self.max_cdblength.as_ref()
    }

    /// Sets the value of MultifunctionDevice
    pub fn set_multifunction_device(&mut self, value: bool) {
        self.multifunction_device = Some(value);
    }

    /// Gets the value of MultifunctionDevice
    pub fn get_multifunction_device(&self) -> Option<&bool> {
        self.multifunction_device.as_ref()
    }

    /// Sets the value of NumberOfPorts
    pub fn set_number_of_ports(&mut self, value: u32) {
        self.number_of_ports = Some(value);
    }

    /// Gets the value of NumberOfPorts
    pub fn get_number_of_ports(&self) -> Option<&u32> {
        self.number_of_ports.as_ref()
    }

    /// Sets the value of OptionRomVersion
    pub fn set_option_rom_version(&mut self, value: String) {
        self.option_rom_version = Some(value);
    }

    /// Gets the value of OptionRomVersion
    pub fn get_option_rom_version(&self) -> Option<&String> {
        self.option_rom_version.as_ref()
    }

    /// Sets the value of RequiresBinaryIpAddresses
    pub fn set_requires_binary_ip_addresses(&mut self, value: bool) {
        self.requires_binary_ip_addresses = Some(value);
    }

    /// Gets the value of RequiresBinaryIpAddresses
    pub fn get_requires_binary_ip_addresses(&self) -> Option<&bool> {
        self.requires_binary_ip_addresses.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: HBAInformation_Status) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&HBAInformation_Status> {
        self.status.as_ref()
    }

    /// Sets the value of UniqueAdapterId
    pub fn set_unique_adapter_id(&mut self, value: u64) {
        self.unique_adapter_id = Some(value);
    }

    /// Gets the value of UniqueAdapterId
    pub fn get_unique_adapter_id(&self) -> Option<&u64> {
        self.unique_adapter_id.as_ref()
    }

    /// Sets the value of VendorID
    pub fn set_vendor_id(&mut self, value: String) {
        self.vendor_id = Some(value);
    }

    /// Gets the value of VendorID
    pub fn get_vendor_id(&self) -> Option<&String> {
        self.vendor_id.as_ref()
    }

    /// Sets the value of VendorModel
    pub fn set_vendor_model(&mut self, value: String) {
        self.vendor_model = Some(value);
    }

    /// Gets the value of VendorModel
    pub fn get_vendor_model(&self) -> Option<&String> {
        self.vendor_model.as_ref()
    }

    /// Sets the value of VendorVersion
    pub fn set_vendor_version(&mut self, value: String) {
        self.vendor_version = Some(value);
    }

    /// Gets the value of VendorVersion
    pub fn get_vendor_version(&self) -> Option<&String> {
        self.vendor_version.as_ref()
    }

    /// Sets the value of VersionMax
    pub fn set_version_max(&mut self, value: u8) {
        self.version_max = Some(value);
    }

    /// Gets the value of VersionMax
    pub fn get_version_max(&self) -> Option<&u8> {
        self.version_max.as_ref()
    }

    /// Sets the value of VersionMin
    pub fn set_version_min(&mut self, value: u8) {
        self.version_min = Some(value);
    }

    /// Gets the value of VersionMin
    pub fn get_version_min(&self) -> Option<&u8> {
        self.version_min.as_ref()
    }
}

