// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.AccessLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftUal_SystemId struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftUal_SystemId {

/// The number of cores for an instance of the physical processor in the system. For example, for a dual-core processor system, this property has a value of 2.
    #[serde(rename = "CoresPerPhysicalProcessor")]
    pub cores_per_physical_processor: Option<u32>,

/// The date and time that the current operating system first became operational with this set of system identity properties. If the properties of a system change, then a new System Identity record is created.
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// The number of logical processors for an instance of a Hyper-Thread capable physical processor in the system. For example, in a Hyper-Thread quad-core processor system, this property has a value of 8.
    #[serde(rename = "LogicalProcessorsPerPhysicalProcessor")]
    pub logical_processors_per_physical_processor: Option<u32>,

/// The maximum system memory size (in bytes). For a virtual machine, this value represents what the hypervisor configured for the virtual machine’s memory size.
    #[serde(rename = "MaximumMemory")]
    pub maximum_memory: Option<u64>,

/// The build number of the operating system.
    #[serde(rename = "OSBuildNumber")]
    pub osbuild_number: Option<u32>,

/// The code for the country or region that an operating system uses. Values are based on international phone dialing prefixes.
    #[serde(rename = "OSCountryCode")]
    pub oscountry_code: Option<String>,

/// The number, in minutes, an operating system is offset from Greenwich mean time (GMT). The number is positive, negative, or zero.
    #[serde(rename = "OSCurrentTimeZone")]
    pub oscurrent_time_zone: Option<i16>,

/// If True, the daylight savings mode is ON.
    #[serde(rename = "OSDaylightInEffect")]
    pub osdaylight_in_effect: Option<bool>,

/// The date and time the operating system was last restarted.
    #[serde(rename = "OSLastBootUpTime")]
    pub oslast_boot_up_time: Option<String>,

/// The major portion of the version number of the operating system.
    #[serde(rename = "OSMajor")]
    pub osmajor: Option<u32>,

/// The minor portion of the version number of the operating system.
    #[serde(rename = "OSMinor")]
    pub osminor: Option<u32>,

/// An integer that represents the operating system platform. The possible values of the data property are "1" to indicate an unsupported Windows system and "2" to indicate a supported Windows system.
    #[serde(rename = "OSPlatformId")]
    pub osplatform_id: Option<u32>,

/// An enumeration type that identifies the operating system that you are running.
    #[serde(rename = "OSProductType")]
    pub osproduct_type: Option<u32>,

/// The operating system product serial identification number.
    #[serde(rename = "OSSerialNumber")]
    pub osserial_number: Option<String>,

/// The SuiteMask of the local system.
    #[serde(rename = "OSSuiteMask")]
    pub ossuite_mask: Option<u32>,

/// The number of physical processors currently available on a system.
    #[serde(rename = "PhysicalProcessorCount")]
    pub physical_processor_count: Option<u32>,

/// The major version number of the service pack.
    #[serde(rename = "ServicePackMajor")]
    pub service_pack_major: Option<u32>,

/// The minor version number of the service pack.
    #[serde(rename = "ServicePackMinor")]
    pub service_pack_minor: Option<u32>,

/// The server name according to the domain name server (DNS).
    #[serde(rename = "SystemDNSHostName")]
    pub system_dnshost_name: Option<String>,

/// The name of the domain, or workgroup, to which the server belongs.
    #[serde(rename = "SystemDomainName")]
    pub system_domain_name: Option<String>,

/// The name of the BIOS manufacturer.
    #[serde(rename = "SystemManufacturer")]
    pub system_manufacturer: Option<String>,

/// The product name specified in the system BIOS.
    #[serde(rename = "SystemProductName")]
    pub system_product_name: Option<String>,

/// The unit identification for the local server.
    #[serde(rename = "SystemSerialNumber")]
    pub system_serial_number: Option<String>,

/// The SMBIOS reported universally unique identifier for this server unit.
    #[serde(rename = "SystemSMBIOSUUID")]
    pub system_smbiosuuid: Option<String>,
}

