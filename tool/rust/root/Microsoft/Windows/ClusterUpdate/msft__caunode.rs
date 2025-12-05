// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAUNode struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAUNode {

/// 
    #[serde(rename = "OrchestratorGuid")]
    pub orchestrator_guid: Option<String>,
}

impl MSFT_CAUNode {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            orchestrator_guid: None,
        }
    }


    /// Sets the value of OrchestratorGuid
    pub fn set_orchestrator_guid(&mut self, value: String) {
        self.orchestrator_guid = Some(value);
    }

    /// Gets the value of OrchestratorGuid
    pub fn get_orchestrator_guid(&self) -> Option<&String> {
        self.orchestrator_guid.as_ref()
    }

/// 

    /// * `return_value` -  (bool)
    pub fn reboot_required(&self) -> Result<(), WmiError> {
        self.invoke_method("RebootRequired", &[])

    }


/// 

    /// * `feature_updates_only` -  (bool)
    /// * `include_recommended_updates` -  (bool)
    /// * `query_string` -  (String)

    /// * `info` -  (MSFT_CAU_ScanUpdateInfo[])
    /// * `return_value` -  (u32)
    pub fn scan_updates(&self, query_string: &String, include_recommended_updates: bool, feature_updates_only: bool, info: &mut Vec<MSFT_CAU_ScanUpdateInfo>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "QueryString".to_string(), value: query_string.into() });
        args.push(MethodParameter { name: "IncludeRecommendedUpdates".to_string(), value: include_recommended_updates.into() });
        args.push(MethodParameter { name: "FeatureUpdatesOnly".to_string(), value: feature_updates_only.into() });

        let result = self.invoke_method("ScanUpdates", &args)?;
        let info = result.get_value("Info")?;
        Ok(result.return_value)

    }


/// 

    /// * `feature_updates_only` -  (bool)
    /// * `include_recommended_updates` -  (bool)
    /// * `query_string` -  (String)

    /// * `info` -  (MSFT_CAU_DownloadUpdateInfo[])
    /// * `return_value` -  (u32)
    pub fn download_updates(&self, query_string: &String, include_recommended_updates: bool, feature_updates_only: bool, info: &mut Vec<MSFT_CAU_DownloadUpdateInfo>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "QueryString".to_string(), value: query_string.into() });
        args.push(MethodParameter { name: "IncludeRecommendedUpdates".to_string(), value: include_recommended_updates.into() });
        args.push(MethodParameter { name: "FeatureUpdatesOnly".to_string(), value: feature_updates_only.into() });

        let result = self.invoke_method("DownloadUpdates", &args)?;
        let info = result.get_value("Info")?;
        Ok(result.return_value)

    }


/// 

    /// * `feature_updates_only` -  (bool)
    /// * `include_recommended_updates` -  (bool)
    /// * `query_string` -  (String)

    /// * `info` -  (MSFT_CAU_InstallUpdateInfo[])
    /// * `return_value` -  (u32)
    pub fn install_updates(&self, query_string: &String, include_recommended_updates: bool, feature_updates_only: bool, info: &mut Vec<MSFT_CAU_InstallUpdateInfo>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "QueryString".to_string(), value: query_string.into() });
        args.push(MethodParameter { name: "IncludeRecommendedUpdates".to_string(), value: include_recommended_updates.into() });
        args.push(MethodParameter { name: "FeatureUpdatesOnly".to_string(), value: feature_updates_only.into() });

        let result = self.invoke_method("InstallUpdates", &args)?;
        let info = result.get_value("Info")?;
        Ok(result.return_value)

    }


