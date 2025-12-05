// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_StorageAllocationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_StorageAllocationSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// Access describes whether the allocated storage extent is 1 (readable), 2 (writeable), or 3 (both).
/// NOTE: This property is a copy of the CIM_StorageExtent.Access property, except deprecated values. See the description of the CIM_StorageExtent.Access property for details.
    #[serde(rename = "Access")]
    pub access: Option<StorageAllocationSettingData_Access>,

/// A unique identifier for the host extent. The identified host extent is used for the storage resource allocation.
/// NOTE: This property is a copy of the CIM_StorageExtent.Name property. See the description of CIM_StorageExtent.Name property for details.
    #[serde(rename = "HostExtentName")]
    pub host_extent_name: Option<String>,

/// The HostExtentNameFormat property identifies the format that is used for the value of the HostExtentName property.
/// NOTE: This property is a copy of the CIM_StorageExtent.NameFormat property, excluding deprecated values. See the description of CIM_StorageExtent.NameFormat class for details.
/// 7 = Serial Number/Vendor/Model (SNVM) SNVM is 3 strings representing the vendor name, product name within the vendor namespace, and the serial number within the model namespace. Strings are delimited with a '+'. Spaces may be included and are significant. The serial number is the text representation of the serial number in hexadecimal upper case. This represents the vendor and model ID from SCSI Inquiry data; the vendor field MUST be 8 characters wide and the product field MUST be 16 characters wide. For example, 
/// 'ACME____+SUPER DISK______+124437458' (_ is a space character) 
/// 9 = NAA as a generic format. See 
/// http://standards.ieee.org/regauth/oui/tutorials/fibrecomp_id.html. Formatted as 16 or 32 unseparated uppercase hex characters (2 per binary byte). For example '21000020372D3C73' 
/// 10 = EUI as a generic format (EUI64) See 
/// http://standards.ieee.org/regauth/oui/tutorials/EUI64.html. 
/// Formatted as 16 unseparated uppercase hex characters (2 per binary byte) 
/// 11 = T10 vendor identifier format as returned by SCSI Inquiry VPD page 83, identifier type 1. See T10 SPC-3 specification. This is the 8-byte ASCII vendor ID from the T10 registry followed by a vendor specific ASCII identifier; spaces are permitted. For non SCSI volumes, 'SNVM' may be the most appropriate choice. 12 = OS Device Name (for LogicalDisks). See LogicalDisk Name description for details.
    #[serde(rename = "HostExtentNameFormat")]
    pub host_extent_name_format: Option<StorageAllocationSettingData_HostExtentNameFormat>,

/// If the host extent is a SCSI volume, then the preferred source for SCSI volume names is SCSI VPD Page 83 responses.
/// NOTE: This property is a copy of the CIM_StorageExtent.NameNamespace property. See the description of CIM_StorageExtent.NameNamespace class for details.
/// Page 83 returns a list of identifiers for various device elements. The metadata for each identifier includes an Association field, identifiers with association of 0 apply to volumes. Page 83 supports several namespaces specified in the Type field in the identifier metadata. See SCSI SPC-3 specification. 
/// 2 = VPD Page 83, Type 3 NAA (NameFormat SHOULD be NAA) 
/// 3 = VPD Page 83, Type 2 EUI64 (NameFormat EUI) 
/// 4 = VPD Page 83, Type 1 T10 Vendor Identification 
/// (NameFormat T10) 
/// Less preferred volume namespaces from other interfaces: 
/// 5 = VPD page 80, Serial number (NameFormat SHOULD be Other) 
/// 6 = FC NodeWWN (NameFormat SHOULD be NAA or EUI) 
/// 7 = Serial Number/Vendor/Model (NameFormat SHOULD be SNVM) cThe preferred namespace for LogigicalDisk names is platform specific device namespace; see LogigicalDIsk Description. 
/// 8 = OS Device Namespace.
    #[serde(rename = "HostExtentNameNamespace")]
    pub host_extent_name_namespace: Option<StorageAllocationSettingData_HostExtentNameNamespace>,

/// The HostExtentStartingAddress property identifies the starting address on the host storage extent identified by the value of the HostExtentName property that is used for the allocation of the virtual storage extent.
/// A value of NULL indicates that there is no direct mapping of the virtual storage extent onto the referenced host storage extent.
/// NOTE: This property is a copy of the CIM_BasedOn.StartingAddess property. See the description of CIM_BasedOn association for details.
    #[serde(rename = "HostExtentStartingAddress")]
    pub host_extent_starting_address: Option<u64>,

/// Size in bytes of the blocks that are allocated at the host as the result of this storage resource allocation or storage resource allocation request. If the block size is variable, then the maximum block size in bytes should be specified. If the block size is unknown or if a block concept does not apply, then the value 1 shall be used.
/// NOTE: This property is a copy of the CIM_StorageExtent.BlockSize property. See the description of the CIM_StorageExtent.BlockSize property for details.
    #[serde(rename = "HostResourceBlockSize")]
    pub host_resource_block_size: Option<u64>,

