#[derive(Clone)]
pub struct InstallationEntry {
    pub name: String,

    /// Additional properties specified on the command line. These are arbitrary String-String mappings passed as long arguments
    /// by the user, and are used for distro-specific or package-manager-specific operations. For example, on Arch linux, a user
    /// may run `darling install joshuto --source=aur` to install a package such as joshuto from the AUR.
    pub properties: std::collections::HashMap<String, String>,
}

pub struct Context {
    pub config: DarlingConfig,
}

pub struct DarlingConfig {
    pub source_location: String,
}

impl std::default::Default for DarlingConfig {
    fn default() -> Self {
        Self {
            source_location: std::env::var("HOME").unwrap() + "/.local/share/darling/source",
        }
    }
}

pub trait PackageManager: Send + Sync {
    fn name(&self) -> String;

    /// Installs a package with the given version. If no version is supplied, this should install the latest version.
    /// Note that this ***does not*** affect the cache file. This simply supplies the system package install command.
    ///
    /// # Parameters
    /// - `context` - The darling context, which provides global immutable information about the program.
    /// - `package` - The name of the package to install.
    ///
    /// # Returns
    /// An error if the package could not be installed.
    fn install(&self, context: &Context, package: &InstallationEntry) -> anyhow::Result<()>;

    /// This is run after a single or group of packages are installed. The difference between placing code here and in
    /// [install] is that when running commands like `load-installed`, which load all installed packages into the
    /// darling config file, this is only run once after all packages are installed, instead of every time an individual
    /// package is installed.
    ///
    /// This is useful for modules such as the core module which needs to rebuild the source code every time a new
    /// module is added. With this system, we can just rebuild the source once after all of the modules are added,
    /// instead of every time each individual module is added.
    ///
    /// **Note that this will still be run for individual installations *after* the [install] method**.
    ///
    /// This method is optional, and has a default implementation of just `Ok(())`.
    ///
    /// # Parameters
    /// - `context` - The darling context, which provides global immutable information about the program.
    ///
    /// # Returns
    /// An error if anything went wrong in the post-installation process. This is different module-to-module so
    /// no more information than this can be specified here.
    fn post_install(&self, _context: &Context) -> anyhow::Result<()> {
        Ok(())
    }

    /// Uninstalls a package from the system. This does ***not*** affect the cache file, it simply removes the package
    /// from the system itself, and `darling-core` will handle removing the package from the cache file.
    ///
    /// # Parameters
    /// - `context` - The darling context, which provides global immutable information about the program.
    /// - `package` - The name of the package to remove.
    ///
    /// # Returns
    /// An error if the package could not be removed.
    fn uninstall(&self, context: &Context, package: &InstallationEntry) -> anyhow::Result<()>;

    /// Returns all *explicitly* installed packages on the system; That is, packages which are not dependencies of
    /// other packages. This **should not** read from a darling file; Instead, darling uses this method to update
    /// the file when running `darling require-all`
    ///
    /// # Parameters
    /// - `context` - The darling context, which provides global immutable information about the program.
    ///
    /// # Returns
    /// The name and version of each installed package. as a `Vec<(name: String, version: String)>`.
    fn get_all_explicit(&self, context: &Context) -> anyhow::Result<Vec<(String, String)>>;
}
