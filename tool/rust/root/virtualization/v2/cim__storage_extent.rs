// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_StorageExtent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_StorageExtent {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// Access describes whether the media is readable (value=1), writeable (value=2), or both (value=3). "Unknown" (0) and "Write Once" (4) can also be defined.
    #[serde(rename = "Access")]
    pub access: Option<StorageExtent_Access>,

/// Size in bytes of the blocks which form this StorageExtent. If variable block size, then the maximum block size in bytes should be specified. If the block size is unknown or if a block concept is not valid (for example, for AggregateExtents, Memory or LogicalDisks), enter a 1.
    #[serde(rename = "BlockSize")]
    pub block_size: Option<u64>,

/// The maximum number of blocks, of size BlockSize, which are available for consumption when layering StorageExtents using the BasedOn association. This property only has meaning when this StorageExtent is an Antecedent reference in a BasedOn relationship. For example, a StorageExtent could be composed of 120 blocks. However, the Extent itself may use 20 blocks for redundancy data. If another StorageExtent is BasedOn this Extent, only 100 blocks would be available to it. This information ('100 blocks is available for consumption') is indicated in the ConsumableBlocks property.
    #[serde(rename = "ConsumableBlocks")]
    pub consumable_blocks: Option<u64>,

/// Type of data organization used.
    #[serde(rename = "DataOrganization")]
    pub data_organization: Option<StorageExtent_DataOrganization>,

/// Number of complete copies of data currently maintained.
    #[serde(rename = "DataRedundancy")]
    pub data_redundancy: Option<u16>,

/// Current value for Delta reservation. This is a percentage that specifies the amount of space that should be reserved in a replica for caching changes.
    #[serde(rename = "DeltaReservation")]
    pub delta_reservation: Option<u8>,

/// ErrorMethodology is a free-form string describing the type of error detection and correction supported by this StorageExtent.
    #[serde(rename = "ErrorMethodology")]
    pub error_methodology: Option<String>,

/// StorageExtents have additional status information beyond that captured in the OperationalStatus and other properties, inherited from ManagedSystemElement. This additional information (for example, "Protection Disabled", value=9) is captured in the ExtentStatus property. 
/// 'In-Band Access Granted' says that access to data on an extent is granted to some consumer and is only valid when 'Exported' is also set. It is set as a side effect of PrivilegeManagementService.ChangeAccess or equivalent interfaces. 
/// 'Imported' indicates that the extent is used in the current system, but known to be managed by some other system. For example, a server imports volumes from a disk array. 
/// 'Exported' indicates the extent is meant to be used by some comsumer. A disk array's logical units are exported. 
/// Intermediate composite extents may be neither imported nor exported.
    #[serde(rename = "ExtentStatus")]
    pub extent_status: Vec<StorageExtent_ExtentStatus>,

/// True indicates that the underlying StorageExtent(s) participate in a StorageRedundancyGroup.
    #[serde(rename = "IsBasedOnUnderlyingRedundancy")]
    pub is_based_on_underlying_redundancy: Option<bool>,

