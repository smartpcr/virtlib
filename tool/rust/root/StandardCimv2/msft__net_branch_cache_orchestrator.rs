// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCacheOrchestrator struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCacheOrchestrator {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,
}

impl MSFT_NetBranchCacheOrchestrator {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
        }
    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn enable__bcdistributed(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Enable_BCDistributed", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `server_names` -  (String[])
    /// * `use_version` -  (u32)

    /// * `return_value` -  (u32)
    pub fn enable__bchosted_client_by_server_names(&self, server_names: &Vec<String>, use_version: u32, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerNames".to_string(), value: server_names.into() });
        args.push(MethodParameter { name: "UseVersion".to_string(), value: use_version.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Enable_BCHostedClientByServerNames", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `use_scp` -  (bool)

    /// * `return_value` -  (u32)
    pub fn enable__bchosted_client_by_use_scp(&self, use_scp: bool, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UseSCP".to_string(), value: use_scp.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Enable_BCHostedClientByUseSCP", &args)

    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn enable__bclocal(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Enable_BCLocal", &args)

    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn disable__bc(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Disable_BC", &args)

    }


/// 

    /// * `data_cache_extension` -  (MSFT_NetBranchCacheDataCacheExtension[])
    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn remove__bcdata_cache_extension_by_data_cache_extension(&self, data_cache_extension: &Vec<MSFT_NetBranchCacheDataCacheExtension>, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DataCacheExtension".to_string(), value: data_cache_extension.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Remove_BCDataCacheExtensionByDataCacheExtension", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `path` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove__bcdata_cache_extension_by_path(&self, path: &String, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Remove_BCDataCacheExtensionByPath", &args)

    }


/// 

    /// * `cache` -  (MSFT_NetBranchCacheCache[])
    /// * `defragment` -  (bool)
    /// * `force` -  (bool)
    /// * `move_to` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `percentage` -  (u32)
    /// * `size_bytes` -  (u64)

    /// * `cmdlet_output` -  (MSFT_NetBranchCacheCache[])
    /// * `return_value` -  (u32)
    pub fn set__bccache_by_cache(&self, cache: &Vec<MSFT_NetBranchCacheCache>, move_to: &String, percentage: u32, size_bytes: u64, defragment: bool, pass_thru: bool, force: bool, cmdlet_output: &mut Vec<MSFT_NetBranchCacheCache>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Cache".to_string(), value: cache.into() });
        args.push(MethodParameter { name: "MoveTo".to_string(), value: move_to.into() });
        args.push(MethodParameter { name: "Percentage".to_string(), value: percentage.into() });
        args.push(MethodParameter { name: "SizeBytes".to_string(), value: size_bytes.into() });
        args.push(MethodParameter { name: "Defragment".to_string(), value: defragment.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Set_BCCacheByCache", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `defragment` -  (bool)
    /// * `force` -  (bool)
    /// * `move_to` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `path` -  (String)
    /// * `percentage` -  (u32)
    /// * `size_bytes` -  (u64)

    /// * `cmdlet_output` -  (MSFT_NetBranchCacheCache)
    /// * `return_value` -  (u32)
    pub fn set__bccache_by_path(&self, path: &String, move_to: &String, percentage: u32, size_bytes: u64, defragment: bool, pass_thru: bool, force: bool, cmdlet_output: &mut MSFT_NetBranchCacheCache) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "MoveTo".to_string(), value: move_to.into() });
        args.push(MethodParameter { name: "Percentage".to_string(), value: percentage.into() });
        args.push(MethodParameter { name: "SizeBytes".to_string(), value: size_bytes.into() });
        args.push(MethodParameter { name: "Defragment".to_string(), value: defragment.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Set_BCCacheByPath", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn clear__bccache(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Clear_BCCache", &args)

    }


/// 

    /// * `filename` -  (String)
    /// * `file_passphrase` -  (String)
    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn import__bcsecret_key(&self, filename: &String, file_passphrase: &String, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Filename".to_string(), value: filename.into() });
        args.push(MethodParameter { name: "FilePassphrase".to_string(), value: file_passphrase.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Import_BCSecretKey", &args)

    }


/// 

    /// * `filename` -  (String)
    /// * `file_passphrase` -  (String)
    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn export__bcsecret_key(&self, filename: &String, file_passphrase: &String, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Filename".to_string(), value: filename.into() });
        args.push(MethodParameter { name: "FilePassphrase".to_string(), value: file_passphrase.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Export_BCSecretKey", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `mode` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set__bcauthentication(&self, mode: u32, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Mode".to_string(), value: mode.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Set_BCAuthentication", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `version` -  (u32)

    /// * `return_value` -  (u32)
    pub fn enable__bcdowngrading(&self, version: u32, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Version".to_string(), value: version.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Enable_BCDowngrading", &args)

    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn enable__bcserve_on_battery(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Enable_BCServeOnBattery", &args)

    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn disable__bcserve_on_battery(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Disable_BCServeOnBattery", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `latency_milliseconds` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set__bcmin_smblatency(&self, latency_milliseconds: u32, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LatencyMilliseconds".to_string(), value: latency_milliseconds.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Set_BCMinSMBLatency", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `passphrase` -  (String)

    /// * `return_value` -  (u32)
    pub fn set__bcsecret_key(&self, passphrase: &String, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Passphrase".to_string(), value: passphrase.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Set_BCSecretKey", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `path` -  (String)
    /// * `percentage` -  (u32)

    /// * `cmdlet_output` -  (MSFT_NetBranchCacheDataCacheExtension)
    /// * `return_value` -  (u32)
    pub fn add__bcdata_cache_extension_by_percentage(&self, percentage: u32, path: &String, pass_thru: bool, force: bool, cmdlet_output: &mut MSFT_NetBranchCacheDataCacheExtension) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Percentage".to_string(), value: percentage.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Add_BCDataCacheExtensionByPercentage", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `path` -  (String)
    /// * `size_bytes` -  (u64)

    /// * `cmdlet_output` -  (MSFT_NetBranchCacheDataCacheExtension)
    /// * `return_value` -  (u32)
    pub fn add__bcdata_cache_extension_by_size_bytes(&self, size_bytes: u64, path: &String, pass_thru: bool, force: bool, cmdlet_output: &mut MSFT_NetBranchCacheDataCacheExtension) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SizeBytes".to_string(), value: size_bytes.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Add_BCDataCacheExtensionBySizeBytes", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn disable__bcdowngrading(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Disable_BCDowngrading", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `register_scp` -  (bool)

    /// * `return_value` -  (u32)
    pub fn enable__bchosted_server(&self, register_scp: bool, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RegisterSCP".to_string(), value: register_scp.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Enable_BCHostedServer", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `reset_fwrules_only` -  (bool)
    /// * `reset_perf_counters_only` -  (bool)

    /// * `return_value` -  (u32)
    pub fn reset__bc(&self, reset_fwrules_only: bool, reset_perf_counters_only: bool, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResetFWRulesOnly".to_string(), value: reset_fwrules_only.into() });
        args.push(MethodParameter { name: "ResetPerfCountersOnly".to_string(), value: reset_perf_counters_only.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Reset_BC", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `path` -  (String[])
    /// * `recurse` -  (bool)
    /// * `reference_file` -  (String)
    /// * `stage_data` -  (bool)
    /// * `staging_path` -  (String)
    /// * `use_version` -  (u32)

    /// * `return_value` -  (u32)
    pub fn publish__bcweb_hashes(&self, path: &Vec<String>, use_version: u32, stage_data: bool, staging_path: &String, reference_file: &String, recurse: bool, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "UseVersion".to_string(), value: use_version.into() });
        args.push(MethodParameter { name: "StageData".to_string(), value: stage_data.into() });
        args.push(MethodParameter { name: "StagingPath".to_string(), value: staging_path.into() });
        args.push(MethodParameter { name: "ReferenceFile".to_string(), value: reference_file.into() });
        args.push(MethodParameter { name: "Recurse".to_string(), value: recurse.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Publish_BCWebHashes", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `path` -  (String[])
    /// * `recurse` -  (bool)
    /// * `reference_file` -  (String)
    /// * `stage_data` -  (bool)
    /// * `staging_path` -  (String)
    /// * `use_version` -  (u32)

    /// * `return_value` -  (u32)
    pub fn publish__bcfile_hashes(&self, path: &Vec<String>, use_version: u32, stage_data: bool, staging_path: &String, reference_file: &String, recurse: bool, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "UseVersion".to_string(), value: use_version.into() });
        args.push(MethodParameter { name: "StageData".to_string(), value: stage_data.into() });
        args.push(MethodParameter { name: "StagingPath".to_string(), value: staging_path.into() });
        args.push(MethodParameter { name: "ReferenceFile".to_string(), value: reference_file.into() });
        args.push(MethodParameter { name: "Recurse".to_string(), value: recurse.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Publish_BCFileHashes", &args)

    }


/// 

    /// * `destination` -  (String)
    /// * `export_data_cache` -  (bool)
    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn export__bccache_package_by_export_data_cache(&self, export_data_cache: bool, destination: &String, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ExportDataCache".to_string(), value: export_data_cache.into() });
        args.push(MethodParameter { name: "Destination".to_string(), value: destination.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Export_BCCachePackageByExportDataCache", &args)

    }


/// 

    /// * `destination` -  (String)
    /// * `force` -  (bool)
    /// * `output_reference_file` -  (String)
    /// * `staging_path` -  (String)

    /// * `return_value` -  (u32)
    pub fn export__bccache_package_by_staging_path(&self, staging_path: &String, destination: &String, output_reference_file: &String, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StagingPath".to_string(), value: staging_path.into() });
        args.push(MethodParameter { name: "Destination".to_string(), value: destination.into() });
        args.push(MethodParameter { name: "OutputReferenceFile".to_string(), value: output_reference_file.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Export_BCCachePackageByStagingPath", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `path` -  (String)

    /// * `return_value` -  (u32)
    pub fn import__bccache_package(&self, path: &String, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Import_BCCachePackage", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `time_days` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set__bcdata_cache_entry_max_age(&self, time_days: u32, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TimeDays".to_string(), value: time_days.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Set_BCDataCacheEntryMaxAge", &args)

    }

}

