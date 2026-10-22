!macro NSIS_HOOK_POSTINSTALL
  DetailPrint "Adding Windows Firewall rule for 35mm Club..."
  ExecWait 'netsh advfirewall firewall add rule name="35mm Club" dir=in program="$INSTDIR\${MAINBINARYNAME}.exe" action=allow enable=yes'
  ExecWait 'netsh advfirewall firewall add rule name="35mm Club" dir=out program="$INSTDIR\${MAINBINARYNAME}.exe" action=allow enable=yes'
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  DetailPrint "Removing Windows Firewall rule for 35mm Club..."
  ExecWait 'netsh advfirewall firewall delete rule name="35mm Club"'
!macroend