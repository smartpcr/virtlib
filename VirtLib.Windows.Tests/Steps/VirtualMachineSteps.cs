namespace VirtLib.Windows.Tests.Steps;

using FluentAssertions;
using TechTalk.SpecFlow;
using Windows.Models;

[Binding]
public class VirtualMachineSteps
{
    private readonly ScenarioContext _context;

    public VirtualMachineSteps(ScenarioContext context)
    {
        _context = context;
    }

    [When(@"I get vm with name ""(.*)""")]
    public void WhenIGetVmWithName(string vmName)
    {
        var hcs = _context.Get<HostComputeSystem>();
        var vm = hcs.GetVirtualMachine(vmName);
        _context.Set(vm);
    }

    [Then(@"vm should exist with name ""(.*)""")]
    public void ThenVmShouldExistWithName(string vmName)
    {
        var vm = _context.Get<VirtualMachine>();
        vm.Should().NotBeNull();
        vm.Name.Should().Be(vmName);
    }
}