/// The list here applies to all StorageExtent subclasses. Please look at the Description in each subclass for guidelines on the approriate values for that subclass. Note that any of these formats could apply to a CompositeExtent. 
/// 
/// Note - this property originally touched on two concepts that are now separated into this property and NameNamespace. Values 2,3,4,5,6, and 8 are retained for backwards compatibility but are deprecated in lieu of the corresponding values in CIM_StorageExtent.NameNamespace. 
/// 
/// For example, the preferred source for SCSI virtual (RAID) disk names is from Inquiry VPD page 83 response, type 3 identifiers. These will have NameFormat set to 'NAA' and NameNamespace to 'VPD83Type3'. 
/// 
/// Format of the Name property. Values for extents representing SCSI volumes are (per SCSI SPC-3): 
/// 2 = VPD Page 83, NAA IEEE Registered Extended (VPD83NAA6) 
/// (DEPRECATED) 
/// 3 = VPD Page 83, NAA IEEE Registered (VPD83NAA5) 
/// (DEPRECATED) 
/// 4 = VPD Page 83, (VPD83Type2) (DEPRECATED) 
/// 5 = VPD Page 83, 
/// T10 Vendor Identification (VPD83Type1) (DEPRECATED) 
/// 6 = VPD Page 83, Vendor Specific (VPD83Type0) (DEPRECATED) 
/// 7 = Serial Number/Vendor/Model (SNVM) SNVM is 3 strings representing the vendor name, product name within the vendor namespace, and the serial number within the model namespace. Strings are delimited with a '+'. Spaces may be included and are significant. The serial number is the text representation of the serial number in hexadecimal upper case. This represents the vendor and model ID from SCSI Inquiry data; the vendor field MUST be 8 characters wide and the product field MUST be 16 characters wide. For example, 
/// 'ACME____+SUPER DISK______+124437458' (_ is a space character) 
/// 8 = Node WWN (for single LUN/controller) (NodeWWN) 
/// (DEPRECATED) 
/// 9 = NAA as a generic format. See 
/// http://standards.ieee.org/regauth/oui/tutorials/fibrecomp_id.html. Formatted as 16 or 32 unseparated uppercase hex characters (2 per binary byte). For example '21000020372D3C73' 
/// 10 = EUI as a generic format (EUI64) See 
/// http://standards.ieee.org/regauth/oui/tutorials/EUI64.html. 
/// Formatted as 16 unseparated uppercase hex characters (2 per binary byte) 
/// 11 = T10 vendor identifier format as returned by SCSI Inquiry VPD page 83, identifier type 1. See T10 SPC-3 specification. This is the 8-byte ASCII vendor ID from the T10 registry followed by a vendor specific ASCII identifier; spaces are permitted. For non SCSI volumes, 'SNVM' may be the most appropriate choice. 12 = OS Device Name (for LogicalDisks). See LogicalDisk Name description for details.
    #[serde(rename = "NameFormat")]
    pub name_format: Option<StorageExtent_NameFormat>,

/// The preferred source SCSI for volume names is SCSI VPD Page 83 responses. Page 83 returns a list of identifiers for various device elements. The metadata for each identifier includes an Association field, identifiers with association of 0 apply to volumes. Page 83 supports several namespaces specified in the Type field in the identifier metadata. See SCSI SPC-3 specification. 
/// 2 = VPD Page 83, Type 3 NAA (NameFormat SHOULD be NAA) 
/// 3 = VPD Page 83, Type 2 EUI64 (NameFormat EUI) 
/// 4 = VPD Page 83, Type 1 T10 Vendor Identification 
/// (NameFormat T10) 
/// Less preferred volume namespaces from other interfaces: 
/// 5 = VPD page 80, Serial number (NameFormat SHOULD be Other) 
/// 6 = FC NodeWWN (NameFormat SHOULD be NAA or EUI) 
/// 7 = Serial Number/Vendor/Model (NameFormat SHOULD be SNVM) 
/// The preferred namespace for LogigicalDisk names is platform specific device namespace; see LogigicalDIsk Description. 
/// 8 = OS Device Namespace.
    #[serde(rename = "NameNamespace")]
    pub name_namespace: Option<StorageExtent_NameNamespace>,

/// Indicates whether or not there exists no single point of failure.
    #[serde(rename = "NoSinglePointOfFailure")]
    pub no_single_point_of_failure: Option<bool>,

/// Total number of logically contiguous blocks, of size Block Size, which form this Extent. The total size of the Extent can be calculated by multiplying BlockSize by NumberOfBlocks. If the BlockSize is 1, this property is the total size of the Extent.
    #[serde(rename = "NumberOfBlocks")]
    pub number_of_blocks: Option<u64>,

/// A string describing the format of the Name property when NameFormat includes the value 1, "Other".
    #[serde(rename = "OtherNameFormat")]
    pub other_name_format: Option<String>,

/// A string describing the namespace of the Name property when NameNamespace includes the value 1, "Other".
    #[serde(rename = "OtherNameNamespace")]
    pub other_name_namespace: Option<String>,

