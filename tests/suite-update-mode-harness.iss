#ifndef HarnessOutput
  #error Define HarnessOutput.
#endif
#ifndef HarnessReport
  #error Define HarnessReport.
#endif
[Setup]
AppName=Creative Suite Arabic Update Mode QA
AppVersion=1
DefaultDirName={tmp}\CreativeSuiteArabicUpdateModeQA
CreateAppDir=no
Uninstallable=no
PrivilegesRequired=lowest
OutputDir={#HarnessOutput}
OutputBaseFilename=suite-update-mode-qa
Compression=none
DisableStartupPrompt=yes
[Code]
#include "..\packaging\suite-update-mode.iss"

function InitializeSetup: Boolean;
var
  Expected: Boolean;
begin
  Result := False;
  Expected := ExpandConstant('{param:EXPECTOFFER|missing}') = '1';
  if ShouldOfferLauncher <> Expected then
    RaiseException('Finish-page launch ownership differs from expected update mode');
  if not SaveStringToFile('{#HarnessReport}', 'PASS launch ownership' + #13#10, False) then
    RaiseException('Could not persist update-mode report');
end;
