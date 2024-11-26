# Replace  with ridge in cliff.toml
(Get-Content -Path cliff.toml) -replace '<<github-repository-placeholder>>', 'https://github.com/vainjoker/ridge' | Set-Content -Path cliff.toml

# Create remote repository and link it to local repository
gh repo create vainjoker/ridge --public

# Execute release-plz init
release-plz init

# Generate Codecov token and open settings page
$codecov_url = "https://app.codecov.io/github/vainjoker/ridge/settings/"
Start-Process $codecov_url

Write-Host "Please complete the Codecov setup in the opened browser window. And copy the token to the github repository settings."
Write-Host "Press Enter when you've finished setting up Codecov..."
Read-Host

# Link Mergify
$mergify_url = "https://dashboard.mergify.com/workflow-automation?repository=ridge&login=vainjoker"
Start-Process $mergify_url

Write-Host "Please complete the Mergify setup in the opened browser window. And active the workflow."
Write-Host "Press Enter when you've finished setting up Mergify..."
Read-Host

Write-Host "Setup process completed."

# 删除 generate 文件
Remove-Item $MyInvocation.MyCommand.Path

Write-Host "The generate script has been removed."
