use talpid_types::ErrorExt;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

const NETCLS_PATH: &str = "/sys/fs/cgroup/net_cls/";
/// Identifies packets coming from the cgroup.
pub const NETCLS_CLASSID: u32 = 0x4d9f41;
const CGROUP_NAME: &str = "mullvad-exclusions";

/// Errors related to split tunneling.
#[derive(err_derive::Error, Debug)]
#[error(no_from)]
pub enum Error {
    /// Unable to create cgroup.
    #[error(display = "Unable to create cgroup for excluded processes")]
    CreateCGroup(#[error(source)] io::Error),

    /// Unable to set class ID for cgroup.
    #[error(display = "Unable to set cgroup class ID")]
    SetCGroupClassId(#[error(source)] io::Error),
}

/// Set up cgroup used to track PIDs for split tunneling.
pub fn create_cgroup() -> Result<(), Error> {
    let mut exclusions_dir = PathBuf::from(NETCLS_PATH);
    exclusions_dir.push(CGROUP_NAME);

    if !exclusions_dir.exists() {
        fs::create_dir(exclusions_dir.clone()).map_err(Error::CreateCGroup)?;
    }

    let mut classid_file = PathBuf::from(exclusions_dir);
    classid_file.push("net_cls.classid");
    fs::write(classid_file, NETCLS_CLASSID.to_string().as_bytes())
        .map_err(Error::SetCGroupClassId)
}