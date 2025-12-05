// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MediaAccessDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MediaAccessDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// Capabilities of the MediaAccessDevice. For example, the Device may support "Random Access", removeable media and "Automatic Cleaning". In this case, the values 3, 7 and 9 would be written to the array. 
/// Several of the enumerated values require some explanation: 1) Value 11, Supports Dual Sided Media, distinguishes a Device that can access both sides of dual sided Media, from a Device that reads only a single side and requires the Media to be flipped; and, 2) Value 12, Predismount Eject Not Required, indicates that Media does not have to be explicitly ejected from the Device before being accessed by a PickerElement.
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<MediaAccessDevice_Capabilities>,

/// An array of free-form strings providing more detailed explanations for any of the AccessDevice features indicated in the Capabilities array. Note, each entry of this array is related to the entry in the Capabilities array that is located at the same index.
    #[serde(rename = "CapabilityDescriptions")]
    pub capability_descriptions: Vec<String>,

/// A free form string indicating the algorithm or tool used by the device to support compression. If it is not possible or not desired to describe the compression scheme (perhaps because it is not known), recommend using the following words: "Unknown" to represent that it is not known whether the device supports compression capabilities or not, "Compressed" to represent that the device supports compression capabilities but either its compression scheme is not known or not disclosed, and "Not Compressed" to represent that the devices does not support compression capabilities.
    #[serde(rename = "CompressionMethod")]
    pub compression_method: Option<String>,

/// Default block size, in bytes, for this Device.
    #[serde(rename = "DefaultBlockSize")]
    pub default_block_size: Option<u64>,

/// ErrorMethodology is a free-form string describing the type(s) of error detection and correction supported by this Device.
    #[serde(rename = "ErrorMethodology")]
    pub error_methodology: Option<String>,

/// The date and time on which the Device was last cleaned.
    #[serde(rename = "LastCleaned")]
    pub last_cleaned: Option<String>,

/// Time in milliseconds from 'load' to being able to read or write a Media. For example, for DiskDrives, this is the interval between a disk not spinning to the disk reporting that it is ready for read/write (ie, the disk spinning at nominal speeds). For TapeDrives, this is the time from a Media being injected to reporting that it is ready for an application. This is usually at the tape's BOT area.
    #[serde(rename = "LoadTime")]
    pub load_time: Option<u64>,

/// Time in milliseconds to move from the first location on the Media to the location that is furthest with respect to time. For a DiskDrive, this represents full seek + full rotational delay. For TapeDrives, this represents a search from the beginning of the tape to the most physically distant point. (The end of a tape may be at its most physically distant point, but this is not necessarily true.)
    #[serde(rename = "MaxAccessTime")]
    pub max_access_time: Option<u64>,

/// Maximum block size, in bytes, for media accessed by this Device.
    #[serde(rename = "MaxBlockSize")]
    pub max_block_size: Option<u64>,

/// Maximum size, in KBytes, of media supported by this Device. KBytes is interpreted as the number of bytes multiplied by 1000 (NOT the number of bytes multiplied by 1024).
    #[serde(rename = "MaxMediaSize")]
    pub max_media_size: Option<u64>,

/// An unsigned integer indicating the maximum 'units' that can be used, with respect to the AccessDevice, before the Device should be cleaned. The property, UnitsDescription, defines how 'units' should be interpreted.
    #[serde(rename = "MaxUnitsBeforeCleaning")]
    pub max_units_before_cleaning: Option<u64>,

/// True indicates that the media is locked in the Device and can not be ejected. For non-removeable Devices, this value should be true.
    #[serde(rename = "MediaIsLocked")]
    pub media_is_locked: Option<bool>,

/// Minimum block size, in bytes, for media accessed by this Device.
    #[serde(rename = "MinBlockSize")]
    pub min_block_size: Option<u64>,

/// For a MediaAccessDevice that supports removable Media, the number of times that Media have been mounted for data transfer or to clean the Device. For Devices accessing nonremovable Media, such as hard disks, this property is not applicable and should be set to 0.
    #[serde(rename = "MountCount")]
    pub mount_count: Option<u64>,

/// Boolean indicating that the MediaAccessDevice needs cleaning. Whether manual or automatic cleaning is possible is indicated in the Capabilities array property.
    #[serde(rename = "NeedsCleaning")]
    pub needs_cleaning: Option<bool>,

/// When the MediaAccessDevice supports multiple individual Media, this property defines the maximum number which can be supported or inserted.
    #[serde(rename = "NumberOfMediaSupported")]
    pub number_of_media_supported: Option<u32>,

/// An enumeration indicating the operational security defined for the MediaAccessDevice. For example, information that the Device is "Read Only" (value=4) or "Boot Bypass" (value=6) can be described using this property.
    #[serde(rename = "Security")]
    pub security: Option<MediaAccessDevice_Security>,

/// For a MediaAccessDevice that supports removable Media, the most recent date and time that Media was mounted on the Device. For Devices accessing nonremovable Media, such as hard disks, this property has no meaning and is not applicable.
    #[serde(rename = "TimeOfLastMount")]
    pub time_of_last_mount: Option<String>,

