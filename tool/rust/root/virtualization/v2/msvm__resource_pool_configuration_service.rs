// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ResourcePoolConfigurationService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ResourcePoolConfigurationService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_ResourcePoolConfigurationService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// Starts a job to create a child ResourcePool. The ResourcePool will be scoped to the same System as this Service. If 0 is returned, then the task completed successfully and the use of ConcreteJob was not required. If the task will take some time to complete, a ConcreteJob will be created and its reference returned in the output parameter Job. The resulting pool will be a child pool.

    /// * `allocation_settings` - String containing one or more embedded instances of CIM_ResourceAllocationSettingData that is used to specify the pools allocation related settings. This array must contain either one element for each elemnt in the ParentPools array or exactly one element. If this array contains one element and ParentPools contains more than one element, the AlllocationSettings specifies a shared capacity allocation that can be satisfied by any of the parent pools. This is used to restrict the resources that can be allocated from the child to pool to a lower limit than the aggregate capacity provided by its parents. This option is not supported by all resource types. If a resource type does not support shared capacity allocation, this method shall return "Not Supported". (String[])
    /// * `parent_pools` - The Pool(s) from which to create the new Pool. (CIM_ResourcePool[])
    /// * `pool_settings` - String containing an embedded instance of a Msvm_ResourcePoolSettingData instance that is used to specify the pools non-allocation related settings. (String)

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `pool` - A reference to the resulting pool. (CIM_ResourcePool)
    /// * `return_value` -  (u32)
    pub fn create_pool(&self, pool_settings: &String, parent_pools: &Vec<CIM_ResourcePool>, allocation_settings: &Vec<String>, pool: &mut CIM_ResourcePool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PoolSettings".to_string(), value: pool_settings.into() });
        args.push(MethodParameter { name: "ParentPools".to_string(), value: parent_pools.into() });
        args.push(MethodParameter { name: "AllocationSettings".to_string(), value: allocation_settings.into() });

        let result = self.invoke_method_with_job("CreatePool", &args)?;
        let job = result.get_value("Job")?;
        let pool = result.get_value("Pool")?;
        Ok(result.return_value)

    }


/// Start a job to change parent pool resource settings for resources assigned to a child pool. If 0 is returned, the function completed successfully and no ConcreteJob instance was required. If 4096/0x1000 is returned, a ConcreteJob will be started to change the parent pool. The Job's reference will be returned in the output parameter Job.

    /// * `allocation_settings` - Optional string containing a representation of a CIM_SettingData instance that is used to specify the settings for the Parent Pool. (String[])
    /// * `child_pool` - Reference to the child pool. (CIM_ResourcePool)
    /// * `parent_pools` - Reference to the parent pool(s). (CIM_ResourcePool[])

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_pool_resources(&self, child_pool: CIM_ResourcePool, parent_pools: &Vec<CIM_ResourcePool>, allocation_settings: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ChildPool".to_string(), value: child_pool.into() });
        args.push(MethodParameter { name: "ParentPools".to_string(), value: parent_pools.into() });
        args.push(MethodParameter { name: "AllocationSettings".to_string(), value: allocation_settings.into() });

        let result = self.invoke_method_with_job("ModifyPoolResources", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Start a job to change the non-allocation related settings of a child. If 0 is returned, the function completed successfully and no ConcreteJob instance was required. If 4096/0x1000 is returned, a ConcreteJob will be started to change the settings. The Job's reference will be returned in the output parameter Job.

    /// * `child_pool` - Reference to the child pool. (CIM_ResourcePool)
    /// * `pool_settings` - String containing an embedded instance of a Msvm_ResourcePoolSettingData that is used to specify the settings for the Pool. (String)

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_pool_settings(&self, child_pool: CIM_ResourcePool, pool_settings: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ChildPool".to_string(), value: child_pool.into() });
        args.push(MethodParameter { name: "PoolSettings".to_string(), value: pool_settings.into() });

        let result = self.invoke_method_with_job("ModifyPoolSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// Start a job to delete a ResourcePool. No allocations may be outstanding or the delete will fail with "In Use." If the resource pool is a root resource pool, any host resources are returned back to the underlying system. If 0 is returned, the function completed successfully, and no ConcreteJob was required. If 4096/0x1000 is returned, a ConcreteJob will be started to delete the ResourcePool. A reference to the Job is returned in the Job parameter.

    /// * `pool` - Reference to the pool to delete. (CIM_ResourcePool)

    /// * `job` - Reference to the job (may be null if job completed). (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn delete_pool(&self, pool: CIM_ResourcePool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Pool".to_string(), value: pool.into() });

        let result = self.invoke_method_with_job("DeletePool", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_ResourcePoolConfigurationService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_ResourcePool object(s)
    pub fn get_related__resource_pool(&self) -> Result<Vec<Msvm_ResourcePool>, WmiError> {
        self.get_all_related("Msvm_ResourcePool")
    }

    /// Gets the related Msvm_Synth3dVideoPool object(s)
    pub fn get_related__synth3d_video_pool(&self) -> Result<Msvm_Synth3dVideoPool, WmiError> {
        self.get_related("Msvm_Synth3dVideoPool")
    }

    /// Gets the related Msvm_ProcessorPool object(s)
    pub fn get_related__processor_pool(&self) -> Result<Msvm_ProcessorPool, WmiError> {
        self.get_related("Msvm_ProcessorPool")
    }

    /// Gets the related Msvm_ResourcePoolConfigurationCapabilities object(s)
    pub fn get_related__resource_pool_configuration_capabilities(&self) -> Result<Msvm_ResourcePoolConfigurationCapabilities, WmiError> {
        self.get_related("Msvm_ResourcePoolConfigurationCapabilities")
    }

}