/// How many physical packages can currently fail without data loss. For example, in the storage domain, this might be disk spindles.
    #[serde(rename = "PackageRedundancy")]
    pub package_redundancy: Option<u16>,

/// If true, "Primordial" indicates that the containing System does not have the ability to create or delete this operational element. This is important because StorageExtents are assembled into higher-level abstractions using the BasedOn association. Although the higher-level abstractions can be created and deleted, the most basic, (i.e. primordial), hardware-based storage entities cannot. They are physically realized as part of the System, or are actually managed by some other System and imported as if they were physically realized. In other words, a Primordial StorageExtent exists in, but is not created by its System and conversely a non-Primordial StorageExtent is created in the context of its System. For StorageVolumes, this property will generally be false. One use of this property is to enable algorithms that aggregate StorageExtent.ConsumableSpace across all, StorageExtents but that also want to distinquish the space that underlies Primordial StoragePools. Since implementations are not required to surface all Component StorageExtents of a StoragePool, this information is not accessible in any other way.
    #[serde(rename = "Primordial")]
    pub primordial: Option<bool>,

/// A free form string describing the media and/or its use.
    #[serde(rename = "Purpose")]
    pub purpose: Option<String>,

/// Boolean set to TRUE if the Storage is sequentially accessed by a MediaAccessDevice. A TapePartition is an example of a sequentially accessed StorageExtent. StorageVolumes, Disk Partitions and LogicalDisks represent randomly accessed Extents.
    #[serde(rename = "SequentialAccess")]
    pub sequential_access: Option<bool>,
}

