fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        // version info for Properties > Details. exes with blank metadata are more
        // likely to trip AV heuristics
        res.set("ProductName", "GBFRER Transmarvel Sigil Picker");
        res.set("FileDescription", "GBFRER Transmarvel Sigil Picker");
        res.set("CompanyName", "Evoyn");
        res.set("LegalCopyright", "Evoyn");
        res.set("OriginalFilename", "GBFRER Sigil Picker.exe");
        res.set("InternalName", "GBFRER Sigil Picker");
        if let Err(e) = res.compile() {
            println!("cargo:warning=could not embed resources: {e}");
        }
    }
    println!("cargo:rerun-if-changed=assets/icon.ico");
}
