
pub fn bash_content(name: &str, script_path: &str) -> String {
    let script = r#"#!/bin/sh
basedir=$(dirname "$(echo "$0" | sed -e 's,\\,/,g')")

case `uname` in
    *CYGWIN*|*MINGW*|*MSYS*) basedir=`cygpath -w "$basedir"`;;
esac

if [ -x "$basedir/node" ]; then
  exec "$basedir/node"  "$basedir/../{{package_name}}/{{script_path}}" "$@"
else
  exec node "$basedir/../{{package_name}}/{{script_path}}" "$@"
fi"#;

    script.replace("{{package_name}}", name).replace("{{script_path}}", script_path)
}

pub fn cmd_content(name: &str, script_path: &str) -> String {
    let script = r#"
    @ECHO off
GOTO start
:find_dp0
SET dp0=%~dp0
EXIT /b
:start
SETLOCAL
CALL :find_dp0

IF EXIST "%dp0%\node.exe" (
  SET "_prog=%dp0%\node.exe"
) ELSE (
  SET "_prog=node"
  SET PATHEXT=%PATHEXT:;.JS;=;%
)

endLocal & goto #_undefined_# 2>NUL || title %COMSPEC% & "%_prog%"  "%dp0%\..\{{package_name}}\{{script_path}}" %*
    "#;

    script.replace("{{package_name}}", name).replace("{{script_path}}", script_path)
}

pub fn powershell_content(name: &str, script_path: &str) -> String {
    let script = r#"
    #!/usr/bin/env pwsh
$basedir=Split-Path $MyInvocation.MyCommand.Definition -Parent

$exe=""
if ($PSVersionTable.PSVersion -lt "6.0" -or $IsWindows) {
  # Fix case when both the Windows and Linux builds of Node
  # are installed in the same directory
  $exe=".exe"
}
$ret=0
if (Test-Path "$basedir/node$exe") {
  # Support pipeline input
  if ($MyInvocation.ExpectingInput) {
    $input | & "$basedir/node$exe"  "$basedir/../{{package_name}}/{{script_path}}" $args
  } else {
    & "$basedir/node$exe"  "$basedir/../{{package_name}}/{{script_path}}" $args
  }
  $ret=$LASTEXITCODE
} else {
  # Support pipeline input
  if ($MyInvocation.ExpectingInput) {
    $input | & "node$exe"  "$basedir/../{{package_name}}/{{script_path}}" $args
  } else {
    & "node$exe"  "$basedir/../{{package_name}}/{{script_path}}" $args
  }
  $ret=$LASTEXITCODE
}
exit $ret
    "#;

    script.replace("{{package_name}}", name).replace("{{script_path}}", script_path)
}