impl CIM_StorageExtent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            access: None,
            block_size: None,
            consumable_blocks: None,
            data_organization: None,
            data_redundancy: None,
            delta_reservation: None,
            error_methodology: None,
            extent_status: Vec::new(),
            is_based_on_underlying_redundancy: None,
            name_format: None,
            name_namespace: None,
            no_single_point_of_failure: None,
            number_of_blocks: None,
            other_name_format: None,
            other_name_namespace: None,
            package_redundancy: None,
            primordial: None,
            purpose: None,
            sequential_access: None,
        }
    }


    /// Sets the value of Access
    pub fn set_access(&mut self, value: StorageExtent_Access) {
        self.access = Some(value);
    }

    /// Gets the value of Access
    pub fn get_access(&self) -> Option<&StorageExtent_Access> {
        self.access.as_ref()
    }

    /// Sets the value of BlockSize
    pub fn set_block_size(&mut self, value: u64) {
        self.block_size = Some(value);
    }

    /// Gets the value of BlockSize
    pub fn get_block_size(&self) -> Option<&u64> {
        self.block_size.as_ref()
    }

    /// Sets the value of ConsumableBlocks
    pub fn set_consumable_blocks(&mut self, value: u64) {
        self.consumable_blocks = Some(value);
    }

    /// Gets the value of ConsumableBlocks
    pub fn get_consumable_blocks(&self) -> Option<&u64> {
        self.consumable_blocks.as_ref()
    }

    /// Sets the value of DataOrganization
    pub fn set_data_organization(&mut self, value: StorageExtent_DataOrganization) {
        self.data_organization = Some(value);
    }

    /// Gets the value of DataOrganization
    pub fn get_data_organization(&self) -> Option<&StorageExtent_DataOrganization> {
        self.data_organization.as_ref()
    }

    /// Sets the value of DataRedundancy
    pub fn set_data_redundancy(&mut self, value: u16) {
        self.data_redundancy = Some(value);
    }

    /// Gets the value of DataRedundancy
    pub fn get_data_redundancy(&self) -> Option<&u16> {
        self.data_redundancy.as_ref()
    }

    /// Sets the value of DeltaReservation
    pub fn set_delta_reservation(&mut self, value: u8) {
        self.delta_reservation = Some(value);
    }

    /// Gets the value of DeltaReservation
    pub fn get_delta_reservation(&self) -> Option<&u8> {
        self.delta_reservation.as_ref()
    }

    /// Sets the value of ErrorMethodology
    pub fn set_error_methodology(&mut self, value: String) {
        self.error_methodology = Some(value);
    }

    /// Gets the value of ErrorMethodology
    pub fn get_error_methodology(&self) -> Option<&String> {
        self.error_methodology.as_ref()
    }

    /// Sets the value of ExtentStatus
    pub fn set_extent_status(&mut self, value: Vec<StorageExtent_ExtentStatus>) {
        self.extent_status = value;
    }

    /// Gets the value of ExtentStatus
    pub fn get_extent_status(&self) -> &Vec<StorageExtent_ExtentStatus> {
        &self.extent_status
    }

    /// Sets the value of IsBasedOnUnderlyingRedundancy
    pub fn set_is_based_on_underlying_redundancy(&mut self, value: bool) {
        self.is_based_on_underlying_redundancy = Some(value);
    }

    /// Gets the value of IsBasedOnUnderlyingRedundancy
    pub fn get_is_based_on_underlying_redundancy(&self) -> Option<&bool> {
        self.is_based_on_underlying_redundancy.as_ref()
    }

    /// Sets the value of NameFormat
    pub fn set_name_format(&mut self, value: StorageExtent_NameFormat) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&StorageExtent_NameFormat> {
        self.name_format.as_ref()
    }

    /// Sets the value of NameNamespace
    pub fn set_name_namespace(&mut self, value: StorageExtent_NameNamespace) {
        self.name_namespace = Some(value);
    }

    /// Gets the value of NameNamespace
    pub fn get_name_namespace(&self) -> Option<&StorageExtent_NameNamespace> {
        self.name_namespace.as_ref()
    }

    /// Sets the value of NoSinglePointOfFailure
    pub fn set_no_single_point_of_failure(&mut self, value: bool) {
        self.no_single_point_of_failure = Some(value);
    }

    /// Gets the value of NoSinglePointOfFailure
    pub fn get_no_single_point_of_failure(&self) -> Option<&bool> {
        self.no_single_point_of_failure.as_ref()
    }

    /// Sets the value of NumberOfBlocks
    pub fn set_number_of_blocks(&mut self, value: u64) {
        self.number_of_blocks = Some(value);
    }

    /// Gets the value of NumberOfBlocks
    pub fn get_number_of_blocks(&self) -> Option<&u64> {
        self.number_of_blocks.as_ref()
    }

    /// Sets the value of OtherNameFormat
    pub fn set_other_name_format(&mut self, value: String) {
        self.other_name_format = Some(value);
    }

    /// Gets the value of OtherNameFormat
    pub fn get_other_name_format(&self) -> Option<&String> {
        self.other_name_format.as_ref()
    }

    /// Sets the value of OtherNameNamespace
    pub fn set_other_name_namespace(&mut self, value: String) {
        self.other_name_namespace = Some(value);
    }

    /// Gets the value of OtherNameNamespace
    pub fn get_other_name_namespace(&self) -> Option<&String> {
        self.other_name_namespace.as_ref()
    }

    /// Sets the value of PackageRedundancy
    pub fn set_package_redundancy(&mut self, value: u16) {
        self.package_redundancy = Some(value);
    }

    /// Gets the value of PackageRedundancy
    pub fn get_package_redundancy(&self) -> Option<&u16> {
        self.package_redundancy.as_ref()
    }

    /// Sets the value of Primordial
    pub fn set_primordial(&mut self, value: bool) {
        self.primordial = Some(value);
    }

    /// Gets the value of Primordial
    pub fn get_primordial(&self) -> Option<&bool> {
        self.primordial.as_ref()
    }

    /// Sets the value of Purpose
    pub fn set_purpose(&mut self, value: String) {
        self.purpose = Some(value);
    }

    /// Gets the value of Purpose
    pub fn get_purpose(&self) -> Option<&String> {
        self.purpose.as_ref()
    }

    /// Sets the value of SequentialAccess
    pub fn set_sequential_access(&mut self, value: bool) {
        self.sequential_access = Some(value);
    }

    /// Gets the value of SequentialAccess
    pub fn get_sequential_access(&self) -> Option<&bool> {
        self.sequential_access.as_ref()
    }
}

