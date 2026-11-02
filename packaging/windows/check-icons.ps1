<#
.SYNOPSIS
  Check MSI icon references without building or installing LocalVector.

.DESCRIPTION
  ICE50 requires advertised shortcut icon identifiers to have the target file's extension.
  Windows Installer uses those identifiers as cached filenames; extensionless identifiers
  can display a generic document icon even when the ICO data and application are valid.
#>
param(
  [string] $WxsPath = (Join-Path $PSScriptRoot 'localvector.wxs')
)
$ErrorActionPreference = 'Stop'
[xml] $Wxs = Get-Content -LiteralPath $WxsPath -Raw
$Ns = [Xml.XmlNamespaceManager]::new($Wxs.NameTable)
$Ns.AddNamespace('w', 'http://wixtoolset.org/schemas/v4/wxs')
$Icons = @{}
foreach ($icon in $Wxs.SelectNodes('//w:Icon', $Ns)) {
  $Icons[$icon.GetAttribute('Id')] = $icon
}

foreach ($shortcut in $Wxs.SelectNodes('//w:File/w:Shortcut[@Advertise="yes"]', $Ns)) {
  $id = $shortcut.GetAttribute('Id')
  $iconId = $shortcut.GetAttribute('Icon')
  if (-not $Icons.ContainsKey($iconId)) {
    throw "Advertised shortcut '$id' references missing icon '$iconId'."
  }
  $iconExtension = [IO.Path]::GetExtension($iconId)
  if ($iconExtension -notin '.exe', '.ico') {
    throw "Advertised shortcut '$id' icon '$iconId' must have an .exe or .ico extension (ICE50)."
  }
  $file = $shortcut.ParentNode
  $fileName = if ($file.HasAttribute('Name')) { $file.GetAttribute('Name') } else { $file.GetAttribute('Source') }
  # Source paths use Windows separators even when this check runs on Linux.
  $fileName = ($fileName -split '[\\/]')[-1]
  $targetExtension = [IO.Path]::GetExtension($fileName)
  if ($iconExtension -ne $targetExtension) {
    throw "Advertised shortcut '$id' icon '$iconId' must match target '$fileName' extension '$targetExtension' (ICE50)."
  }
}

$arpIcon = $Wxs.SelectSingleNode('//w:Property[@Id="ARPPRODUCTICON"]', $Ns)
if ($arpIcon -and -not $Icons.ContainsKey($arpIcon.GetAttribute('Value'))) {
  throw "ARPPRODUCTICON references missing icon '$($arpIcon.GetAttribute('Value'))'."
}

# Plain (non-advertised) shortcuts still cache their icon under the <Icon Id>, so their identifiers
# carry the target's .exe extension too.
foreach ($shortcut in $Wxs.SelectNodes('//w:File//w:Shortcut', $Ns)) {
  if ($shortcut.GetAttribute('Advertise') -ne 'yes') {
    $iconId = $shortcut.GetAttribute('Icon')
    if ($iconId -and $Icons.ContainsKey($iconId) -and [IO.Path]::GetExtension($iconId) -ne '.exe') {
      throw "Shortcut '$($shortcut.GetAttribute('Id'))' icon '$iconId' must have the .exe extension of its target (ICE50)."
    }
  }
}
foreach ($shortcut in $Wxs.SelectNodes('//w:Component/w:Shortcut', $Ns)) {
  $iconId = $shortcut.GetAttribute('Icon')
  if ($iconId -and $Icons.ContainsKey($iconId) -and [IO.Path]::GetExtension($iconId) -ne '.exe') {
    throw "Shortcut '$($shortcut.GetAttribute('Id'))' icon '$iconId' must have the .exe extension of its target (ICE50)."
  }
}
Write-Output 'Windows shortcut icon references ok'
