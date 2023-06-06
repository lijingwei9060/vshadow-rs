use std::{
    iter::once,
    ptr::null_mut,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Local, Utc};
use tracing::debug;
use windows::{
    core::{Interface, BSTR, GUID, HRESULT},
    Win32::{
        Foundation::S_FALSE,
        Storage::Vss::{
            IVssAsync, IVssEnumObject, VSS_BT_FULL, VSS_CTX_BACKUP, VSS_OBJECT_NONE,
            VSS_OBJECT_PROP, VSS_OBJECT_SNAPSHOT, VSS_SNAPSHOT_CONTEXT, VSS_SNAPSHOT_PROP,
            VSS_SNAPSHOT_STATE, VSS_SS_ABORTED, VSS_SS_COMMITTED, VSS_SS_COUNT, VSS_SS_CREATED,
            VSS_SS_DELETED, VSS_SS_POSTCOMMITTED, VSS_SS_PRECOMMITTED, VSS_SS_PREFINALCOMMITTED,
            VSS_SS_PREPARED, VSS_SS_PREPARING, VSS_SS_PROCESSING_COMMIT,
            VSS_SS_PROCESSING_POSTCOMMIT, VSS_SS_PROCESSING_POSTFINALCOMMIT,
            VSS_SS_PROCESSING_PRECOMMIT, VSS_SS_PROCESSING_PREFINALCOMMIT,
            VSS_SS_PROCESSING_PREPARE, VSS_VOLSNAP_ATTR_AUTORECOVER,
            VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE, VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT,
            VSS_VOLSNAP_ATTR_DIFFERENTIAL, VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY,
            VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY, VSS_VOLSNAP_ATTR_FILE_SHARE,
            VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED, VSS_VOLSNAP_ATTR_IMPORTED,
            VSS_VOLSNAP_ATTR_NOT_SURFACED, VSS_VOLSNAP_ATTR_NOT_TRANSACTED,
            VSS_VOLSNAP_ATTR_NO_AUTORECOVERY, VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE,
            VSS_VOLSNAP_ATTR_NO_WRITERS, VSS_VOLSNAP_ATTR_PERSISTENT, VSS_VOLSNAP_ATTR_PLEX,
            VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY, VSS_VOLSNAP_ATTR_TRANSPORTABLE,
            VSS_VOLSNAP_ATTR_TXF_RECOVERY, VSS_VOLUME_SNAPSHOT_ATTRIBUTES,
        },
        System::Com::{
            CoInitialize, CoInitializeSecurity, CoUninitialize, EOAC_NONE,
            RPC_C_AUTHN_LEVEL_PKT_PRIVACY, RPC_C_IMP_LEVEL_IDENTIFY,
        },
    },
};

use crate::vssbackupcomponent::{CreateVssBackupComponents, IVssBackupComponent};

pub struct VssClient {
    co_initialize_called: bool,
    context: VSS_SNAPSHOT_CONTEXT,
    latest_snapshot_set_id: Option<GUID>,
    during_restore: bool,
    vss_object: Option<IVssBackupComponent>,
}

impl Default for VssClient {
    fn default() -> Self {
        VssClient {
            co_initialize_called: false,
            context: VSS_CTX_BACKUP,
            latest_snapshot_set_id: None,
            during_restore: false,
            vss_object: None,
        }
    }
}

impl Drop for VssClient {
    fn drop(&mut self) {
        if self.vss_object.is_some() {
            self.vss_object.take();
        }

        if self.co_initialize_called {
            unsafe {
                CoUninitialize();
            }
        }
    }
}

