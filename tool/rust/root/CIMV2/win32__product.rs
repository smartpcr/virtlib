// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Product struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Product {
    #[serde(flatten)]
    pub base: CIM_Product,

/// 
    #[serde(rename = "AssignmentType")]
    pub assignment_type: Option<u16>,

/// 
    #[serde(rename = "HelpLink")]
    pub help_link: Option<String>,

/// 
    #[serde(rename = "HelpTelephone")]
    pub help_telephone: Option<String>,

/// 
    #[serde(rename = "InstallDate")]
    pub install_date: Option<String>,

/// 
    #[serde(rename = "InstallDate2")]
    pub install_date2: Option<String>,

/// 
    #[serde(rename = "InstallLocation")]
    pub install_location: Option<String>,

/// 
    #[serde(rename = "InstallSource")]
    pub install_source: Option<String>,

/// 
    #[serde(rename = "InstallState")]
    pub install_state: Option<i16>,

/// 
    #[serde(rename = "Language")]
    pub language: Option<String>,

/// 
    #[serde(rename = "LocalPackage")]
    pub local_package: Option<String>,

/// 
    #[serde(rename = "PackageCache")]
    pub package_cache: Option<String>,

/// 
    #[serde(rename = "PackageCode")]
    pub package_code: Option<String>,

/// 
    #[serde(rename = "PackageName")]
    pub package_name: Option<String>,

/// 
    #[serde(rename = "ProductID")]
    pub product_id: Option<String>,

/// 
    #[serde(rename = "RegCompany")]
    pub reg_company: Option<String>,

/// 
    #[serde(rename = "RegOwner")]
    pub reg_owner: Option<String>,

/// 
    #[serde(rename = "Transforms")]
    pub transforms: Option<String>,

/// 
    #[serde(rename = "URLInfoAbout")]
    pub urlinfo_about: Option<String>,

/// 
    #[serde(rename = "URLUpdateInfo")]
    pub urlupdate_info: Option<String>,

/// 
    #[serde(rename = "WordCount")]
    pub word_count: Option<u32>,
}

impl Win32_Product {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Product::new(),
            assignment_type: None,
            help_link: None,
            help_telephone: None,
            install_date: None,
            install_date2: None,
            install_location: None,
            install_source: None,
            install_state: None,
            language: None,
            local_package: None,
            package_cache: None,
            package_code: None,
            package_name: None,
            product_id: None,
            reg_company: None,
            reg_owner: None,
            transforms: None,
            urlinfo_about: None,
            urlupdate_info: None,
            word_count: None,
        }
    }


    /// Sets the value of AssignmentType
    pub fn set_assignment_type(&mut self, value: u16) {
        self.assignment_type = Some(value);
    }

    /// Gets the value of AssignmentType
    pub fn get_assignment_type(&self) -> Option<&u16> {
        self.assignment_type.as_ref()
    }

    /// Sets the value of HelpLink
    pub fn set_help_link(&mut self, value: String) {
        self.help_link = Some(value);
    }

    /// Gets the value of HelpLink
    pub fn get_help_link(&self) -> Option<&String> {
        self.help_link.as_ref()
    }

    /// Sets the value of HelpTelephone
    pub fn set_help_telephone(&mut self, value: String) {
        self.help_telephone = Some(value);
    }

    /// Gets the value of HelpTelephone
    pub fn get_help_telephone(&self) -> Option<&String> {
        self.help_telephone.as_ref()
    }

    /// Sets the value of InstallDate
    pub fn set_install_date(&mut self, value: String) {
        self.install_date = Some(value);
    }

    /// Gets the value of InstallDate
    pub fn get_install_date(&self) -> Option<&String> {
        self.install_date.as_ref()
    }

    /// Sets the value of InstallDate2
    pub fn set_install_date2(&mut self, value: String) {
        self.install_date2 = Some(value);
    }

    /// Gets the value of InstallDate2
    pub fn get_install_date2(&self) -> Option<&String> {
        self.install_date2.as_ref()
    }

    /// Sets the value of InstallLocation
    pub fn set_install_location(&mut self, value: String) {
        self.install_location = Some(value);
    }

    /// Gets the value of InstallLocation
    pub fn get_install_location(&self) -> Option<&String> {
        self.install_location.as_ref()
    }

    /// Sets the value of InstallSource
    pub fn set_install_source(&mut self, value: String) {
        self.install_source = Some(value);
    }

    /// Gets the value of InstallSource
    pub fn get_install_source(&self) -> Option<&String> {
        self.install_source.as_ref()
    }

    /// Sets the value of InstallState
    pub fn set_install_state(&mut self, value: i16) {
        self.install_state = Some(value);
    }

    /// Gets the value of InstallState
    pub fn get_install_state(&self) -> Option<&i16> {
        self.install_state.as_ref()
    }

    /// Sets the value of Language
    pub fn set_language(&mut self, value: String) {
        self.language = Some(value);
    }

    /// Gets the value of Language
    pub fn get_language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Sets the value of LocalPackage
    pub fn set_local_package(&mut self, value: String) {
        self.local_package = Some(value);
    }

    /// Gets the value of LocalPackage
    pub fn get_local_package(&self) -> Option<&String> {
        self.local_package.as_ref()
    }

    /// Sets the value of PackageCache
    pub fn set_package_cache(&mut self, value: String) {
        self.package_cache = Some(value);
    }

    /// Gets the value of PackageCache
    pub fn get_package_cache(&self) -> Option<&String> {
        self.package_cache.as_ref()
    }

    /// Sets the value of PackageCode
    pub fn set_package_code(&mut self, value: String) {
        self.package_code = Some(value);
    }

    /// Gets the value of PackageCode
    pub fn get_package_code(&self) -> Option<&String> {
        self.package_code.as_ref()
    }

    /// Sets the value of PackageName
    pub fn set_package_name(&mut self, value: String) {
        self.package_name = Some(value);
    }

    /// Gets the value of PackageName
    pub fn get_package_name(&self) -> Option<&String> {
        self.package_name.as_ref()
    }

    /// Sets the value of ProductID
    pub fn set_product_id(&mut self, value: String) {
        self.product_id = Some(value);
    }

    /// Gets the value of ProductID
    pub fn get_product_id(&self) -> Option<&String> {
        self.product_id.as_ref()
    }

    /// Sets the value of RegCompany
    pub fn set_reg_company(&mut self, value: String) {
        self.reg_company = Some(value);
    }

    /// Gets the value of RegCompany
    pub fn get_reg_company(&self) -> Option<&String> {
        self.reg_company.as_ref()
    }

    /// Sets the value of RegOwner
    pub fn set_reg_owner(&mut self, value: String) {
        self.reg_owner = Some(value);
    }

    /// Gets the value of RegOwner
    pub fn get_reg_owner(&self) -> Option<&String> {
        self.reg_owner.as_ref()
    }

    /// Sets the value of Transforms
    pub fn set_transforms(&mut self, value: String) {
        self.transforms = Some(value);
    }

    /// Gets the value of Transforms
    pub fn get_transforms(&self) -> Option<&String> {
        self.transforms.as_ref()
    }

    /// Sets the value of URLInfoAbout
    pub fn set_urlinfo_about(&mut self, value: String) {
        self.urlinfo_about = Some(value);
    }

    /// Gets the value of URLInfoAbout
    pub fn get_urlinfo_about(&self) -> Option<&String> {
        self.urlinfo_about.as_ref()
    }

    /// Sets the value of URLUpdateInfo
    pub fn set_urlupdate_info(&mut self, value: String) {
        self.urlupdate_info = Some(value);
    }

    /// Gets the value of URLUpdateInfo
    pub fn get_urlupdate_info(&self) -> Option<&String> {
        self.urlupdate_info.as_ref()
    }

    /// Sets the value of WordCount
    pub fn set_word_count(&mut self, value: u32) {
        self.word_count = Some(value);
    }

    /// Gets the value of WordCount
    pub fn get_word_count(&self) -> Option<&u32> {
        self.word_count.as_ref()
    }

