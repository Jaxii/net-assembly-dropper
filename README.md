# net-assembly-dropper
A proof of concept file dropper utilizing PowerShell loosely based off of https://github.com/ch2sh/Jlaive 

**Results:

before: ![image](https://user-images.githubusercontent.com/42078529/169199884-6b8f7605-db08-435d-82ca-4935b1d86a79.png)
*generic reverse tcp shell off github*
after: ![image](https://user-images.githubusercontent.com/42078529/169199902-304cef2c-fa87-4c4a-a7e1-85f9529cfd40.png)

**Todo:
- Reduce binary size
- DInvoke AMSI bypass instead of powershell based
- rewrite powershell builder and add "-ExecutionPolicy Bypass" flag