/// For a MediaAccessDevice that supports removable Media, the total time (in seconds) that Media have been mounted for data transfer or to clean the Device. For Devices accessing nonremovable Media, such as hard disks, this property is not applicable and should be set to 0.
    #[serde(rename = "TotalMountTime")]
    pub total_mount_time: Option<u64>,

/// The sustained data transfer rate in KB/sec that the Device can read from and write to a Media. This is a sustained, raw data rate. Maximum rates or rates assuming compression should not be reported in this property.
    #[serde(rename = "UncompressedDataRate")]
    pub uncompressed_data_rate: Option<u32>,

/// Defines 'Units' relative to its use in the property, MaxUnitsBeforeCleaning. This describes the criteria used to determine when the MediaAccessDevice should be cleaned.
    #[serde(rename = "UnitsDescription")]
    pub units_description: Option<String>,

/// An unsigned integer indicating the currently used 'units' of the AccessDevice, helpful to describe when the Device may require cleaning. The property, UnitsDescription, defines how 'units' should be interpreted.
    #[serde(rename = "UnitsUsed")]
    pub units_used: Option<u64>,

/// Time in milliseconds from being able to read or write a Media to its 'unload'. For example, for DiskDrives, this is the interval between a disk spinning at nominal speeds and a disk not spinning. For TapeDrives, this is the time for a Media to go from its BOT to being fully ejected and accessible to a PickerElement or human operator.
    #[serde(rename = "UnloadTime")]
    pub unload_time: Option<u64>,
}

