use chrono::{DateTime, Local};
use std::{iter::once, ptr::null_mut, time::SystemTime};
use tracing::debug;
use windows::{
    core::{Interface, Type, BSTR, GUID, HRESULT},
    Win32::{
        Foundation::{FALSE, S_FALSE},
        Storage::Vss::{
            IVssAsync, IVssEnumObject, IVssExamineWriterMetadata, VSS_BT_FULL, VSS_CTX_BACKUP,
            VSS_OBJECT_NONE, VSS_OBJECT_PROP, VSS_OBJECT_SNAPSHOT, VSS_OBJECT_SNAPSHOT_SET,
            VSS_SNAPSHOT_CONTEXT, VSS_SNAPSHOT_PROP,
        },
        System::Com::{
            CoInitialize, CoInitializeSecurity, CoUninitialize, EOAC_NONE,
            RPC_C_AUTHN_LEVEL_PKT_PRIVACY, RPC_C_IMP_LEVEL_IDENTIFY,
        },
    },
};

use crate::{
    utils::get_unique_volume_name_for_path,
    vssbackupcomponent::{CreateVssBackupComponents, IVssBackupComponent},
    vssprop::VSSProp,
};

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

    /// Gather writers metadata
    pub fn gather_writer_metadata(&self) -> ::windows::core::Result<()> {
        tracing::info!("(Gathering writer metadata...)");
        let mut p_async =
            unsafe { IVssAsync::from_abi(::windows::core::zeroed::<IVssAsync>()).unwrap() };
        unsafe {
            // Gathers writer metadata
            // WARNING: this call can be performed only once per IVssBackupComponents instance!
            self.vss_object
                .clone()
                .as_ref()
                .unwrap()
                .GatherWriterMetadata(&mut p_async)?
        };
        self.wait_and_check_for_async_operation(&mut p_async)?;
        tracing::info!("Initialize writer metadata ...");
        // Initialize the internal metadata data structures
        Ok(())
    }

    /// Initialize writer metadata
    pub fn initialize_writer_metadata(&self) -> ::windows::core::Result<()> {
        let mut cnt_writer = 0;
        unsafe {
            self.vss_object
                .clone()
                .as_ref()
                .unwrap()
                .GetWriterMetadataCount(&mut cnt_writer)?
        };

        for i in 0..cnt_writer {
            // IVssExamineWriterMetadata
        }
        // Enumerate writers
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

    pub fn get_snapshot_properties(&self, snapshot_id: GUID) -> ::windows::core::Result<VSSProp> {
        let mut prop = VSS_SNAPSHOT_PROP::default();

        let res = unsafe {
            self.vss_object
                .as_ref()
                .unwrap()
                .GetSnapshotProperties(snapshot_id, &mut prop)
        };

        match res {
            Err(e) => return Err(e),
            Ok(_) => {
                return Ok(VSSProp::from_props(&prop));
            }
        }
    }

    /// Delete the given shadow copy
    pub fn delete_snapshot(&self, vss_id: GUID) -> ::windows::core::Result<()> {
        tracing::debug!("-Deleting shadow copy {:?}", vss_id);
        let mut l_snapshot = 0;
        let mut id_non_deleted_snapshot_id = GUID::default();
        let hr_result = unsafe {
            self.vss_object.as_ref().unwrap().DeleteSnapshots(
                vss_id,
                VSS_OBJECT_SNAPSHOT,
                FALSE,
                &mut l_snapshot,
                &mut id_non_deleted_snapshot_id,
            )
        };

        if hr_result.is_err() {
            tracing::debug!("Error while deleting shadow copies...");
            tracing::debug!(
                "-Last shadow copy that could not be deleted:{:?}",
                id_non_deleted_snapshot_id
            );
            return hr_result;
        }
        Ok(())
    }

    /// Delete all the shadow copies in the system
    pub fn delete_all_snapshots(&self) -> ::windows::core::Result<()> {
        let all_snapshosts = self.query_snapshot_set(GUID::default())?;

        if all_snapshosts.len() == 0 {
            tracing::debug!("There are no shadow copies on the system");
            return Ok(());
        }

        for i in all_snapshosts {
            match self.delete_snapshot(i.snapshot_id) {
                Ok(_) => continue,
                Err(e) => {
                    tracing::error!("Error while deleting shadow copies...");
                    tracing::error!(
                        "- Last shadow copy that could not be deleted: {:?}",
                        i.snapshot_id
                    );
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    /// Delete the given shadow copy set
    pub fn delete_snapshotset(&self, set_id: GUID) -> ::windows::core::Result<()> {
        tracing::debug!("- Deleting shadow copy set {:?}", set_id);
        let mut l_snapshot = 0;
        let mut id_non_deleted_snapshot_id = GUID::default();
        let hr_result = unsafe {
            self.vss_object.as_ref().unwrap().DeleteSnapshots(
                set_id,
                VSS_OBJECT_SNAPSHOT_SET,
                FALSE,
                &mut l_snapshot,
                &mut id_non_deleted_snapshot_id,
            )
        };

        if hr_result.is_err() {
            tracing::error!("Error while deleting shadow copies...");
            tracing::error!(
                "-Last shadow copy that could not be deleted:{:?}",
                id_non_deleted_snapshot_id
            );
            return hr_result;
        }
        Ok(())
    }

    pub fn delete_oldest_snapshot(&self, vol_name: &str) -> ::windows::core::Result<()> {
        let unique_volume = get_unique_volume_name_for_path(vol_name)?;

        let all_snapshosts = self.query_snapshot_set(GUID::default())?;

        if all_snapshosts.len() == 0 {
            tracing::debug!("There are no shadow copies on the system");
            return Ok(());
        }
        let mut oldest_id = GUID::default();
        let mut oldest_time: DateTime<chrono::Utc> = DateTime::<Local>::MIN_UTC;
        for i in all_snapshosts {
            if i.origin_vol_name == unique_volume && i.create_time < oldest_time {
                oldest_id = i.snapshot_id;
                oldest_time = i.create_time;
            }
        }

        if oldest_id != GUID::default() {
            self.delete_snapshot(oldest_id)
        } else {
            tracing::debug!("There are no specified shadow copies on the system");
            Ok(())
        }
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
