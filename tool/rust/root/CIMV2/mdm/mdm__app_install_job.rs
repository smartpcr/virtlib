// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_AppInstallJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_AppInstallJob {

/// 
    #[serde(rename = "ActionType")]
    pub action_type: Option<u32>,

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "Dependencies")]
    pub dependencies: Vec<String>,

/// 
    #[serde(rename = "DependencyUrlLists")]
    pub dependency_url_lists: Vec<String>,

/// 
    #[serde(rename = "DeploymentOptions")]
    pub deployment_options: Option<u32>,

/// 
    #[serde(rename = "DownloadUrlList")]
    pub download_url_list: Vec<String>,

/// 
    #[serde(rename = "JobID")]
    pub job_id: Option<String>,

/// 
    #[serde(rename = "JobType")]
    pub job_type: Option<u32>,

/// 
    #[serde(rename = "LastError")]
    pub last_error: Option<u32>,

/// 
    #[serde(rename = "LicenseXml")]
    pub license_xml: Option<String>,

/// 
    #[serde(rename = "PackageFullName")]
    pub package_full_name: Option<String>,

/// 
    #[serde(rename = "Progress")]
    pub progress: Option<u32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl MDM_AppInstallJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            action_type: None,
            creation_time: None,
            dependencies: Vec::new(),
            dependency_url_lists: Vec::new(),
            deployment_options: None,
            download_url_list: Vec::new(),
            job_id: None,
            job_type: None,
            last_error: None,
            license_xml: None,
            package_full_name: None,
            progress: None,
            status: None,
        }
    }


    /// Sets the value of ActionType
    pub fn set_action_type(&mut self, value: u32) {
        self.action_type = Some(value);
    }

    /// Gets the value of ActionType
    pub fn get_action_type(&self) -> Option<&u32> {
        self.action_type.as_ref()
    }

    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of Dependencies
    pub fn set_dependencies(&mut self, value: Vec<String>) {
        self.dependencies = value;
    }

    /// Gets the value of Dependencies
    pub fn get_dependencies(&self) -> &Vec<String> {
        &self.dependencies
    }

    /// Sets the value of DependencyUrlLists
    pub fn set_dependency_url_lists(&mut self, value: Vec<String>) {
        self.dependency_url_lists = value;
    }

    /// Gets the value of DependencyUrlLists
    pub fn get_dependency_url_lists(&self) -> &Vec<String> {
        &self.dependency_url_lists
    }

    /// Sets the value of DeploymentOptions
    pub fn set_deployment_options(&mut self, value: u32) {
        self.deployment_options = Some(value);
    }

    /// Gets the value of DeploymentOptions
    pub fn get_deployment_options(&self) -> Option<&u32> {
        self.deployment_options.as_ref()
    }

    /// Sets the value of DownloadUrlList
    pub fn set_download_url_list(&mut self, value: Vec<String>) {
        self.download_url_list = value;
    }

    /// Gets the value of DownloadUrlList
    pub fn get_download_url_list(&self) -> &Vec<String> {
        &self.download_url_list
    }

    /// Sets the value of JobID
    pub fn set_job_id(&mut self, value: String) {
        self.job_id = Some(value);
    }

    /// Gets the value of JobID
    pub fn get_job_id(&self) -> Option<&String> {
        self.job_id.as_ref()
    }

    /// Sets the value of JobType
    pub fn set_job_type(&mut self, value: u32) {
        self.job_type = Some(value);
    }

    /// Gets the value of JobType
    pub fn get_job_type(&self) -> Option<&u32> {
        self.job_type.as_ref()
    }

    /// Sets the value of LastError
    pub fn set_last_error(&mut self, value: u32) {
        self.last_error = Some(value);
    }

    /// Gets the value of LastError
    pub fn get_last_error(&self) -> Option<&u32> {
        self.last_error.as_ref()
    }

    /// Sets the value of LicenseXml
    pub fn set_license_xml(&mut self, value: String) {
        self.license_xml = Some(value);
    }

    /// Gets the value of LicenseXml
    pub fn get_license_xml(&self) -> Option<&String> {
        self.license_xml.as_ref()
    }

    /// Sets the value of PackageFullName
    pub fn set_package_full_name(&mut self, value: String) {
        self.package_full_name = Some(value);
    }

    /// Gets the value of PackageFullName
    pub fn get_package_full_name(&self) -> Option<&String> {
        self.package_full_name.as_ref()
    }

    /// Sets the value of Progress
    pub fn set_progress(&mut self, value: u32) {
        self.progress = Some(value);
    }

    /// Gets the value of Progress
    pub fn get_progress(&self) -> Option<&u32> {
        self.progress.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

/// 

    /// * `job_data` -  (String)

    /// * `return_value` -  (u32)
    pub fn create_job(&self, job_data: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "JobData".to_string(), value: job_data.into() });
        self.invoke_method("CreateJob", &args)

    }

}

