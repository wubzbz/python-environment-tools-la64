# Support Guidelines for LoongArch64 Port

> [!NOTE]
> Please check the [Known Issues and Possible Solution](./KNOWN_ISSUE.md) page first!

## 🎯 Where to Report Issues?

**Please report issues in THIS repository when:**
- The issue is related to **LoongArch64 architecture**
- Specific behavior or errors occur only on LoongArch64 platforms
- Build, installation, or packaging problems with this port
- Feature requests specific to LoongArch64

**Please report issues in the [OFFICIAL REPOSITORY](https://github.com/microsoft/python-environment-tools) when:**
- The issue occurs across all architectures (e.g., x86_64, ARM64)
- Related to core environment detection logic (not architecture-specific)
- General feature problems that also appear in official versions

## 🔍 Issue Categorization Guide

| Issue Type | Report Location | Reason |
|------------|-----------------|--------|
| **LoongArch64 build failure** | ✅ **THIS REPOSITORY** | Architecture-specific compilation |
| **Runtime crash on LoongArch64** | ✅ **THIS REPOSITORY** | Platform-specific binary fault |
| **Environment discovery failure (LA64 only)** | ✅ **THIS REPOSITORY** | Architecture-related locator logic |
| **General Python environment detection issues** | ⚠️ **OFFICIAL REPOSITORY** | Core functionality, not arch-specific |
| **Integration problem with VS Code Python extension** | ⚠️ **OFFICIAL EXTENSION REPO** | If reproducible on x86_64 |
| **Uncertain about issue origin** | 🔍 **START HERE** | We can help diagnose |

## 📝 When Reporting Issues, Please Provide

**Required Information:**
- Complete error messages and stack traces
- Your LoongArch64 system information (OS, kernel version, architecture details)
- Python version(s) installed and their paths (`uname -a`, `python --version`)
- Specific commit hash or release tag of this port
- Output of the tool (e.g., discovery result, any terminal output)

## ⚠️ Important Notes

**This is a community-maintained port:**
- This project is NOT officially supported and is maintained by the community
- Response times may not be as prompt as the official version
- Some advanced features may have limitations on LoongArch64

**Issue Triage Process:**
1. First check [existing issues of official repo](https://github.com/microsoft/python-environment-tools/issues) and [of this repo](https://github.com/wubzbz/python-environment-tools-la64/issues) for similar problems
2. Provide detailed reproduction steps and environment information
3. If the issue likely belongs upstream, we will assist in reporting it

## 🔗 Related Links

- [Official Python Environment Tools Repository](https://github.com/microsoft/python-environment-tools)
- [Build Guide for LoongArch64](./BUILD_LA64.md)

---

## 💡 Before Reporting

To help us quickly identify the issue:

1. **Test with minimal setup** - Try reproducing with a clean, basic Python environment
2. **Check verbose logs** - Enable `RUST_LOG=debug` and include the relevant output sections
3. **Compare with x86_64** - If possible, test if the same issue occurs on an x86_64 build of the official tool
4. **Provide reproduction steps** - Clear steps to trigger the problem

## 🐛 Common LoongArch64-Specific Issues

We're particularly interested in:
- Memory alignment problems in the native binary
- Endianness-related issues
- Instruction set compatibility
- Library binding or linking errors
- Performance characteristics on LA64
- Environment discovery quirks unique to LoongArch64

**Thank you for helping improve the LoongArch64 port!** 🚀


