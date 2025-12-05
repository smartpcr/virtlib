// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OperatingSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OperatingSystem {
    #[serde(flatten)]
    pub base: CIM_OperatingSystem,

/// 
    #[serde(rename = "BootDevice")]
    pub boot_device: Option<String>,

/// 
    #[serde(rename = "BuildNumber")]
    pub build_number: Option<String>,

/// 
    #[serde(rename = "BuildType")]
    pub build_type: Option<String>,

/// 
    #[serde(rename = "CodeSet")]
    pub code_set: Option<String>,

/// 
    #[serde(rename = "CountryCode")]
    pub country_code: Option<String>,

/// 
    #[serde(rename = "CSDVersion")]
    pub csdversion: Option<String>,

/// 
    #[serde(rename = "DataExecutionPrevention_32BitApplications")]
    pub data_execution_prevention_32_bit_applications: Option<bool>,

/// 
    #[serde(rename = "DataExecutionPrevention_Available")]
    pub data_execution_prevention__available: Option<bool>,

/// 
    #[serde(rename = "DataExecutionPrevention_Drivers")]
    pub data_execution_prevention__drivers: Option<bool>,

/// 
    #[serde(rename = "DataExecutionPrevention_SupportPolicy")]
    pub data_execution_prevention__support_policy: Option<u8>,

/// 
    #[serde(rename = "Debug")]
    pub debug: Option<bool>,

/// 
    #[serde(rename = "EncryptionLevel")]
    pub encryption_level: Option<u32>,

/// 
    #[serde(rename = "ForegroundApplicationBoost")]
    pub foreground_application_boost: Option<u8>,

/// 
    #[serde(rename = "LargeSystemCache")]
    pub large_system_cache: Option<u32>,

/// 
    #[serde(rename = "Locale")]
    pub locale: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MUILanguages")]
    pub muilanguages: Vec<String>,

/// 
    #[serde(rename = "OperatingSystemSKU")]
    pub operating_system_sku: Option<u32>,

/// 
    #[serde(rename = "Organization")]
    pub organization: Option<String>,

/// 
    #[serde(rename = "OSArchitecture")]
    pub osarchitecture: Option<String>,

/// 
    #[serde(rename = "OSLanguage")]
    pub oslanguage: Option<u32>,

/// 
    #[serde(rename = "OSProductSuite")]
    pub osproduct_suite: Option<u32>,

/// 
    #[serde(rename = "PAEEnabled")]
    pub paeenabled: Option<bool>,

/// 
    #[serde(rename = "PlusProductID")]
    pub plus_product_id: Option<String>,

/// 
    #[serde(rename = "PlusVersionNumber")]
    pub plus_version_number: Option<String>,

/// 
    #[serde(rename = "PortableOperatingSystem")]
    pub portable_operating_system: Option<bool>,

/// 
    #[serde(rename = "Primary")]
    pub primary: Option<bool>,

/// 
    #[serde(rename = "ProductType")]
    pub product_type: Option<u32>,

/// 
    #[serde(rename = "RegisteredUser")]
    pub registered_user: Option<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "ServicePackMajorVersion")]
    pub service_pack_major_version: Option<u16>,

/// 
    #[serde(rename = "ServicePackMinorVersion")]
    pub service_pack_minor_version: Option<u16>,

/// 
    #[serde(rename = "SuiteMask")]
    pub suite_mask: Option<u32>,

/// 
    #[serde(rename = "SystemDevice")]
    pub system_device: Option<String>,

/// 
    #[serde(rename = "SystemDirectory")]
    pub system_directory: Option<String>,

/// 
    #[serde(rename = "SystemDrive")]
    pub system_drive: Option<String>,

/// 
    #[serde(rename = "WindowsDirectory")]
    pub windows_directory: Option<String>,
}

