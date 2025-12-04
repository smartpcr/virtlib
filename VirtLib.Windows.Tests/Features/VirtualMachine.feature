Feature: VirtualMachine
	Validate VM logic and functionality

    @integration_test @vm
    Scenario: Get virtual machine from home machine
        Given hyper-v host running on localhost
	    When I get vm with name "devbox1"
	    Then vm should exist with name "devbox1"

    @integration_test @vm
    Scenario: Get virtual machine from work machine
        Given hyper-v host running on localhost
        When I get vm with name "ubuntu"
        Then vm should exist with name "ubuntu"

    @integration_test @vm
    Scenario: Create gen-1 virtual machine
        Given hyper-v host running on localhost
        And ubuntu image "ubuntu2404.iso" is at "X:\iso\ubuntu2404.iso"
        When I create gen 1 vm with name "ubuntu2"
        | Name             | Value          |
        | Username         | ubuntu         |
        | Password         | ubuntu         |
        | SshPublicKeyFile | ubuntu_rsa.pub |
        | CpuCount         | 2              |
        | MemoryInMB       | 2048           |
        | HardDiskSizeInGB | 20             |
        | SwitchName       | Default Switch |
        Then vm should exist with name "ubuntu2"
        And delete vm with name "ubuntu2"

    @integration_test @vm
    Scenario: Create gen-2 virtual machine
        Given hyper-v host running on localhost
        And ubuntu image "ubuntu2404.iso" is at "X:\iso\ubuntu2404.iso"
        When I create gen 2 vm with name "ubuntu3"
          | Name             | Value          |
          | Username         | ubuntu         |
          | Password         | ubuntu         |
          | SshPublicKeyFile | ubuntu_rsa.pub |
          | CpuCount         | 2              |
          | MemoryInMB       | 2048           |
          | HardDiskSizeInGB | 20             |
          | SwitchName       | Default Switch |
        Then vm should exist with name "ubuntu3"
        And delete vm with name "ubuntu3"
