# net-assembly-dropper
A proof of concept file dropper utilizing PowerShell loosely based off of https://github.com/ch2sh/Jlaive 

note: only works with .NET assemblies due to use of Assembly.Load

**Results:**

before: ![image](https://user-images.githubusercontent.com/42078529/169199884-6b8f7605-db08-435d-82ca-4935b1d86a79.png)
*generic reverse tcp shell off github* (https://github.com/ihack4falafel/ReverseShell)

after: ![image](https://user-images.githubusercontent.com/42078529/169199902-304cef2c-fa87-4c4a-a7e1-85f9529cfd40.png)



**Usage:**

Replace "C:\\Users\\Desktop\\File.exe" with the path to your file and build the project in release mode. The resulting binary will have your original file embedded and AES encrypted.


**Todo:**
- Reduce binary size
- DInvoke AMSI bypass: https://github.com/Kudaes/DInvoke_rs 
- Rewrite powershell builder and add "-ExecutionPolicy Bypass" flag
- Add anti-VM
- Native support
