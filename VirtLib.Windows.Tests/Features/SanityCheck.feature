Feature: SanityCheck
	Make sure pre-requisites are in place

@pre-requisite
Scenario: System should be running in windows
	When check current os platform
	Then os platform should be "Windows"

@pre-requisite
Scenario: Program should be running in elevated mode
    When check current user is admin
    Then current user should be admin

@pre-requisite
Scenario: Hyper-V should be running
    When check hyper-v is running
    Then hyper-v should be running