use vshadow_rs::{vssclient::VssClient, vssprop::VSSProp};
use windows::{core::GUID, Win32::Storage::Vss::VSS_CTX_ALL};

#[derive(Debug, Default)]
pub struct Args {
    pub create: bool,
    /// Creates persistent shadow copies
    pub persistent: bool,
    /// Creates shadow copies without involving writers
    pub no_wirters: bool,
    /// Creates Differential Hardware shadow copies
    pub add_differential: bool,
    /// Creates Plex Hardware shadow copies
    pub add_plex: bool,
    /// Creates Shadow Copies for Shared Folders (Client accessible)
    pub create_shadow_copy_for_shared_folders: bool,
    /// Verifies that a certain writer/component is included
    pub writer_included: Option<String>,
    /// Excludes a certain writer/component from the shadow copy
    pub writer_excluded: Option<String>,
    /// Creates a transportable shadow copy and saves the Backup Components
    /// document into the given file. This file can be used in a subsequent Import and/or restore.
    pub transportable: Option<String>,
    /// Creates a non-transportable shadow copy and saves the Backup Components
    /// document into the given file. This file can be used in a subsequent restore.
    pub non_transportable: Option<String>,
    /// Generates a CMD file containing  environment variables related to created
    /// shadow copies (the shadow copy IDs, the shadow copy set ID, etc)
    pub script: Option<String>,
    /// list of volumes for creation snapshot
    pub volumes: Vec<String>,
    /// shadow copy import
    pub import: bool,
    /// The {file.xml} file must be a backup components file previously created with the –t option.
    pub import_file: Option<String>,
    /// Executes a shell command between the shadow set creation and
    /// VSHADOW program exit. Useful for non-persistent shadow copies.
    pub exec: Option<String>,
    pub query: bool,
    pub delete: bool,
    pub breaks: bool,
    /// Break the shadow copy set into standalone writable volumes
    pub writable: bool,
    pub exposing: bool,
    /// local dir or unused drive letter
    pub local: Option<String>,
    /// unused shared names and optional path from root on shadow
    pub remote: Option<String>,
    /// writer operator
    pub writers: bool,
    /// writer list
    pub writer_status: bool,
    /// writer summary meta
    pub writer_meta: bool,
    /// writer full meta
    pub writer_meta2: bool,
    pub wrestore: bool,
    /// The {file.xml} file must be a backup components file previously created during a shadow copy
    /// creation with either the –t or –bc options.
    pub wr_file: Option<String>,
    ///  perform a simulated restore
    pub wr_simulate: bool,

    /// all
    pub all: bool,
    /// snapshot set id
    pub snapshot_set_id: Option<String>,
    /// snapshot id
    pub snapshot_id: Option<String>,
    /// Wait for the user interaction before exiting. This will keep alive non-persistent shadows.
    pub wait: bool,
    /// Verbose output – useful for diagnosis.
    pub tracing: bool,
}

fn delete(comm: &Args) -> ::windows::core::Result<()> {
    assert!(comm.delete);
    let mut client = VssClient::default();
    client.initialize(VSS_CTX_ALL, None, false)?;
    if comm.all {
        tracing::debug!("(Option: Delete all shadow copies)");
        client.delete_all_snapshots()
    } else if comm.snapshot_id.is_some() {
        let snapshot_id = GUID::try_from(comm.snapshot_id.clone().unwrap().as_str()).unwrap();
        client.delete_snapshot(snapshot_id)
    } else if comm.snapshot_set_id.is_some() {
        let snapshot_set_id =
            GUID::try_from(comm.snapshot_set_id.clone().unwrap().as_str()).unwrap();
        client.delete_snapshotset(snapshot_set_id)
    } else {
        Ok(())
    }
}

fn query(comm: &Args) -> ::windows::core::Result<Vec<VSSProp>> {
    //valid
    assert!(comm.query);
    let mut client = VssClient::default();
    client.initialize(VSS_CTX_ALL, None, false)?;
    let res = if comm.all {
        tracing::debug!("(Option: Query all shadow copies)");
        let res = client.query_snapshot_set(GUID::default())?;
        res
    } else if comm.snapshot_set_id.is_some() {
        tracing::debug!("(Option: Query shadow copy)");
        let id = GUID::try_from(comm.snapshot_set_id.clone().unwrap().as_str()).unwrap();
        let res = client.query_snapshot_set(id)?;
        res
    } else if comm.snapshot_id.is_some() {
        tracing::debug!("(Option: Query shadow copy set)");
        let id = GUID::try_from(comm.snapshot_id.clone().unwrap().as_str()).unwrap();
        let res = client.get_snapshot_properties(id)?;
        let mut r = Vec::new();
        r.push(res);
        r
    } else {
        let res = Vec::new();
        res
    };

    Ok(res)
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    // remove first command
    args.pop();
    let command = parse_args(&args);
    if command.query {
        let res = query(&command).unwrap();
        println!("{:#?}", res);
        return;
    }

    if command.delete {
        delete(&command).unwrap();
        return;
    }
}