/// A string describing the format of the HostExtentName property if the value of the HostExtentNameFormat property is 1 (Other).
    #[serde(rename = "OtherHostExtentNameFormat")]
    pub other_host_extent_name_format: Option<String>,

/// A string describing the namespace of the HostExtentName property if the value of the HostExtentNameNamespace matches 1 (Other).
    #[serde(rename = "OtherHostExtentNameNamespace")]
    pub other_host_extent_name_namespace: Option<String>,

/// Size in bytes of the blocks that are presented to the consumer as the result of this storage resource allocation or storage resource allocation request. If the block size is variable, then the maximum block size in bytes should be specified. If the block size is unknown or if a block concept does not apply, then the value 1 shall be used.
/// NOTE: The use of 1 (and not 0) to indicate that the blocksize is unknown still allows the use of the VirtualQuantity property to convey the size in blocks of size 1).
/// NOTE: This property is a copy of the CIM_StorageExtent.BlockSize property. See the description of the CIM_StorageExtent.BlockSize property for details.
    #[serde(rename = "VirtualResourceBlockSize")]
    pub virtual_resource_block_size: Option<u64>,
}

impl CIM_StorageAllocationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            access: None,
            host_extent_name: None,
            host_extent_name_format: None,
            host_extent_name_namespace: None,
            host_extent_starting_address: None,
            host_resource_block_size: None,
            other_host_extent_name_format: None,
            other_host_extent_name_namespace: None,
            virtual_resource_block_size: None,
        }
    }


    /// Sets the value of Access
    pub fn set_access(&mut self, value: StorageAllocationSettingData_Access) {
        self.access = Some(value);
    }

    /// Gets the value of Access
    pub fn get_access(&self) -> Option<&StorageAllocationSettingData_Access> {
        self.access.as_ref()
    }

    /// Sets the value of HostExtentName
    pub fn set_host_extent_name(&mut self, value: String) {
        self.host_extent_name = Some(value);
    }

    /// Gets the value of HostExtentName
    pub fn get_host_extent_name(&self) -> Option<&String> {
        self.host_extent_name.as_ref()
    }

    /// Sets the value of HostExtentNameFormat
    pub fn set_host_extent_name_format(&mut self, value: StorageAllocationSettingData_HostExtentNameFormat) {
        self.host_extent_name_format = Some(value);
    }

    /// Gets the value of HostExtentNameFormat
    pub fn get_host_extent_name_format(&self) -> Option<&StorageAllocationSettingData_HostExtentNameFormat> {
        self.host_extent_name_format.as_ref()
    }

    /// Sets the value of HostExtentNameNamespace
    pub fn set_host_extent_name_namespace(&mut self, value: StorageAllocationSettingData_HostExtentNameNamespace) {
        self.host_extent_name_namespace = Some(value);
    }

    /// Gets the value of HostExtentNameNamespace
    pub fn get_host_extent_name_namespace(&self) -> Option<&StorageAllocationSettingData_HostExtentNameNamespace> {
        self.host_extent_name_namespace.as_ref()
    }

    /// Sets the value of HostExtentStartingAddress
    pub fn set_host_extent_starting_address(&mut self, value: u64) {
        self.host_extent_starting_address = Some(value);
    }

    /// Gets the value of HostExtentStartingAddress
    pub fn get_host_extent_starting_address(&self) -> Option<&u64> {
        self.host_extent_starting_address.as_ref()
    }

    /// Sets the value of HostResourceBlockSize
    pub fn set_host_resource_block_size(&mut self, value: u64) {
        self.host_resource_block_size = Some(value);
    }

    /// Gets the value of HostResourceBlockSize
    pub fn get_host_resource_block_size(&self) -> Option<&u64> {
        self.host_resource_block_size.as_ref()
    }

    /// Sets the value of OtherHostExtentNameFormat
    pub fn set_other_host_extent_name_format(&mut self, value: String) {
        self.other_host_extent_name_format = Some(value);
    }

    /// Gets the value of OtherHostExtentNameFormat
    pub fn get_other_host_extent_name_format(&self) -> Option<&String> {
        self.other_host_extent_name_format.as_ref()
    }

    /// Sets the value of OtherHostExtentNameNamespace
    pub fn set_other_host_extent_name_namespace(&mut self, value: String) {
        self.other_host_extent_name_namespace = Some(value);
    }

    /// Gets the value of OtherHostExtentNameNamespace
    pub fn get_other_host_extent_name_namespace(&self) -> Option<&String> {
        self.other_host_extent_name_namespace.as_ref()
    }

    /// Sets the value of VirtualResourceBlockSize
    pub fn set_virtual_resource_block_size(&mut self, value: u64) {
        self.virtual_resource_block_size = Some(value);
    }

    /// Gets the value of VirtualResourceBlockSize
    pub fn get_virtual_resource_block_size(&self) -> Option<&u64> {
        self.virtual_resource_block_size.as_ref()
    }
}