impl MsftUal_SystemId {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cores_per_physical_processor: None,
            creation_time: None,
            logical_processors_per_physical_processor: None,
            maximum_memory: None,
            osbuild_number: None,
            oscountry_code: None,
            oscurrent_time_zone: None,
            osdaylight_in_effect: None,
            oslast_boot_up_time: None,
            osmajor: None,
            osminor: None,
            osplatform_id: None,
            osproduct_type: None,
            osserial_number: None,
            ossuite_mask: None,
            physical_processor_count: None,
            service_pack_major: None,
            service_pack_minor: None,
            system_dnshost_name: None,
            system_domain_name: None,
            system_manufacturer: None,
            system_product_name: None,
            system_serial_number: None,
            system_smbiosuuid: None,
        }
    }


    /// Sets the value of CoresPerPhysicalProcessor
    pub fn set_cores_per_physical_processor(&mut self, value: u32) {
        self.cores_per_physical_processor = Some(value);
    }

    /// Gets the value of CoresPerPhysicalProcessor
    pub fn get_cores_per_physical_processor(&self) -> Option<&u32> {
        self.cores_per_physical_processor.as_ref()
    }

    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of LogicalProcessorsPerPhysicalProcessor
    pub fn set_logical_processors_per_physical_processor(&mut self, value: u32) {
        self.logical_processors_per_physical_processor = Some(value);
    }

    /// Gets the value of LogicalProcessorsPerPhysicalProcessor
    pub fn get_logical_processors_per_physical_processor(&self) -> Option<&u32> {
        self.logical_processors_per_physical_processor.as_ref()
    }

    /// Sets the value of MaximumMemory
    pub fn set_maximum_memory(&mut self, value: u64) {
        self.maximum_memory = Some(value);
    }

    /// Gets the value of MaximumMemory
    pub fn get_maximum_memory(&self) -> Option<&u64> {
        self.maximum_memory.as_ref()
    }

    /// Sets the value of OSBuildNumber
    pub fn set_osbuild_number(&mut self, value: u32) {
        self.osbuild_number = Some(value);
    }

    /// Gets the value of OSBuildNumber
    pub fn get_osbuild_number(&self) -> Option<&u32> {
        self.osbuild_number.as_ref()
    }

    /// Sets the value of OSCountryCode
    pub fn set_oscountry_code(&mut self, value: String) {
        self.oscountry_code = Some(value);
    }

    /// Gets the value of OSCountryCode
    pub fn get_oscountry_code(&self) -> Option<&String> {
        self.oscountry_code.as_ref()
    }

    /// Sets the value of OSCurrentTimeZone
    pub fn set_oscurrent_time_zone(&mut self, value: i16) {
        self.oscurrent_time_zone = Some(value);
    }

    /// Gets the value of OSCurrentTimeZone
    pub fn get_oscurrent_time_zone(&self) -> Option<&i16> {
        self.oscurrent_time_zone.as_ref()
    }

    /// Sets the value of OSDaylightInEffect
    pub fn set_osdaylight_in_effect(&mut self, value: bool) {
        self.osdaylight_in_effect = Some(value);
    }

    /// Gets the value of OSDaylightInEffect
    pub fn get_osdaylight_in_effect(&self) -> Option<&bool> {
        self.osdaylight_in_effect.as_ref()
    }

    /// Sets the value of OSLastBootUpTime
    pub fn set_oslast_boot_up_time(&mut self, value: String) {
        self.oslast_boot_up_time = Some(value);
    }

    /// Gets the value of OSLastBootUpTime
    pub fn get_oslast_boot_up_time(&self) -> Option<&String> {
        self.oslast_boot_up_time.as_ref()
    }

    /// Sets the value of OSMajor
    pub fn set_osmajor(&mut self, value: u32) {
        self.osmajor = Some(value);
    }

    /// Gets the value of OSMajor
    pub fn get_osmajor(&self) -> Option<&u32> {
        self.osmajor.as_ref()
    }

    /// Sets the value of OSMinor
    pub fn set_osminor(&mut self, value: u32) {
        self.osminor = Some(value);
    }

    /// Gets the value of OSMinor
    pub fn get_osminor(&self) -> Option<&u32> {
        self.osminor.as_ref()
    }

    /// Sets the value of OSPlatformId
    pub fn set_osplatform_id(&mut self, value: u32) {
        self.osplatform_id = Some(value);
    }

    /// Gets the value of OSPlatformId
    pub fn get_osplatform_id(&self) -> Option<&u32> {
        self.osplatform_id.as_ref()
    }

    /// Sets the value of OSProductType
    pub fn set_osproduct_type(&mut self, value: u32) {
        self.osproduct_type = Some(value);
    }

    /// Gets the value of OSProductType
    pub fn get_osproduct_type(&self) -> Option<&u32> {
        self.osproduct_type.as_ref()
    }

    /// Sets the value of OSSerialNumber
    pub fn set_osserial_number(&mut self, value: String) {
        self.osserial_number = Some(value);
    }

    /// Gets the value of OSSerialNumber
    pub fn get_osserial_number(&self) -> Option<&String> {
        self.osserial_number.as_ref()
    }

    /// Sets the value of OSSuiteMask
    pub fn set_ossuite_mask(&mut self, value: u32) {
        self.ossuite_mask = Some(value);
    }

    /// Gets the value of OSSuiteMask
    pub fn get_ossuite_mask(&self) -> Option<&u32> {
        self.ossuite_mask.as_ref()
    }

    /// Sets the value of PhysicalProcessorCount
    pub fn set_physical_processor_count(&mut self, value: u32) {
        self.physical_processor_count = Some(value);
    }

    /// Gets the value of PhysicalProcessorCount
    pub fn get_physical_processor_count(&self) -> Option<&u32> {
        self.physical_processor_count.as_ref()
    }

    /// Sets the value of ServicePackMajor
    pub fn set_service_pack_major(&mut self, value: u32) {
        self.service_pack_major = Some(value);
    }

    /// Gets the value of ServicePackMajor
    pub fn get_service_pack_major(&self) -> Option<&u32> {
        self.service_pack_major.as_ref()
    }

    /// Sets the value of ServicePackMinor
    pub fn set_service_pack_minor(&mut self, value: u32) {
        self.service_pack_minor = Some(value);
    }

    /// Gets the value of ServicePackMinor
    pub fn get_service_pack_minor(&self) -> Option<&u32> {
        self.service_pack_minor.as_ref()
    }

    /// Sets the value of SystemDNSHostName
    pub fn set_system_dnshost_name(&mut self, value: String) {
        self.system_dnshost_name = Some(value);
    }

    /// Gets the value of SystemDNSHostName
    pub fn get_system_dnshost_name(&self) -> Option<&String> {
        self.system_dnshost_name.as_ref()
    }

    /// Sets the value of SystemDomainName
    pub fn set_system_domain_name(&mut self, value: String) {
        self.system_domain_name = Some(value);
    }

    /// Gets the value of SystemDomainName
    pub fn get_system_domain_name(&self) -> Option<&String> {
        self.system_domain_name.as_ref()
    }

    /// Sets the value of SystemManufacturer
    pub fn set_system_manufacturer(&mut self, value: String) {
        self.system_manufacturer = Some(value);
    }

    /// Gets the value of SystemManufacturer
    pub fn get_system_manufacturer(&self) -> Option<&String> {
        self.system_manufacturer.as_ref()
    }

    /// Sets the value of SystemProductName
    pub fn set_system_product_name(&mut self, value: String) {
        self.system_product_name = Some(value);
    }

    /// Gets the value of SystemProductName
    pub fn get_system_product_name(&self) -> Option<&String> {
        self.system_product_name.as_ref()
    }

    /// Sets the value of SystemSerialNumber
    pub fn set_system_serial_number(&mut self, value: String) {
        self.system_serial_number = Some(value);
    }

    /// Gets the value of SystemSerialNumber
    pub fn get_system_serial_number(&self) -> Option<&String> {
        self.system_serial_number.as_ref()
    }

    /// Sets the value of SystemSMBIOSUUID
    pub fn set_system_smbiosuuid(&mut self, value: String) {
        self.system_smbiosuuid = Some(value);
    }

    /// Gets the value of SystemSMBIOSUUID
    pub fn get_system_smbiosuuid(&self) -> Option<&String> {
        self.system_smbiosuuid.as_ref()
    }
}