/// 

    /// * `query_string` -  (String)

    /// * `result` -  (MSFT_CAU_CommitFUpdateWuaResult)
    /// * `return_value` -  (u32)
    pub fn commit_fupdates_wua(&self, query_string: &String, result: &mut MSFT_CAU_CommitFUpdateWuaResult) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "QueryString".to_string(), value: query_string.into() });

        let result = self.invoke_method("CommitFUpdatesWua", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `installer_program_path` -  (String)
    /// * `parameters` -  (String)
    /// * `require_smb_encryption` -  (bool)
    /// * `update_path` -  (String)

    /// * `result` -  (MSFT_CAU_Update_Installer_Result)
    /// * `return_value` -  (u32)
    pub fn run_update_installer(&self, installer_program_path: &String, parameters: &String, update_path: &String, require_smb_encryption: bool, result: &mut MSFT_CAU_Update_Installer_Result) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InstallerProgramPath".to_string(), value: installer_program_path.into() });
        args.push(MethodParameter { name: "Parameters".to_string(), value: parameters.into() });
        args.push(MethodParameter { name: "UpdatePath".to_string(), value: update_path.into() });
        args.push(MethodParameter { name: "RequireSmbEncryption".to_string(), value: require_smb_encryption.into() });

        let result = self.invoke_method("RunUpdateInstaller", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmd_line_args` -  (String)
    /// * `path_to_setup` -  (String)

    /// * `result` -  (MSFT_CAU_Legacy_Upgrade_Install_Result)
    /// * `return_value` -  (u32)
    pub fn run_upgrade_legacy_media_interface(&self, path_to_setup: &String, cmd_line_args: &String, result: &mut MSFT_CAU_Legacy_Upgrade_Install_Result) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathToSetup".to_string(), value: path_to_setup.into() });
        args.push(MethodParameter { name: "CmdLineArgs".to_string(), value: cmd_line_args.into() });

        let result = self.invoke_method("RunUpgradeLegacyMediaInterface", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `path_to_deployment_cab` -  (String)
    /// * `path_to_setup` -  (String)

    /// * `result` -  (MSFT_CAU_Audi_Install_Result)
    /// * `return_value` -  (u32)
    pub fn run_upgrade_install_audi_interface(&self, path_to_setup: &String, path_to_deployment_cab: &String, result: &mut MSFT_CAU_Audi_Install_Result) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathToSetup".to_string(), value: path_to_setup.into() });
        args.push(MethodParameter { name: "PathToDeploymentCab".to_string(), value: path_to_deployment_cab.into() });

        let result = self.invoke_method("RunUpgradeInstallAudiInterface", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `path_to_deployment_cab` -  (String)
    /// * `path_to_setup` -  (String)
    /// * `update_id` -  (String)

    /// * `result` -  (MSFT_CAU_Audi_Commit_Result)
    /// * `return_value` -  (u32)
    pub fn run_upgrade_commit_audi_interface(&self, path_to_setup: &String, update_id: &String, path_to_deployment_cab: &String, result: &mut MSFT_CAU_Audi_Commit_Result) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathToSetup".to_string(), value: path_to_setup.into() });
        args.push(MethodParameter { name: "UpdateId".to_string(), value: update_id.into() });
        args.push(MethodParameter { name: "PathToDeploymentCab".to_string(), value: path_to_deployment_cab.into() });

        let result = self.invoke_method("RunUpgradeCommitAudiInterface", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `path_to_deployment_cab` -  (String)
    /// * `path_to_setup` -  (String)
    /// * `update_id` -  (String)

    /// * `error_code` -  (MSFT_CAU_Audi_GetPostRebootResult_Result)
    /// * `return_value` -  (u32)
    pub fn get_post_reboot_result(&self, path_to_setup: &String, update_id: &String, path_to_deployment_cab: &String, error_code: &mut MSFT_CAU_Audi_GetPostRebootResult_Result) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathToSetup".to_string(), value: path_to_setup.into() });
        args.push(MethodParameter { name: "UpdateId".to_string(), value: update_id.into() });
        args.push(MethodParameter { name: "PathToDeploymentCab".to_string(), value: path_to_deployment_cab.into() });

        let result = self.invoke_method("GetPostRebootResult", &args)?;
        let error_code = result.get_value("ErrorCode")?;
        Ok(result.return_value)

    }


/// 

    /// * `path_to_deployment_cab` -  (String)
    /// * `path_to_files` -  (String)

    /// * `result` -  (MSFT_CAU_Audi_GenerateDeviceInfoFilesFU_Result)
    /// * `return_value` -  (u32)
    pub fn generate_device_info_files_fu(&self, path_to_files: &String, path_to_deployment_cab: &String, result: &mut MSFT_CAU_Audi_GenerateDeviceInfoFilesFU_Result) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathToFiles".to_string(), value: path_to_files.into() });
        args.push(MethodParameter { name: "PathToDeploymentCab".to_string(), value: path_to_deployment_cab.into() });

        let result = self.invoke_method("GenerateDeviceInfoFilesFU", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }

}