impl VssClient {
    /// Initialize the COM infrastructure and the internal pointers
    pub fn initialize(
        &mut self,
        context: VSS_SNAPSHOT_CONTEXT,
        xml: Option<&str>,
        restore: bool,
    ) -> ::windows::core::Result<()> {
        unsafe {
            // Initialize COM
            CoInitialize(None).unwrap();
            // Initialize COM security
            CoInitializeSecurity(
                None,                          //  Allow *all* VSS writers to communicate back!
                -1,                            //  Default COM authentication service
                None,                          //  Default COM authorization service
                None,                          //  reserved parameter
                RPC_C_AUTHN_LEVEL_PKT_PRIVACY, //  Strongest COM authentication level
                RPC_C_IMP_LEVEL_IDENTIFY,      //  Minimal impersonation abilities
                None,                          //  Default COM authentication settings
                EOAC_NONE,                     //  No special options
                None,                          //  Reserved parameter
            )
            .unwrap();
        }
        self.co_initialize_called = true;

        // Create the internal backup components object
        let vss_backup = unsafe { CreateVssBackupComponents().unwrap() };
        self.vss_object = Some(vss_backup);
        // We are during restore now?
        self.during_restore = restore;

        // Call either Initialize for backup or for restore
        let xml = match xml {
            Some(xml) => {
                let xml = xml.encode_utf16().chain(once(0)).collect::<Vec<u16>>();
                unsafe { BSTR::from_raw(xml.as_ptr()) }
            }
            None => BSTR::new(),
        };
        if self.during_restore {
            unsafe {
                self.vss_object
                    .as_ref()
                    .unwrap()
                    .InitializeForRestore(xml)
                    .unwrap()
            };
        } else {
            unsafe {
                self.vss_object
                    .as_ref()
                    .unwrap()
                    .InitializeForBackup(xml)
                    .unwrap()
            };

            // Set the context, if different than the default context
            if context != VSS_CTX_BACKUP {
                unsafe {
                    self.vss_object
                        .as_ref()
                        .unwrap()
                        .SetContext(context)
                        .unwrap();
                }
            }
        }
        // Keep the context
        self.context = context;

        // Set various properties per backup components instance
        unsafe {
            self.vss_object
                .as_ref()
                .unwrap()
                .SetBackupState(true, true, VSS_BT_FULL, false)
                .unwrap();
        }
        Ok(())
    }

    /// Waits for the completion of the asynchronous operation
    pub fn wait_and_check_for_async_operation(
        &self,
        p_async: &mut IVssAsync,
    ) -> ::windows::core::Result<()> {
        debug!("(Waiting for the asynchronous operation to finish...)");
        unsafe { p_async.Wait(u32::MAX).unwrap() };

        // Check the result of the asynchronous operation
        let mut hr_result = HRESULT::default();
        unsafe { p_async.QueryStatus(&mut hr_result, null_mut()).unwrap() };
        // Check if the async operation succeeded...
        hr_result.ok()
    }

    /// Query all the shadow copies in the given set
    /// If snapshotSetID is zeroed, just query all shadow copies in the system
    pub fn query_snapshot_set(&self, snapshot_id: GUID) -> ::windows::core::Result<Vec<VSSProp>> {
        let mut p_ienum_snapshots = ::windows::core::zeroed::<IVssEnumObject>();
        if snapshot_id == GUID::zeroed() {
            debug!("Querying all shadow copies in the system ...");
        } else {
            debug!(
                "Querying all shadow copies with the SnapshotSetID, {:?}",
                &snapshot_id
            );
        }
        let hr_result = unsafe {
            self.vss_object.as_ref().unwrap().Query(
                snapshot_id,
                VSS_OBJECT_NONE,
                VSS_OBJECT_SNAPSHOT,
                &mut p_ienum_snapshots,
            )
        };

        println!("resutl: {:#?}", hr_result);
        let mut result = Vec::new();

        // If there are no shadow copies, just return
        if hr_result == S_FALSE && snapshot_id == GUID::zeroed() {
            return Ok(result);
        }

        let p_ienum_snapshots = unsafe { IVssEnumObject::from_raw(p_ienum_snapshots) };
        let mut props: [VSS_OBJECT_PROP; 1] = [VSS_OBJECT_PROP::default(); 1];
        loop {
            let mut fetched = 0;
            // Get the next element
            unsafe {
                p_ienum_snapshots.Next(&mut props, &mut fetched).unwrap();
            }
            // We reached the end of list
            if fetched == 0 {
                break;
            }

            // Print the shadow copy (if not filtered out)
            if snapshot_id == GUID::zeroed()
                || unsafe { props[0].Obj.Snap.m_SnapshotId == snapshot_id }
            {
                let p = unsafe { props[0].Obj.Snap.clone() };
                let p = VSSProp::from_props(&p);
                result.push(p);
            }
        }

        Ok(result)
    }
}

