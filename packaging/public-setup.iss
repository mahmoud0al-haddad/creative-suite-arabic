#ifndef PublicPayload
  #error Define PublicPayload as the prepared public payload directory.
#endif
#ifndef PublicOutput
  #define PublicOutput "..\dist"
#endif
#ifndef PublicVersion
  #define PublicVersion "2026.10-ar.1"
#endif
#define ExcludedRuntime "concrt140.dll,msvcp140.dll,msvcp140_1.dll,msvcp140_2.dll,msvcp140_atomic_wait.dll,msvcp140_codecvt_ids.dll,vccorlib140.dll,vcruntime140.dll,vcruntime140_1.dll,vcruntime140_threads.dll"

[Setup]
AppId={{149F3153-387D-4B57-9478-74333EAA5D28}
AppName={cm:SuiteName}
AppVersion={#PublicVersion}
AppPublisher=mahmoud0al-haddad — independent Arabic edition maintainer
AppPublisherURL=https://github.com/mahmoud0al-haddad/creative-suite-arabic
AppSupportURL=https://github.com/mahmoud0al-haddad/creative-suite-arabic/issues
AppUpdatesURL=https://github.com/mahmoud0al-haddad/creative-suite-arabic/releases
VersionInfoVersion=2026.10.0.1
VersionInfoCompany=mahmoud0al-haddad — independent Arabic edition maintainer
VersionInfoCopyright=Independent Arabic edition; original application authors retain their rights.
VersionInfoDescription=Creative Suite Arabic Independent Setup
VersionInfoProductName=Creative Suite Arabic
DefaultDirName={localappdata}\Programs\Creative Suite Arabic Community
DefaultGroupName=Creative Suite Arabic Community
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0.17763
WizardStyle=modern
Compression=lzma2/fast
SolidCompression=yes
OutputDir={#PublicOutput}
OutputBaseFilename=Creative-Suite-Arabic-Windows-x64-Setup
UninstallDisplayIcon={app}\artcraft-launcher.exe
CloseApplications=yes
RestartApplications=no
DisableProgramGroupPage=yes
SetupLogging=yes
LanguageDetectionMethod=uilanguage
ShowLanguageDialog=yes

[Languages]
Name: "ar"; MessagesFile: "compiler:Languages\Arabic.isl"
Name: "en"; MessagesFile: "compiler:Default.isl"

[CustomMessages]
ar.SuiteName=حزمة التطبيقات الإبداعية العربية — إصدار مستقل
en.SuiteName=Creative Suite Arabic — Independent Edition
ar.LauncherName=مشغّل التطبيقات
en.LauncherName=Application Launcher
ar.DesktopShortcut=إنشاء اختصار على سطح المكتب
en.DesktopShortcut=Create a desktop shortcut
ar.StartLauncher=فتح مشغّل التطبيقات
en.StartLauncher=Open the application launcher
ar.RuntimeRequired=يلزم Microsoft Visual C++ Runtime بإصدار x64، من السلسلة 14 وبإصدار 14.44.35207 أو أحدث. ثبّته من صفحة Microsoft الرسمية ثم أعد المحاولة. قد يطلب مثبّت Microsoft صلاحيات المسؤول.
en.RuntimeRequired=Microsoft Visual C++ x64 v14 Runtime 14.44.35207 or later is required. Install it from Microsoft's official page, then retry. Microsoft's installer may request administrator rights.
ar.RuntimeOffer=متطلبات التشغيل غير مثبتة أو أقدم من المطلوب. هل تريد فتح صفحة Microsoft الرسمية لتنزيل إصدار x64؟ ستثبّته بنفسك، ثم يمكنك إعادة المحاولة هنا.
en.RuntimeOffer=The required runtime is missing or too old. Open Microsoft's official download page for the x64 version? Install it yourself, then retry here.
ar.RuntimeOpenFailed=تعذّر فتح المتصفح. افتح هذا الرابط:
en.RuntimeOpenFailed=The browser could not be opened. Open this link:

[Tasks]
Name: "desktopicon"; Description: "{cm:DesktopShortcut}"; Flags: unchecked

[Files]
Source: "{#PublicPayload}\artcraft-launcher.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#PublicPayload}\portable.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#PublicPayload}\apps\*"; DestDir: "{app}\apps"; Excludes: "{#ExcludedRuntime},*.pdb"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "{#PublicPayload}\licenses\*"; DestDir: "{app}\licenses"; Excludes: "microsoft-runtime\*"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "{#PublicPayload}\ArtCraftLauncherData\data\installed.json"; DestDir: "{app}\ArtCraftLauncherData\data"; Flags: ignoreversion
Source: "{#PublicPayload}\ArtCraftLauncherData\data\bundle.json"; DestDir: "{app}\ArtCraftLauncherData\data"; Flags: ignoreversion
Source: "{#PublicPayload}\ArtCraftLauncherData\personal-suite.txt"; DestDir: "{app}\ArtCraftLauncherData"; Flags: ignoreversion
Source: "{#PublicPayload}\ArtCraftLauncherData\app.ron"; DestDir: "{app}\ArtCraftLauncherData"; Flags: onlyifdoesntexist uninsneveruninstall; Languages: ar
Source: "{#PublicPayload}\ArtCraftLauncherData\app-en.ron"; DestDir: "{app}\ArtCraftLauncherData"; DestName: "app.ron"; Flags: onlyifdoesntexist uninsneveruninstall; Languages: en
Source: "{#PublicPayload}\README-*.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#PublicPayload}\sources\*"; DestDir: "{app}\sources"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "{#PublicPayload}\validation\*"; DestDir: "{app}\validation"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{cm:LauncherName}"; Filename: "{app}\artcraft-launcher.exe"; WorkingDir: "{app}"; AppUserModelID: "local.creative-suite.launcher"
Name: "{autodesktop}\{cm:LauncherName}"; Filename: "{app}\artcraft-launcher.exe"; WorkingDir: "{app}"; Tasks: desktopicon; AppUserModelID: "local.creative-suite.launcher"

[Run]
Filename: "{app}\artcraft-launcher.exe"; Description: "{cm:StartLauncher}"; Flags: nowait postinstall skipifsilent; Check: ShouldOfferLauncher

[Code]
#include "suite-update-mode.iss"
#include "runtime-prerequisite.iss"