impl Win32_OperatingSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_OperatingSystem::new(),
            boot_device: None,
            build_number: None,
            build_type: None,
            code_set: None,
            country_code: None,
            csdversion: None,
            data_execution_prevention_32_bit_applications: None,
            data_execution_prevention__available: None,
            data_execution_prevention__drivers: None,
            data_execution_prevention__support_policy: None,
            debug: None,
            encryption_level: None,
            foreground_application_boost: None,
            large_system_cache: None,
            locale: None,
            manufacturer: None,
            muilanguages: Vec::new(),
            operating_system_sku: None,
            organization: None,
            osarchitecture: None,
            oslanguage: None,
            osproduct_suite: None,
            paeenabled: None,
            plus_product_id: None,
            plus_version_number: None,
            portable_operating_system: None,
            primary: None,
            product_type: None,
            registered_user: None,
            serial_number: None,
            service_pack_major_version: None,
            service_pack_minor_version: None,
            suite_mask: None,
            system_device: None,
            system_directory: None,
            system_drive: None,
            windows_directory: None,
        }
    }


    /// Sets the value of BootDevice
    pub fn set_boot_device(&mut self, value: String) {
        self.boot_device = Some(value);
    }

    /// Gets the value of BootDevice
    pub fn get_boot_device(&self) -> Option<&String> {
        self.boot_device.as_ref()
    }

    /// Sets the value of BuildNumber
    pub fn set_build_number(&mut self, value: String) {
        self.build_number = Some(value);
    }

    /// Gets the value of BuildNumber
    pub fn get_build_number(&self) -> Option<&String> {
        self.build_number.as_ref()
    }

    /// Sets the value of BuildType
    pub fn set_build_type(&mut self, value: String) {
        self.build_type = Some(value);
    }

    /// Gets the value of BuildType
    pub fn get_build_type(&self) -> Option<&String> {
        self.build_type.as_ref()
    }

    /// Sets the value of CodeSet
    pub fn set_code_set(&mut self, value: String) {
        self.code_set = Some(value);
    }

    /// Gets the value of CodeSet
    pub fn get_code_set(&self) -> Option<&String> {
        self.code_set.as_ref()
    }

    /// Sets the value of CountryCode
    pub fn set_country_code(&mut self, value: String) {
        self.country_code = Some(value);
    }

    /// Gets the value of CountryCode
    pub fn get_country_code(&self) -> Option<&String> {
        self.country_code.as_ref()
    }

    /// Sets the value of CSDVersion
    pub fn set_csdversion(&mut self, value: String) {
        self.csdversion = Some(value);
    }

    /// Gets the value of CSDVersion
    pub fn get_csdversion(&self) -> Option<&String> {
        self.csdversion.as_ref()
    }

    /// Sets the value of DataExecutionPrevention_32BitApplications
    pub fn set_data_execution_prevention_32_bit_applications(&mut self, value: bool) {
        self.data_execution_prevention_32_bit_applications = Some(value);
    }

    /// Gets the value of DataExecutionPrevention_32BitApplications
    pub fn get_data_execution_prevention_32_bit_applications(&self) -> Option<&bool> {
        self.data_execution_prevention_32_bit_applications.as_ref()
    }

    /// Sets the value of DataExecutionPrevention_Available
    pub fn set_data_execution_prevention__available(&mut self, value: bool) {
        self.data_execution_prevention__available = Some(value);
    }

    /// Gets the value of DataExecutionPrevention_Available
    pub fn get_data_execution_prevention__available(&self) -> Option<&bool> {
        self.data_execution_prevention__available.as_ref()
    }

    /// Sets the value of DataExecutionPrevention_Drivers
    pub fn set_data_execution_prevention__drivers(&mut self, value: bool) {
        self.data_execution_prevention__drivers = Some(value);
    }

    /// Gets the value of DataExecutionPrevention_Drivers
    pub fn get_data_execution_prevention__drivers(&self) -> Option<&bool> {
        self.data_execution_prevention__drivers.as_ref()
    }

    /// Sets the value of DataExecutionPrevention_SupportPolicy
    pub fn set_data_execution_prevention__support_policy(&mut self, value: u8) {
        self.data_execution_prevention__support_policy = Some(value);
    }

    /// Gets the value of DataExecutionPrevention_SupportPolicy
    pub fn get_data_execution_prevention__support_policy(&self) -> Option<&u8> {
        self.data_execution_prevention__support_policy.as_ref()
    }

    /// Sets the value of Debug
    pub fn set_debug(&mut self, value: bool) {
        self.debug = Some(value);
    }

    /// Gets the value of Debug
    pub fn get_debug(&self) -> Option<&bool> {
        self.debug.as_ref()
    }

    /// Sets the value of EncryptionLevel
    pub fn set_encryption_level(&mut self, value: u32) {
        self.encryption_level = Some(value);
    }

    /// Gets the value of EncryptionLevel
    pub fn get_encryption_level(&self) -> Option<&u32> {
        self.encryption_level.as_ref()
    }

    /// Sets the value of ForegroundApplicationBoost
    pub fn set_foreground_application_boost(&mut self, value: u8) {
        self.foreground_application_boost = Some(value);
    }

    /// Gets the value of ForegroundApplicationBoost
    pub fn get_foreground_application_boost(&self) -> Option<&u8> {
        self.foreground_application_boost.as_ref()
    }

    /// Sets the value of LargeSystemCache
    pub fn set_large_system_cache(&mut self, value: u32) {
        self.large_system_cache = Some(value);
    }

    /// Gets the value of LargeSystemCache
    pub fn get_large_system_cache(&self) -> Option<&u32> {
        self.large_system_cache.as_ref()
    }

    /// Sets the value of Locale
    pub fn set_locale(&mut self, value: String) {
        self.locale = Some(value);
    }

    /// Gets the value of Locale
    pub fn get_locale(&self) -> Option<&String> {
        self.locale.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MUILanguages
    pub fn set_muilanguages(&mut self, value: Vec<String>) {
        self.muilanguages = value;
    }

    /// Gets the value of MUILanguages
    pub fn get_muilanguages(&self) -> &Vec<String> {
        &self.muilanguages
    }

    /// Sets the value of OperatingSystemSKU
    pub fn set_operating_system_sku(&mut self, value: u32) {
        self.operating_system_sku = Some(value);
    }

    /// Gets the value of OperatingSystemSKU
    pub fn get_operating_system_sku(&self) -> Option<&u32> {
        self.operating_system_sku.as_ref()
    }

    /// Sets the value of Organization
    pub fn set_organization(&mut self, value: String) {
        self.organization = Some(value);
    }

    /// Gets the value of Organization
    pub fn get_organization(&self) -> Option<&String> {
        self.organization.as_ref()
    }

    /// Sets the value of OSArchitecture
    pub fn set_osarchitecture(&mut self, value: String) {
        self.osarchitecture = Some(value);
    }

    /// Gets the value of OSArchitecture
    pub fn get_osarchitecture(&self) -> Option<&String> {
        self.osarchitecture.as_ref()
    }

    /// Sets the value of OSLanguage
    pub fn set_oslanguage(&mut self, value: u32) {
        self.oslanguage = Some(value);
    }

    /// Gets the value of OSLanguage
    pub fn get_oslanguage(&self) -> Option<&u32> {
        self.oslanguage.as_ref()
    }

    /// Sets the value of OSProductSuite
    pub fn set_osproduct_suite(&mut self, value: u32) {
        self.osproduct_suite = Some(value);
    }

    /// Gets the value of OSProductSuite
    pub fn get_osproduct_suite(&self) -> Option<&u32> {
        self.osproduct_suite.as_ref()
    }

    /// Sets the value of PAEEnabled
    pub fn set_paeenabled(&mut self, value: bool) {
        self.paeenabled = Some(value);
    }

    /// Gets the value of PAEEnabled
    pub fn get_paeenabled(&self) -> Option<&bool> {
        self.paeenabled.as_ref()
    }

    /// Sets the value of PlusProductID
    pub fn set_plus_product_id(&mut self, value: String) {
        self.plus_product_id = Some(value);
    }

    /// Gets the value of PlusProductID
    pub fn get_plus_product_id(&self) -> Option<&String> {
        self.plus_product_id.as_ref()
    }

    /// Sets the value of PlusVersionNumber
    pub fn set_plus_version_number(&mut self, value: String) {
        self.plus_version_number = Some(value);
    }

    /// Gets the value of PlusVersionNumber
    pub fn get_plus_version_number(&self) -> Option<&String> {
        self.plus_version_number.as_ref()
    }

    /// Sets the value of PortableOperatingSystem
    pub fn set_portable_operating_system(&mut self, value: bool) {
        self.portable_operating_system = Some(value);
    }

    /// Gets the value of PortableOperatingSystem
    pub fn get_portable_operating_system(&self) -> Option<&bool> {
        self.portable_operating_system.as_ref()
    }

    /// Sets the value of Primary
    pub fn set_primary(&mut self, value: bool) {
        self.primary = Some(value);
    }

    /// Gets the value of Primary
    pub fn get_primary(&self) -> Option<&bool> {
        self.primary.as_ref()
    }

    /// Sets the value of ProductType
    pub fn set_product_type(&mut self, value: u32) {
        self.product_type = Some(value);
    }

    /// Gets the value of ProductType
    pub fn get_product_type(&self) -> Option<&u32> {
        self.product_type.as_ref()
    }

    /// Sets the value of RegisteredUser
    pub fn set_registered_user(&mut self, value: String) {
        self.registered_user = Some(value);
    }

    /// Gets the value of RegisteredUser
    pub fn get_registered_user(&self) -> Option<&String> {
        self.registered_user.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of ServicePackMajorVersion
    pub fn set_service_pack_major_version(&mut self, value: u16) {
        self.service_pack_major_version = Some(value);
    }

    /// Gets the value of ServicePackMajorVersion
    pub fn get_service_pack_major_version(&self) -> Option<&u16> {
        self.service_pack_major_version.as_ref()
    }

    /// Sets the value of ServicePackMinorVersion
    pub fn set_service_pack_minor_version(&mut self, value: u16) {
        self.service_pack_minor_version = Some(value);
    }

    /// Gets the value of ServicePackMinorVersion
    pub fn get_service_pack_minor_version(&self) -> Option<&u16> {
        self.service_pack_minor_version.as_ref()
    }

    /// Sets the value of SuiteMask
    pub fn set_suite_mask(&mut self, value: u32) {
        self.suite_mask = Some(value);
    }

    /// Gets the value of SuiteMask
    pub fn get_suite_mask(&self) -> Option<&u32> {
        self.suite_mask.as_ref()
    }

    /// Sets the value of SystemDevice
    pub fn set_system_device(&mut self, value: String) {
        self.system_device = Some(value);
    }

    /// Gets the value of SystemDevice
    pub fn get_system_device(&self) -> Option<&String> {
        self.system_device.as_ref()
    }

    /// Sets the value of SystemDirectory
    pub fn set_system_directory(&mut self, value: String) {
        self.system_directory = Some(value);
    }

    /// Gets the value of SystemDirectory
    pub fn get_system_directory(&self) -> Option<&String> {
        self.system_directory.as_ref()
    }

    /// Sets the value of SystemDrive
    pub fn set_system_drive(&mut self, value: String) {
        self.system_drive = Some(value);
    }

    /// Gets the value of SystemDrive
    pub fn get_system_drive(&self) -> Option<&String> {
        self.system_drive.as_ref()
    }

    /// Sets the value of WindowsDirectory
    pub fn set_windows_directory(&mut self, value: String) {
        self.windows_directory = Some(value);
    }

    /// Gets the value of WindowsDirectory
    pub fn get_windows_directory(&self) -> Option<&String> {
        self.windows_directory.as_ref()
    }

/// 

    /// * `flags` -  (i32)
    /// * `reserved` -  (i32)

    /// * `return_value` -  (u32)
    pub fn win32_shutdown(&self, flags: i32, reserved: i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "Reserved".to_string(), value: reserved.into() });
        self.invoke_method("Win32Shutdown", &args)

    }


/// 

    /// * `comment` -  (String)
    /// * `flags` -  (i32)
    /// * `reason_code` -  (u32)
    /// * `timeout` -  (u32)

    /// * `return_value` -  (u32)
    pub fn win32_shutdown_tracker(&self, timeout: u32, comment: &String, reason_code: u32, flags: i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });
        args.push(MethodParameter { name: "Comment".to_string(), value: comment.into() });
        args.push(MethodParameter { name: "ReasonCode".to_string(), value: reason_code.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("Win32ShutdownTracker", &args)

    }


/// 

    /// * `local_date_time` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_date_time(&self, local_date_time: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LocalDateTime".to_string(), value: local_date_time.into() });
        self.invoke_method("SetDateTime", &args)

    }

}