fn parse_args(args: &[String]) -> Args {
    let mut command = Args::default();
    for arg in args {
        match arg.as_str() {
            "-p" => {
                command.create = true;
                command.persistent = true;
            }
            "-nw" => {
                command.create = true;
                command.no_wirters = true;
            }
            "-ad" => {
                command.create = true;
                command.add_differential = true;
            }
            "-ap" => {
                command.create = true;
                command.add_plex = true;
            }
            "-scsf" => {
                command.create = true;
                command.create_shadow_copy_for_shared_folders = true;
            }

            "-q" => {
                command.query = true;
                command.all = true;
            }

            "-da" => {
                command.delete = true;
                command.all = true;
            }

            "-ws" => {
                command.writers = true;
                command.writer_status = true;
            }
            "-wm" => {
                command.writers = true;
                command.writer_meta = true;
            }
            "-wm2" => {
                command.writers = true;
                command.writer_meta2 = true;
            }

            "-wait" => {
                command.wait = true;
            }
            "-tracing" => {
                command.tracing = true;
            }
            s => {
                if s.starts_with("-") {
                    match split_kv(s) {
                        (s, None) => panic_on_key_empty(s.as_str()),
                        (k, Some(v)) => match k.as_str() {
                            "-wi" => {
                                command.create = true;
                                command.writer_included = Some(v);
                            }
                            "-wx" => {
                                command.create = true;
                                command.writer_excluded = Some(v);
                            }
                            "-script" => {
                                command.create = true;
                                command.script = Some(v);
                            }
                            "-exec" => {
                                command.exec = Some(v);
                            }
                            "-i" => {
                                command.import = true;
                                command.import_file = Some(v);
                            }
                            "-qx" => {
                                command.query = true;
                                command.snapshot_set_id = Some(v);
                            }
                            "-s" => {
                                command.query = true;
                                command.snapshot_id = Some(v);
                            }
                            "-dx" => {
                                command.delete = true;
                                command.snapshot_set_id = Some(v);
                            }
                            "-ds" => {
                                command.delete = true;
                                command.snapshot_id = Some(v);
                            }
                            "-b" => {
                                command.breaks = true;
                                command.snapshot_set_id = Some(v);
                            }
                            "-bw" => {
                                command.breaks = true;
                                command.snapshot_id = Some(v);
                            }
                            "-el" => {
                                command.exposing = true;
                                command.local = Some(v);
                            }
                            "-er" => {
                                command.exposing = true;
                                command.remote = Some(v);
                            }
                            "-r" => {
                                command.wrestore = true;
                                command.wr_file = Some(v);
                            }
                            "-rs" => {
                                command.wrestore = true;
                                command.wr_simulate = true;
                                command.wr_file = Some(v);
                            }
                            u => panic!("unspported key {}", u),
                        },
                    }
                } else {
                    command.create = true;
                    command.volumes.push(s.to_owned());
                }
                // create
            }
        }
    }

    return command;
}

fn panic_on_key_empty(key: &str) {
    panic!("{} is empty, donot specific if no need", key)
}

pub fn split_kv(kv: &str) -> (String, Option<String>) {
    match kv.split_once("=") {
        None => (kv.to_owned(), None),
        Some((a, b)) => (
            a.to_owned(),
            if b.len() > 0 {
                Some(b.to_owned())
            } else {
                None
            },
        ),
    }
}

#[cfg(test)]
mod test {
    use crate::split_kv;

    #[test]
    fn test_kv() {
        assert_eq!(
            split_kv("-bc={file.xml}"),
            ("-bc".to_owned(), Some("{file.xml}".to_owned()))
        );

        assert_eq!(split_kv("-bc"), ("-bc".to_owned(), None));
        assert_eq!(split_kv("-bc="), ("-bc".to_owned(), None));
    }
}
