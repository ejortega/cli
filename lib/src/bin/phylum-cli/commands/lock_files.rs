use std::path::Path;

use phylum_cli::lockfiles::{
    GemLock, PackageLock, Parseable, PipFile, Pom, PyRequirements, YarnLock,
};
use phylum_cli::types::{PackageDescriptor, PackageType};

/// Attempt to get packages from an unknown lockfile type
pub fn try_get_packages(path: &Path) -> Option<(Vec<PackageDescriptor>, PackageType)> {
    log::warn!(
        "Attempting to obtain packages from unrecognized lockfile type: {}",
        path.to_string_lossy()
    );

    let packages = YarnLock::new(path).ok()?.parse();
    if packages.is_ok() {
        log::debug!("Submitting file as type yarn lock");
        return packages.ok().map(|pkgs| (pkgs, PackageType::Npm));
    }

    let packages = PackageLock::new(path).ok()?.parse();
    if packages.is_ok() {
        log::debug!("Submitting file as type package lock");
        return packages.ok().map(|pkgs| (pkgs, PackageType::Npm));
    }

    let packages = GemLock::new(path).ok()?.parse();
    if packages.is_ok() {
        log::debug!("Submitting file as type gem lock");
        return packages.ok().map(|pkgs| (pkgs, PackageType::Ruby));
    }

    let packages = PyRequirements::new(path).ok()?.parse();
    if packages.is_ok() {
        log::debug!("Submitting file as type pip requirements.txt");
        return packages.ok().map(|pkgs| (pkgs, PackageType::Python));
    }

    let packages = PipFile::new(path).ok()?.parse();
    if packages.is_ok() {
        log::debug!("Submitting file as type pip Pipfile or Pipfile.lock");
        return packages.ok().map(|pkgs| (pkgs, PackageType::Python));
    }

    let packages = Pom::new(path).ok()?.parse();
    if packages.is_ok() {
        log::debug!("Submitting file as type pom xml");
        return packages.ok().map(|pkgs| (pkgs, PackageType::Java));
    }

    log::error!("Failed to identify lock file type");
    None
}

/// Determine the lockfile type based on its name and parse
/// accordingly to obtain the packages from it
pub fn get_packages_from_lockfile(path: &str) -> Option<(Vec<PackageDescriptor>, PackageType)> {
    let path = Path::new(path);
    let file = path.file_name()?.to_str()?;

    let res = match file {
        "Gemfile.lock" => {
            let parser = GemLock::new(path).ok()?;
            parser.parse().ok().map(|pkgs| (pkgs, PackageType::Ruby))
        }
        "package-lock.json" => {
            let parser = PackageLock::new(path).ok()?;
            parser.parse().ok().map(|pkgs| (pkgs, PackageType::Npm))
        }
        "yarn.lock" => {
            let parser = YarnLock::new(path).ok()?;
            parser.parse().ok().map(|pkgs| (pkgs, PackageType::Npm))
        }
        "requirements.txt" => {
            let parser = PyRequirements::new(path).ok()?;
            parser.parse().ok().map(|pkgs| (pkgs, PackageType::Python))
        }
        "Pipfile.txt" | "Pipfile.lock" => {
            let parser = PipFile::new(path).ok()?;
            parser.parse().ok().map(|pkgs| (pkgs, PackageType::Python))
        }
        "pom.xml" => {
            let parser = GemLock::new(path).ok()?;
            parser.parse().ok().map(|pkgs| (pkgs, PackageType::Ruby))
        }
        _ => try_get_packages(path),
    };

    let pkg_count = res.as_ref().map(|p| p.0.len()).unwrap_or_default();

    log::debug!("Read {} packages from file `{}`", pkg_count, file);

    res
}
