use chrono::{DateTime, Local, Utc};
use std::time::{Duration, SystemTime};
use windows::Win32::Storage::Vss::{
    VSS_SNAPSHOT_STATE, VSS_SS_ABORTED, VSS_SS_COMMITTED, VSS_SS_COUNT, VSS_SS_CREATED,
    VSS_SS_DELETED, VSS_SS_POSTCOMMITTED, VSS_SS_PRECOMMITTED, VSS_SS_PREFINALCOMMITTED,
    VSS_SS_PREPARED, VSS_SS_PREPARING, VSS_SS_PROCESSING_COMMIT, VSS_SS_PROCESSING_POSTCOMMIT,
    VSS_SS_PROCESSING_POSTFINALCOMMIT, VSS_SS_PROCESSING_PRECOMMIT,
    VSS_SS_PROCESSING_PREFINALCOMMIT, VSS_SS_PROCESSING_PREPARE, VSS_VOLSNAP_ATTR_AUTORECOVER,
    VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE, VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT,
    VSS_VOLSNAP_ATTR_DIFFERENTIAL, VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY,
    VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY, VSS_VOLSNAP_ATTR_FILE_SHARE,
    VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED, VSS_VOLSNAP_ATTR_IMPORTED, VSS_VOLSNAP_ATTR_NOT_SURFACED,
    VSS_VOLSNAP_ATTR_NOT_TRANSACTED, VSS_VOLSNAP_ATTR_NO_AUTORECOVERY,
    VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE, VSS_VOLSNAP_ATTR_NO_WRITERS, VSS_VOLSNAP_ATTR_PERSISTENT,
    VSS_VOLSNAP_ATTR_PLEX, VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY, VSS_VOLSNAP_ATTR_TRANSPORTABLE,
    VSS_VOLSNAP_ATTR_TXF_RECOVERY,
};

pub fn get_string_for_snapshot_state(v: VSS_SNAPSHOT_STATE) -> String {
    let res = match v {
        VSS_SS_PREPARING => "VSS_SS_PREPARING",
        VSS_SS_PROCESSING_PREPARE => "VSS_SS_PROCESSING_PREPARE",
        VSS_SS_PREPARED => "VSS_SS_PREPARED",
        VSS_SS_PROCESSING_PRECOMMIT => "VSS_SS_PROCESSING_PRECOMMIT",
        VSS_SS_PRECOMMITTED => "VSS_SS_PRECOMMITTED",
        VSS_SS_PROCESSING_COMMIT => "VSS_SS_PROCESSING_COMMIT",
        VSS_SS_COMMITTED => "VSS_SS_COMMITTED",
        VSS_SS_PROCESSING_POSTCOMMIT => "VSS_SS_PROCESSING_POSTCOMMIT",
        VSS_SS_PROCESSING_PREFINALCOMMIT => "VSS_SS_PROCESSING_PREFINALCOMMIT",
        VSS_SS_PREFINALCOMMITTED => "VSS_SS_PREFINALCOMMITTED",
        VSS_SS_PROCESSING_POSTFINALCOMMIT => "VSS_SS_PROCESSING_POSTFINALCOMMIT",
        VSS_SS_CREATED => "VSS_SS_CREATED",
        VSS_SS_ABORTED => "VSS_SS_ABORTED",
        VSS_SS_DELETED => "VSS_SS_DELETED",
        VSS_SS_POSTCOMMITTED => "VSS_SS_POSTCOMMITTED",
        VSS_SS_COUNT => "VSS_SS_COUNT",
        _ => "VSS_SS_UNKNOWN",
    };

    res.to_owned()
}

pub(crate) fn i64_to_date(t: i64) -> DateTime<Utc> {
    let v = t as u64;
    const NANOS_PER_SEC: u64 = 1_000_000_000;
    const INTERVALS_PER_SEC: u64 = NANOS_PER_SEC / 100;
    const INTERVALS_TO_UNIX_EPOCH: u64 = 11_644_473_600 * INTERVALS_PER_SEC;

    if v < INTERVALS_TO_UNIX_EPOCH {
        return SystemTime::now().into();
    }

    let t = SystemTime::UNIX_EPOCH + Duration::from_nanos((v - INTERVALS_TO_UNIX_EPOCH) * 100);
    let l: DateTime<Local> = t.into();
    DateTime::from(l)
}

pub(crate) fn u16_to_string(ptr: *const u16) -> String {
    let len = unsafe { (0..).take_while(|&i| *ptr.offset(i) != 0).count() };
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };

    String::from_utf16_lossy(
        &slice
            .iter()
            .cloned()
            .take_while(|&n| n != 0)
            .collect::<Vec<u16>>(),
    )
}

pub(crate) fn volsnap_attrs_to_str(attr: i32) -> Vec<String> {
    let mut attrs = Vec::new();

    if attr & VSS_VOLSNAP_ATTR_PERSISTENT.0 > 0 {
        attrs.push("Persistent".to_owned());
    }
    if attr & VSS_VOLSNAP_ATTR_NO_AUTORECOVERY.0 > 0 {
        attrs.push("No_AutoRecovery".to_owned());
    }
    if attr & VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE.0 > 0 {
        attrs.push("Client_accessible".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE.0 > 0 {
        attrs.push("No_Auto_Release".to_owned());
    } else {
        attrs.push("Auto_Release".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_NO_WRITERS.0 > 0 {
        attrs.push("No_Writers".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_TRANSPORTABLE.0 > 0 {
        attrs.push("Transportable".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_NOT_SURFACED.0 > 0 {
        attrs.push("Not_Surfaced".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_NOT_TRANSACTED.0 > 0 {
        attrs.push("Not_Transacted".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED.0 > 0 {
        attrs.push("Hardware".to_owned());
    }
    if attr & VSS_VOLSNAP_ATTR_DIFFERENTIAL.0 > 0 {
        attrs.push("Differential".to_owned());
    }
    if attr & VSS_VOLSNAP_ATTR_PLEX.0 > 0 {
        attrs.push("Plex".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_IMPORTED.0 > 0 {
        attrs.push("Imported".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY.0 > 0 {
        attrs.push("Exposed_Locally".to_owned());
    }
    if attr & VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY.0 > 0 {
        attrs.push("Exposed_Remotely".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_AUTORECOVER.0 > 0 {
        attrs.push("Autorecover".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY.0 > 0 {
        attrs.push("Rollback_Recovery".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT.0 > 0 {
        attrs.push("Delayed_Postsnapshot".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_TXF_RECOVERY.0 > 0 {
        attrs.push("Txf_Recovery".to_owned());
    }

    if attr & VSS_VOLSNAP_ATTR_FILE_SHARE.0 > 0 {
        attrs.push("File_Share".to_owned());
    }
    return attrs;
}