pub fn fmt_vss_snapshot_prop(
    this: &VSS_SNAPSHOT_PROP,
    f: &mut ::core::fmt::Formatter<'_>,
) -> ::core::fmt::Result {
    f.debug_struct("VSS_SNAPSHOT_PROP")
        .field("* SNAPSHOT ID = ", &this.m_SnapshotId)
        .field("m_SnapshotSetId", &this.m_SnapshotSetId)
        .field("m_lSnapshotsCount", &this.m_lSnapshotsCount)
        .field(
            "m_pwszSnapshotDeviceObject",
            &this.m_pwszSnapshotDeviceObject,
        )
        .field("m_pwszOriginalVolumeName", &this.m_pwszOriginalVolumeName)
        .field("m_pwszOriginatingMachine", &this.m_pwszOriginatingMachine)
        .field("m_pwszServiceMachine", &this.m_pwszServiceMachine)
        .field("m_pwszExposedName", &this.m_pwszExposedName)
        .field("m_pwszExposedPath", &this.m_pwszExposedPath)
        .field("m_ProviderId", &this.m_ProviderId)
        .field("m_lSnapshotAttributes", &this.m_lSnapshotAttributes)
        .field("m_tsCreationTimestamp", &this.m_tsCreationTimestamp)
        .field("m_eStatus", &this.m_eStatus)
        .finish()
}

#[derive(Clone)]
pub struct VSSProp {
    pub snapshot_id: GUID,
    pub shadow_copy_set_id: GUID,
    pub snapshot_count: i32,
    pub origin_vol_name: String,
    pub create_time: DateTime<Utc>,
    pub device_name: String,
    pub origin_machine: String,
    pub origin_service: String,
    pub snapshot_attrs: VSS_VOLUME_SNAPSHOT_ATTRIBUTES,
    pub exposed_name: Option<String>,
    pub exposed_path: Option<String>,
    pub provider_id: GUID,
    pub state: VSS_SNAPSHOT_STATE,
}

impl Default for VSSProp {
    fn default() -> Self {
        Self {
            snapshot_id: Default::default(),
            shadow_copy_set_id: Default::default(),
            snapshot_count: Default::default(),
            origin_vol_name: Default::default(),
            create_time: Utc::now(),
            device_name: Default::default(),
            origin_machine: Default::default(),
            origin_service: Default::default(),
            snapshot_attrs: Default::default(),
            exposed_name: Default::default(),
            exposed_path: Default::default(),
            provider_id: Default::default(),
            state: Default::default(),
        }
    }
}

impl VSSProp {
    pub fn from_props(prop: &VSS_SNAPSHOT_PROP) -> Self {
        let mut ret = VSSProp::default();
        ret.snapshot_id = prop.m_SnapshotId;
        ret.shadow_copy_set_id = prop.m_SnapshotSetId;
        ret.snapshot_count = prop.m_lSnapshotsCount;
        ret.origin_vol_name = u16_to_string(prop.m_pwszOriginalVolumeName);
        //time
        ret.create_time = i64_to_date(prop.m_tsCreationTimestamp);

        ret.device_name = u16_to_string(prop.m_pwszSnapshotDeviceObject);
        ret.origin_machine = u16_to_string(prop.m_pwszOriginatingMachine);
        ret.origin_service = u16_to_string(prop.m_pwszServiceMachine);
        if !prop.m_pwszExposedName.is_null() {
            ret.exposed_name = Some(u16_to_string(prop.m_pwszExposedName));
        }
        if !prop.m_pwszExposedPath.is_null() {
            ret.exposed_path = Some(u16_to_string(prop.m_pwszExposedPath))
        }
        ret.snapshot_attrs = windows::Win32::Storage::Vss::VSS_VOLUME_SNAPSHOT_ATTRIBUTES(
            prop.m_lSnapshotAttributes,
        );

        ret.provider_id = prop.m_ProviderId;
        ret.state = prop.m_eStatus;

        return ret;
    }
}

impl ::core::fmt::Debug for VSSProp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSSProp")
            .field("SnapshotId", &self.snapshot_id)
            .field("SnapshotSetId", &self.shadow_copy_set_id)
            .field("SnapshotsCount", &self.snapshot_count)
            .field("SnapshotDeviceObject", &self.device_name)
            .field("OriginalVolumeName", &self.origin_vol_name)
            .field("OriginatingMachine", &self.origin_machine)
            .field("ServiceMachine", &self.origin_service)
            .field("ExposedName", &self.exposed_name)
            .field("ExposedPath", &self.exposed_path)
            .field("ProviderId", &self.provider_id)
            .field(
                "SnapshotAttributes",
                &volsnap_attrs_to_str(self.snapshot_attrs.0).join(" "),
            )
            .field(
                "CreationTimestamp",
                &self.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            )
            .field("Status", &get_string_for_snapshot_state(self.state))
            .finish()
    }
}

fn i64_to_date(t: i64) -> DateTime<Utc> {
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

fn u16_to_string(ptr: *const u16) -> String {
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

pub fn volsnap_attrs_to_str(attr: i32) -> Vec<String> {
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
