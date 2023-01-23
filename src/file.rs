use std::{collections::{HashSet, HashMap}, ffi::OsStr};

pub fn get_supported_filetypes() -> HashMap<&'static OsStr, &'static str> {
    let mut supported_filetypes = HashMap::new();
    supported_filetypes.insert(OsStr::new("html"), "text/html");
    supported_filetypes.insert(OsStr::new("css"), "text/css");
    supported_filetypes.insert(OsStr::new("js"), "text/javascript");
    supported_filetypes.insert(OsStr::new("png"), "image/png");
    supported_filetypes.insert(OsStr::new("jpg"), "image/jpeg");
    supported_filetypes.insert(OsStr::new("jpeg"), "image/jpeg");
    supported_filetypes.insert(OsStr::new("gif"), "image/gif");
    supported_filetypes.insert(OsStr::new("svg"), "image/svg+xml");
    supported_filetypes.insert(OsStr::new("ico"), "image/x-icon");
    supported_filetypes.insert(OsStr::new("json"), "application/json");
    supported_filetypes.insert(OsStr::new("xml"), "application/xml");
    supported_filetypes.insert(OsStr::new("pdf"), "application/pdf");
    supported_filetypes.insert(OsStr::new("zip"), "application/zip");
    supported_filetypes.insert(OsStr::new("txt"), "text/plain");
    supported_filetypes.insert(OsStr::new("md"), "text/markdown");
    supported_filetypes.insert(OsStr::new("wasm"), "application/wasm");
    supported_filetypes.insert(OsStr::new("mp3"), "audio/mpeg");
    supported_filetypes.insert(OsStr::new("wav"), "audio/wav");
    supported_filetypes.insert(OsStr::new("mp4"), "video/mp4");
    supported_filetypes
}