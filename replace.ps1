Get-ChildItem -Path "G:\myself_rust_p\rquest-util\src\emulation\device" -Include *.rs -Recurse | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    $content = $content -replace 'CertificateCompressionAlgorithm','CertCompressionAlgorithm'
    $content = $content -replace 'TlsOptions','TlsConfig'
    $content = $content -replace 'Http2Options','Http2Config'
    $content = $content -replace 'AlpsProtocol','AlpsProtos'
    $content = $content -replace 'AlpnProtocol','AlpnProtos'
    Set-Content -Path $_.FullName -Value $content
}
Write-Host "Batch replacement complete"