/// 

    /// * `all_users` -  (bool)
    /// * `options` -  (String)
    /// * `package_location` -  (String)

    /// * `return_value` -  (u32)
    pub fn install(&self, package_location: &String, options: &String, all_users: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PackageLocation".to_string(), value: package_location.into() });
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });
        args.push(MethodParameter { name: "AllUsers".to_string(), value: all_users.into() });
        self.invoke_method("Install", &args)

    }


/// 

    /// * `options` -  (String)
    /// * `package_location` -  (String)
    /// * `target_location` -  (String)

    /// * `return_value` -  (u32)
    pub fn admin(&self, package_location: &String, target_location: &String, options: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PackageLocation".to_string(), value: package_location.into() });
        args.push(MethodParameter { name: "TargetLocation".to_string(), value: target_location.into() });
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });
        self.invoke_method("Admin", &args)

    }


/// 

    /// * `all_users` -  (bool)
    /// * `options` -  (String)
    /// * `package_location` -  (String)

    /// * `return_value` -  (u32)
    pub fn advertise(&self, package_location: &String, options: &String, all_users: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PackageLocation".to_string(), value: package_location.into() });
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });
        args.push(MethodParameter { name: "AllUsers".to_string(), value: all_users.into() });
        self.invoke_method("Advertise", &args)

    }


/// 

    /// * `reinstall_mode` -  (u16)

    /// * `return_value` -  (u32)
    pub fn reinstall(&self, reinstall_mode: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReinstallMode".to_string(), value: reinstall_mode.into() });
        self.invoke_method("Reinstall", &args)

    }


/// 

    /// * `options` -  (String)
    /// * `package_location` -  (String)

    /// * `return_value` -  (u32)
    pub fn upgrade(&self, package_location: &String, options: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PackageLocation".to_string(), value: package_location.into() });
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });
        self.invoke_method("Upgrade", &args)

    }


/// 

    /// * `install_level` -  (u16)
    /// * `install_state` -  (u16)
    /// * `options` -  (String)

    /// * `return_value` -  (u32)
    pub fn configure(&self, install_state: u16, install_level: u16, options: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InstallState".to_string(), value: install_state.into() });
        args.push(MethodParameter { name: "InstallLevel".to_string(), value: install_level.into() });
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });
        self.invoke_method("Configure", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn uninstall(&self) -> Result<(), WmiError> {
        self.invoke_method("Uninstall", &[])

    }

}

