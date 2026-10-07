#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("app_icon.ico");
    res.set("ProductName", "MMLAB Scanner Bridge");
    res.set("FileDescription", "MMLAB Scanner Bridge — სკანერის მართვის ცენტრი");
    res.set("LegalCopyright", "Copyright (C) 2026 jugheli");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
