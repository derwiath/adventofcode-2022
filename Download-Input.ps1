# This script requires a cookie file with the session token
# Put this in .cookie.txt:
# -- BEGIN --
# # Netscape HTTP Cookie File
# .adventofcode.com	TRUE	/	FALSE	0	session	<token-copied-from-browser-devtools>
# -- END --

Param(
	[parameter(Mandatory=$true)]
	[ValidateRange(1, 25)]
	[int]
	$Day,

	[String]
	$Filename="input.txt",

	[String]
	$CookieFilename=".cookie.txt",

	[switch]
	$Force=$false

)
$OutputDir="day$Day"
$OutputPath="$OutputDir/$Filename"
$UserAgent="Download-Input.ps1 by github.com/derwiath via cURL"
$URL="https://adventofcode.com/2022/day/$Day/input"

If (-Not (Test-Path -Path "$CookieFilename" -PathType Leaf)) {
	Write-Error "Failed to find $CookieFilename"
	Exit 1
}
ElseIf (-Not (Test-Path -Path "$OutputDir" -PathType Container)) {
	Write-Error "Failed to find $OutputDir."
	Exit 1
}
ElseIf ((-Not $Force) -And (Test-Path -Path "$OutputPath" -PathType Leaf)) {
	Write-Error "$OutputPath already exists. Use -Force to overwrite."
	Exit 1
}

Write-Host "curl --cookie $CookieFilename -A `"$UserAgent`" $URL"
curl --cookie $CookieFilename -A `"$UserAgent`" $URL | Set-Content $OutputPath

Exit $LASTEXITCODE
