Set-Location -Path "C:\Users\Logan Saso\Dev\new\rust\automator"
Start-Process -WindowStyle hidden -FilePath python -ArgumentList "src\discord_monitor\monitor.py" -WorkingDirectory "C:\Users\Logan Saso\Dev\new\rust\automator"
Start-Process -WindowStyle hidden -FilePath cargo -ArgumentList "run" -WorkingDirectory "C:\Users\Logan Saso\Dev\new\rust\automator"
