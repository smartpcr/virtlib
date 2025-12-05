// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerManagerTasks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerManagerTasks {
}

impl MSFT_ServerManagerTasks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `batch_size` -  (u32)
    /// * `collector_name` -  (String)
    /// * `counter_paths` -  (String[])
    /// * `timestamps` -  (String[])

    /// * `cmdlet_output` -  (MSFT_ServerPerformanceCounterSamples[])
    /// * `return_value` -  (u32)
    pub fn get_counter_samples_at_time(&self, collector_name: &String, counter_paths: &Vec<String>, timestamps: &Vec<String>, batch_size: u32, cmdlet_output: &mut Vec<MSFT_ServerPerformanceCounterSamples>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CollectorName".to_string(), value: collector_name.into() });
        args.push(MethodParameter { name: "CounterPaths".to_string(), value: counter_paths.into() });
        args.push(MethodParameter { name: "Timestamps".to_string(), value: timestamps.into() });
        args.push(MethodParameter { name: "BatchSize".to_string(), value: batch_size.into() });

        let result = self.invoke_method("GetCounterSamplesAtTime", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `batch_size` -  (u32)
    /// * `collector_name` -  (String)
    /// * `counter_paths` -  (String[])
    /// * `end_time` -  (String)
    /// * `start_time` -  (String)

    /// * `cmdlet_output` -  (MSFT_ServerPerformanceCounterSamples[])
    /// * `return_value` -  (u32)
    pub fn get_counter_samples_in_time_range(&self, collector_name: &String, counter_paths: &Vec<String>, start_time: &String, end_time: &String, batch_size: u32, cmdlet_output: &mut Vec<MSFT_ServerPerformanceCounterSamples>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CollectorName".to_string(), value: collector_name.into() });
        args.push(MethodParameter { name: "CounterPaths".to_string(), value: counter_paths.into() });
        args.push(MethodParameter { name: "StartTime".to_string(), value: start_time.into() });
        args.push(MethodParameter { name: "EndTime".to_string(), value: end_time.into() });
        args.push(MethodParameter { name: "BatchSize".to_string(), value: batch_size.into() });

        let result = self.invoke_method("GetCounterSamplesInTimeRange", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `collector_name` -  (String)

    /// * `cmdlet_output` -  (u8)
    /// * `return_value` -  (u32)
    pub fn get_performance_collector_state(&self, collector_name: &String, cmdlet_output: &mut u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CollectorName".to_string(), value: collector_name.into() });

        let result = self.invoke_method("GetPerformanceCollectorState", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `collector_name` -  (String)
    /// * `state` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set_performance_collector_state(&self, collector_name: &String, state: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CollectorName".to_string(), value: collector_name.into() });
        args.push(MethodParameter { name: "State".to_string(), value: state.into() });
        self.invoke_method("SetPerformanceCollectorState", &args)

    }


/// 

    /// * `batch_size` -  (u32)
    /// * `bpa_xpaths` -  (String[])

    /// * `cmdlet_output` -  (MSFT_ServerBpaResult[])
    /// * `return_value` -  (u32)
    pub fn get_server_bpa_result(&self, bpa_xpaths: &Vec<String>, batch_size: u32, cmdlet_output: &mut Vec<MSFT_ServerBpaResult>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "BpaXPaths".to_string(), value: bpa_xpaths.into() });
        args.push(MethodParameter { name: "BatchSize".to_string(), value: batch_size.into() });

        let result = self.invoke_method("GetServerBpaResult", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_server_cluster_name(&self, cmdlet_output: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetServerClusterName", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `batch_size` -  (u32)
    /// * `end_times` -  (u64[])
    /// * `levels` -  (u8[])
    /// * `logs` -  (String[])
    /// * `query_file_ids` -  (i32[])
    /// * `query_files` -  (String[])
    /// * `start_times` -  (u64[])

    /// * `cmdlet_output` -  (MSFT_ServerEventDetail[])
    /// * `latest_event_timestamp` -  (u64)
    /// * `return_value` -  (u32)
    pub fn get_server_event_detail(&self, logs: &Vec<String>, levels: &Vec<u8>, start_times: &Vec<u64>, end_times: &Vec<u64>, batch_size: u32, query_files: &Vec<String>, query_file_ids: &Vec<i32>, latest_event_timestamp: &mut u64, cmdlet_output: &mut Vec<MSFT_ServerEventDetail>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Logs".to_string(), value: logs.into() });
        args.push(MethodParameter { name: "Levels".to_string(), value: levels.into() });
        args.push(MethodParameter { name: "StartTimes".to_string(), value: start_times.into() });
        args.push(MethodParameter { name: "EndTimes".to_string(), value: end_times.into() });
        args.push(MethodParameter { name: "BatchSize".to_string(), value: batch_size.into() });
        args.push(MethodParameter { name: "QueryFiles".to_string(), value: query_files.into() });
        args.push(MethodParameter { name: "QueryFileIds".to_string(), value: query_file_ids.into() });

        let result = self.invoke_method("GetServerEventDetail", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        let latest_event_timestamp = result.get_value("LatestEventTimestamp")?;
        Ok(result.return_value)

    }


/// 

    /// * `batch_size` -  (u32)
    /// * `filter_flags` -  (u32)

    /// * `cmdlet_output` -  (MSFT_ServerFeature[])
    /// * `requires_reboot` -  (bool)
    /// * `return_value` -  (u32)
    pub fn get_server_feature(&self, filter_flags: u32, batch_size: u32, requires_reboot: &mut bool, cmdlet_output: &mut Vec<MSFT_ServerFeature>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilterFlags".to_string(), value: filter_flags.into() });
        args.push(MethodParameter { name: "BatchSize".to_string(), value: batch_size.into() });

        let result = self.invoke_method("GetServerFeature", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        let requires_reboot = result.get_value("RequiresReboot")?;
        Ok(result.return_value)

    }


/// 

    /// * `smserver_id` -  (String)

    /// * `cluster_information` -  (MSFT_ServerClusterInformation)
    /// * `event_logs` -  (MSFT_ServerEventLog[])
    /// * `network_adapters` -  (MSFT_ServerNetworkAdapter[])
    /// * `operating_system` -  (MSFT_ServerOperatingSystem)
    /// * `return_value` -  (u32)
    /// * `server_inventory` -  (MSFT_ServerInventory)
    pub fn get_server_inventory(&self, smserver_id: &String, server_inventory: &mut MSFT_ServerInventory, operating_system: &mut MSFT_ServerOperatingSystem, cluster_information: &mut MSFT_ServerClusterInformation, network_adapters: &mut Vec<MSFT_ServerNetworkAdapter>, event_logs: &mut Vec<MSFT_ServerEventLog>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SMServerId".to_string(), value: smserver_id.into() });

        let result = self.invoke_method("GetServerInventory", &args)?;
        let cluster_information = result.get_value("ClusterInformation")?;
        let event_logs = result.get_value("EventLogs")?;
        let network_adapters = result.get_value("NetworkAdapters")?;
        let operating_system = result.get_value("OperatingSystem")?;
        let server_inventory = result.get_value("ServerInventory")?;
        Ok(result.return_value)

    }


/// 

    /// * `batch_size` -  (u32)
    /// * `services` -  (String[])

    /// * `cmdlet_output` -  (MSFT_ServerServiceDetail[])
    /// * `return_value` -  (u32)
    pub fn get_server_service_detail(&self, services: &Vec<String>, batch_size: u32, cmdlet_output: &mut Vec<MSFT_ServerServiceDetail>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Services".to_string(), value: services.into() });
        args.push(MethodParameter { name: "BatchSize".to_string(), value: batch_size.into() });

        let result = self.invoke_method("GetServerServiceDetail", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `collector_name` -  (String)
    /// * `millisecond_threshold` -  (u64)

    /// * `return_value` -  (u32)
    pub fn remove_server_performance_log(&self, collector_name: &String, millisecond_threshold: u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CollectorName".to_string(), value: collector_name.into() });
        args.push(MethodParameter { name: "MillisecondThreshold".to_string(), value: millisecond_threshold.into() });
        self.invoke_method("RemoveServerPerformanceLog", &args)

    }


/// 

    /// * `batch_size` -  (u32)
    /// * `filter_xml` -  (String)
    /// * `reverse_direction` -  (bool)
    /// * `skip` -  (u64)
    /// * `top` -  (u64)

    /// * `cmdlet_output` -  (MSFT_ServerEventDetail[])
    /// * `return_value` -  (u32)
    pub fn get_server_event_detail_ex(&self, filter_xml: &String, skip: u64, top: u64, reverse_direction: bool, batch_size: u32, cmdlet_output: &mut Vec<MSFT_ServerEventDetail>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilterXml".to_string(), value: filter_xml.into() });
        args.push(MethodParameter { name: "Skip".to_string(), value: skip.into() });
        args.push(MethodParameter { name: "Top".to_string(), value: top.into() });
        args.push(MethodParameter { name: "ReverseDirection".to_string(), value: reverse_direction.into() });
        args.push(MethodParameter { name: "BatchSize".to_string(), value: batch_size.into() });

        let result = self.invoke_method("GetServerEventDetailEx", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

