use regex::Regex;

fn main() {
    // get file with rfd
    let file = rfd::FileDialog::new()
        .set_title("Select the COMPONENTS Hive")
        .set_file_name("COMPONENTS")
        .pick_file();
    // load hive "HKLM\COMPONENTS", usually hidden
    let hive = winreg::RegKey::load_app_key(file.unwrap(), true);
    let components = hive
        .unwrap()
        .open_subkey("DerivedData\\Components")
        .unwrap();
    let components_vec = components
        .enum_keys()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    // use regex to match the component name with something that contains "directui"
    let scan = [
        // some filters start abrubtly because of microsoft shortening strings for path limit
        "microsoft-windows-shell-wallpaper-theme",
        "microsoft-windows-t..nbackgrounds-client", // img101-104.png
        "microsoft-windows-storagemigration",
        "xtservice-migration",
        "upgrade.resources",
        "microsoft-windows-shell-grouppolicy", // adml/admx files
        "ouppolicy.resources",
        "microsoft-windows-security-spp", // sppmig.dll
        "wallpaper-windows",
        "calaccountmigplugin",                 // NgcLocalAccountMigPlugin.dll
        "microsoft-windows-pnpmigration",      // pnpmig.dll
        "unterinfrastructure",                 // CntrtextMig.dll
        "microsoft-windows-mediaplayer-core",  // wmpshare
        "ntrolpanel.appxmain",                 // multiple .pngs
        "microsoft-windows-tcpip",             // netiomig.dll
        "microsoft-windows-shmig",             // shmig.dll
        "mentation-migration",                 // wininetplugin.dll
        "windowssearchengine",                 // WSearchMigPlugin.dll
        "service-migration",                   // ClipMigPlugin.dll
        "microsoft-windows-bth-user",          // BthMigPlugin.dll
        "framework-migration",                 // msctfmig
        "microsoft-windows-wmi-core",          // WMIMigrationPlugin.dll
        "rastructure-upgrade",                 // WsUpgrade.dll
        "microsoft-windows-sxs",               // various .dlls
        "microsoft-windows-hwvid",             // hwvidmigplugin.dll
        "netfx4-netfx_upgradecleanup",         // netfx45_upgradecleanup.inf
        "microsoft-windows-com-complus-setup", // conmig.dll
        "microsoft-gaming-ga..rnal-presencewriter", // gamebarpresencewriter
        "presencewriter",                      // gamebarpresencewriter
        "microsoft-windows-appx",              // AppxUpgradeMigrationPlugin.dll
        "microsoft-windows-appx-deployment-server", // AppxUpgradeMigrationPlugin.dll
        "microsoft-onecore-tiledatarepository", // TileStoreMigrationPlugin.dll
        "microsoft-windows-deviceaccess",      // dabmigplugin.dll
        "nframeworkmigration",                 // dafmigplugin.dll
        "microsoft-windows-ui-shellcommon",    // People*
        "microsoft-windows-ui-pcshell",        // People*
        "multimedia-other",                    // audmigplugin.dll
        "netfx-wcf-migration",                 // ServiceModelRegMigPlugin
        "anagement-migration",                 // AppManMigrationPlugin.dll
        "microsoft-windows-usbmigplugin",      // UsbMigPlugin.dll
        "microsoft-windows-mup",               // MupMigPlugin.dll
        "update-authenticamd",                 // mcupdate
        "update-genuineintel",                 // mcupdate
        "microsoft-windows-onedrive-setup",    // OneDriveSetup.exe
    ]
    .to_vec();
    // build regex statement from scan vector
    let re = Regex::new(&scan.join("|")).unwrap();
    // iterate through the vector and delete the component if it matches the regex
    for component in components_vec {
        let component_name = &component;
        if re.is_match(component_name) {
            println!("{}", component_name);
            components.delete_subkey(component_name).unwrap();
        }
    }
}