impl CIM_MediaAccessDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            capabilities: Vec::new(),
            capability_descriptions: Vec::new(),
            compression_method: None,
            default_block_size: None,
            error_methodology: None,
            last_cleaned: None,
            load_time: None,
            max_access_time: None,
            max_block_size: None,
            max_media_size: None,
            max_units_before_cleaning: None,
            media_is_locked: None,
            min_block_size: None,
            mount_count: None,
            needs_cleaning: None,
            number_of_media_supported: None,
            security: None,
            time_of_last_mount: None,
            total_mount_time: None,
            uncompressed_data_rate: None,
            units_description: None,
            units_used: None,
            unload_time: None,
        }
    }


    /// Sets the value of Capabilities
    pub fn set_capabilities(&mut self, value: Vec<MediaAccessDevice_Capabilities>) {
        self.capabilities = value;
    }

    /// Gets the value of Capabilities
    pub fn get_capabilities(&self) -> &Vec<MediaAccessDevice_Capabilities> {
        &self.capabilities
    }

    /// Sets the value of CapabilityDescriptions
    pub fn set_capability_descriptions(&mut self, value: Vec<String>) {
        self.capability_descriptions = value;
    }

    /// Gets the value of CapabilityDescriptions
    pub fn get_capability_descriptions(&self) -> &Vec<String> {
        &self.capability_descriptions
    }

    /// Sets the value of CompressionMethod
    pub fn set_compression_method(&mut self, value: String) {
        self.compression_method = Some(value);
    }

    /// Gets the value of CompressionMethod
    pub fn get_compression_method(&self) -> Option<&String> {
        self.compression_method.as_ref()
    }

    /// Sets the value of DefaultBlockSize
    pub fn set_default_block_size(&mut self, value: u64) {
        self.default_block_size = Some(value);
    }

    /// Gets the value of DefaultBlockSize
    pub fn get_default_block_size(&self) -> Option<&u64> {
        self.default_block_size.as_ref()
    }

    /// Sets the value of ErrorMethodology
    pub fn set_error_methodology(&mut self, value: String) {
        self.error_methodology = Some(value);
    }

    /// Gets the value of ErrorMethodology
    pub fn get_error_methodology(&self) -> Option<&String> {
        self.error_methodology.as_ref()
    }

    /// Sets the value of LastCleaned
    pub fn set_last_cleaned(&mut self, value: String) {
        self.last_cleaned = Some(value);
    }

    /// Gets the value of LastCleaned
    pub fn get_last_cleaned(&self) -> Option<&String> {
        self.last_cleaned.as_ref()
    }

    /// Sets the value of LoadTime
    pub fn set_load_time(&mut self, value: u64) {
        self.load_time = Some(value);
    }

    /// Gets the value of LoadTime
    pub fn get_load_time(&self) -> Option<&u64> {
        self.load_time.as_ref()
    }

    /// Sets the value of MaxAccessTime
    pub fn set_max_access_time(&mut self, value: u64) {
        self.max_access_time = Some(value);
    }

    /// Gets the value of MaxAccessTime
    pub fn get_max_access_time(&self) -> Option<&u64> {
        self.max_access_time.as_ref()
    }

    /// Sets the value of MaxBlockSize
    pub fn set_max_block_size(&mut self, value: u64) {
        self.max_block_size = Some(value);
    }

    /// Gets the value of MaxBlockSize
    pub fn get_max_block_size(&self) -> Option<&u64> {
        self.max_block_size.as_ref()
    }

    /// Sets the value of MaxMediaSize
    pub fn set_max_media_size(&mut self, value: u64) {
        self.max_media_size = Some(value);
    }

    /// Gets the value of MaxMediaSize
    pub fn get_max_media_size(&self) -> Option<&u64> {
        self.max_media_size.as_ref()
    }

    /// Sets the value of MaxUnitsBeforeCleaning
    pub fn set_max_units_before_cleaning(&mut self, value: u64) {
        self.max_units_before_cleaning = Some(value);
    }

    /// Gets the value of MaxUnitsBeforeCleaning
    pub fn get_max_units_before_cleaning(&self) -> Option<&u64> {
        self.max_units_before_cleaning.as_ref()
    }

    /// Sets the value of MediaIsLocked
    pub fn set_media_is_locked(&mut self, value: bool) {
        self.media_is_locked = Some(value);
    }

    /// Gets the value of MediaIsLocked
    pub fn get_media_is_locked(&self) -> Option<&bool> {
        self.media_is_locked.as_ref()
    }

    /// Sets the value of MinBlockSize
    pub fn set_min_block_size(&mut self, value: u64) {
        self.min_block_size = Some(value);
    }

    /// Gets the value of MinBlockSize
    pub fn get_min_block_size(&self) -> Option<&u64> {
        self.min_block_size.as_ref()
    }

    /// Sets the value of MountCount
    pub fn set_mount_count(&mut self, value: u64) {
        self.mount_count = Some(value);
    }

    /// Gets the value of MountCount
    pub fn get_mount_count(&self) -> Option<&u64> {
        self.mount_count.as_ref()
    }

    /// Sets the value of NeedsCleaning
    pub fn set_needs_cleaning(&mut self, value: bool) {
        self.needs_cleaning = Some(value);
    }

    /// Gets the value of NeedsCleaning
    pub fn get_needs_cleaning(&self) -> Option<&bool> {
        self.needs_cleaning.as_ref()
    }

    /// Sets the value of NumberOfMediaSupported
    pub fn set_number_of_media_supported(&mut self, value: u32) {
        self.number_of_media_supported = Some(value);
    }

    /// Gets the value of NumberOfMediaSupported
    pub fn get_number_of_media_supported(&self) -> Option<&u32> {
        self.number_of_media_supported.as_ref()
    }

    /// Sets the value of Security
    pub fn set_security(&mut self, value: MediaAccessDevice_Security) {
        self.security = Some(value);
    }

    /// Gets the value of Security
    pub fn get_security(&self) -> Option<&MediaAccessDevice_Security> {
        self.security.as_ref()
    }

    /// Sets the value of TimeOfLastMount
    pub fn set_time_of_last_mount(&mut self, value: String) {
        self.time_of_last_mount = Some(value);
    }

    /// Gets the value of TimeOfLastMount
    pub fn get_time_of_last_mount(&self) -> Option<&String> {
        self.time_of_last_mount.as_ref()
    }

    /// Sets the value of TotalMountTime
    pub fn set_total_mount_time(&mut self, value: u64) {
        self.total_mount_time = Some(value);
    }

    /// Gets the value of TotalMountTime
    pub fn get_total_mount_time(&self) -> Option<&u64> {
        self.total_mount_time.as_ref()
    }

    /// Sets the value of UncompressedDataRate
    pub fn set_uncompressed_data_rate(&mut self, value: u32) {
        self.uncompressed_data_rate = Some(value);
    }

    /// Gets the value of UncompressedDataRate
    pub fn get_uncompressed_data_rate(&self) -> Option<&u32> {
        self.uncompressed_data_rate.as_ref()
    }

    /// Sets the value of UnitsDescription
    pub fn set_units_description(&mut self, value: String) {
        self.units_description = Some(value);
    }

    /// Gets the value of UnitsDescription
    pub fn get_units_description(&self) -> Option<&String> {
        self.units_description.as_ref()
    }

    /// Sets the value of UnitsUsed
    pub fn set_units_used(&mut self, value: u64) {
        self.units_used = Some(value);
    }

    /// Gets the value of UnitsUsed
    pub fn get_units_used(&self) -> Option<&u64> {
        self.units_used.as_ref()
    }

    /// Sets the value of UnloadTime
    pub fn set_unload_time(&mut self, value: u64) {
        self.unload_time = Some(value);
    }

    /// Gets the value of UnloadTime
    pub fn get_unload_time(&self) -> Option<&u64> {
        self.unload_time.as_ref()
    }

/// Method to lock and unlock the media in a removeable Access Device. The method takes one parameter as input - a boolean indicating whether to lock or unlock. TRUE indicates that the media should be locked in the Device, FALSE indicates that the media should be unlocked. The method returns 0 if successful, 1 if not supported, and any other value if an error occurred. The set of possible return codes should be specified in a ValueMap qualifier on the method. The strings to which the ValueMap contents are 'translated' should be specified as a Values array qualifier on the method.

    /// * `lock` - If TRUE, lock the media. If FALSE release the media. (bool)

    /// * `return_value` -  (u32)
    pub fn lock_media(&self, lock: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Lock".to_string(), value: lock.into() });
        self.invoke_method("LockMedia", &args)

    }

}

