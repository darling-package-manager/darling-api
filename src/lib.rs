/// Data about an (un)installation command. This contains the name of the package to install, as well as
/// a map of properties specified on the command line.
#[derive(Clone)]
pub struct InstallationEntry {
    /// The name of the package. This is a unique identifier, not a human readable string.
    pub name: String,

    /// Additional properties specified on the command line. These are arbitrary String-String mappings passed as long arguments
    /// by the user, and are used for distro-specific or package-manager-specific operations. For example, on Arch linux, a user
    /// may run `darling install joshuto --source=aur` to install a package such as joshuto from the AUR.
    pub properties: std::collections::HashMap<String, String>,
}

/// Global immutable data about the current darling session. This is currently almost entirely unused, but various
/// configurations in the future are going to go here.
pub struct Context {
    /// The configuration cative when running darling.
    pub config: DarlingConfig,
}

/// The user-defined configuration options.
pub struct DarlingConfig {
    /// The location of the darling source on the users machine; `~/.local/share/darling/source` by default.
    pub source_location: String,
}

impl std::default::Default for DarlingConfig {
    fn default() -> Self {
        Self {
            source_location: std::env::var("HOME").unwrap() + "/.local/share/darling/source",
        }
    }
}

/// A package manager which gets a darling implementation. This provides the core functionality on how to install,
/// uninstall, and list packages through darling. Most of these methods when implemented commonly use
/// `std::process::Command` to install things through shell commands. In the rare case that a Rust API is available
/// and advantageous for a particular package manager, that of course could be used as well.
pub trait PackageManager: Send + Sync {
    /// Returns the name of this package manager. This is a unique all-lowercase identifier that should not conflict
    /// with any others. It's common to make this the name of the crate, without the `darling-` prefix. For example,
    /// this could return `"example".to_owned()`, and the crate would be called `darling-example`.
    ///
    /// **TODO:** This may change into returning a `&'static str`. This might be easier to handle on the receiving end
    /// (such as not having to borrow a value that's already borrowed, and we can convert it to owned when necessary),
    /// but more importantly, it'd help reinforce that this should be an unchanging constant compile-time known value.
    /// I set this as an owned `String` because they're just generally easier to work with and is the common convention
    /// for method returns, but if there aren't any bad ramifications then this option should be considered.
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
