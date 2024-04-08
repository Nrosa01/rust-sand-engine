use app_core::api::Plugin;

type PluginLoader<'a> =
    Result<libloading::Symbol<'a, fn() -> Vec<Box<dyn Plugin>>>, libloading::Error>;

pub struct DylibLoader {
    libraries: Vec<libloading::Library>,
}

impl DylibLoader {
    pub fn new() -> DylibLoader {
        DylibLoader {
            libraries: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &str) -> Result<Vec<Box<dyn Plugin>>, String> {
        let plugin_lib = unsafe { libloading::Library::new(path) };
        if let Ok(plugin_lib) = plugin_lib {
            let plugin_loader: PluginLoader = unsafe { plugin_lib.get(b"plugin") };

            match plugin_loader {
                Ok(plugin_loader) => {
                    let result = plugin_loader();
                    self.libraries.push(plugin_lib);
                    return Ok(result);
                }
                Err(e) => {
                    return Err(format!("Error loading plugin: {:?}, at path {}", e, path));
                }
            }
        } else {
            return Err(format!("Error loading library: {:?} at path {}", plugin_lib.err(), path));
        }
    }

    pub fn extension() -> String {
        let platform = match std::env::consts::OS {
            "windows" => "windows",
            "linux" => "linux",
            "macos" => "macos",
            _ => "unknown",
        };

        let str = match platform {
            "windows" => "dll",
            "linux" => "so",
            "macos" => "dylib",
            _ => "unknown",
        };

        return str.to_string();
    }
}
