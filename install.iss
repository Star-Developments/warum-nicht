[Setup]
AppName=WarumNicht
AppVersion=1.0
DefaultDirName={localappdata}\WarumNicht
DisableProgramGroupPage=yes
OutputDir=.
OutputBaseFilename=WarumNichtInstaller
Compression=lzma
SolidCompression=yes
PrivilegesRequired=lowest
UninstallDisplayIcon={app}\WarumNicht.exe

[Files]
Source: "WarumNicht.exe"; DestDir: "{app}"
Source: "WarumNicht.ico"; DestDir: "{app}"

[Run]
; Bin-Ordner erzwingen
Filename: "{cmd}"; Parameters: "/C mkdir ""{localappdata}\WarumNicht\bin"" 2>nul"; Flags: runhidden

; wnrun.bat erstellen
Filename: "{cmd}"; Parameters: "/C echo @echo off > ""{localappdata}\WarumNicht\bin\wnrun.bat"""; Flags: runhidden
Filename: "{cmd}"; Parameters: "/C echo ""{app}\WarumNicht.exe"" ""%%CD%%\%%1"" >> ""{localappdata}\WarumNicht\bin\wnrun.bat"""; Flags: runhidden

; PATH dauerhaft erweitern (nur für User)
Filename: "{cmd}"; Parameters: "/C setx PATH ""%PATH%;{localappdata}\WarumNicht\bin"""; Flags: runhidden

; PATH sofort für aktuelle Session aktivieren (damit man direkt loslegen kann)
Filename: "{cmd}"; Parameters: "/C set PATH=%PATH%;{localappdata}\WarumNicht\bin"; Flags: runhidden

[UninstallDelete]
Type: files; Name: "{localappdata}\WarumNicht\bin\wnrun.bat"
Type: dirifempty; Name: "{localappdata}\WarumNicht\bin"