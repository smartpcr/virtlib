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
    Scenario: Create virtual machine
        Given hyper-v host running on localhost
        And ubuntu image is downloaded from url "https://releases.ubuntu.com/jammy/ubuntu-22.04.4-live-server-amd64.iso"
        When I create vm with name "ubuntu-2"
        And the following cloud-init settings
        Then vm should exist with name "ubuntu-2"