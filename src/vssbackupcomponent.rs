use windows::{
    core::{BSTR, HRESULT},
    Win32::{
        Foundation::BOOL,
        Storage::Vss::{
            IVssAsync, VSS_BACKUP_TYPE, VSS_COMPONENT_TYPE, VSS_FILE_RESTORE_STATUS,
            VSS_OBJECT_TYPE, VSS_RESTORE_TYPE, VSS_SNAPSHOT_CONTEXT, VSS_SNAPSHOT_PROP,
            VSS_WRITER_STATE,
        },
    },
};

#[repr(transparent)]
pub struct IVssBackupComponent(::windows::core::IUnknown);

impl ::windows::core::RuntimeName for IVssBackupComponent {}

impl IVssBackupComponent {
    pub unsafe fn AbortBackup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AbortBackup)(::windows::core::Interface::as_raw(
            self,
        ))
        .ok()
    }

    pub unsafe fn AddAlternativeLocationMapping<P0>(
        &self,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: P0,
        wszComponentName: P0,
        wszPath: P0,
        wszFilespec: P0,
        bRecursive: bool,
        wszDestination: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddAlternativeLocationMapping)(
            ::windows::core::Interface::as_raw(self),
            writerId,
            ct,
            wszLogicalPath.into_param().abi(),
            wszComponentName.into_param().abi(),
            wszPath.into_param().abi(),
            wszFilespec.into_param().abi(),
            bRecursive,
            wszDestination.into_param().abi(),
        )
        .ok()
    }
    // pub AddComponent: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     instanceId: ::windows::core::GUID,
    //     writerId: ::windows::core::GUID,
    //     ct: VSS_COMPONENT_TYPE,
    //     wszLogicalPath: ::windows::core::PCWSTR,
    //     wszComponentName: ::windows::core::PCWSTR,
    // ) -> ::windows::core::HRESULT,

    // pub AddNewTarget: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     writerId: ::windows::core::GUID,
    //     ct: VSS_COMPONENT_TYPE,
    //     wszLogicalPath: ::windows::core::PCWSTR,
    //     wszComponentName: ::windows::core::PCWSTR,
    //     wszPath: ::windows::core::PCWSTR,
    //     wszFileName: ::windows::core::PCWSTR,
    //     bRecursive: bool,
    //     wszAlternatePath: ::windows::core::PCWSTR,
    // ) -> ::windows::core::HRESULT,

    // pub AddRestoreSubcomponent: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     writerId: ::windows::core::GUID,
    //     ct: VSS_COMPONENT_TYPE,
    //     wszLogicalPath: ::windows::core::PCWSTR,
    //     wszComponentName: ::windows::core::PCWSTR,
    //     wszSubComponentLogicalPath: ::windows::core::PCWSTR,
    //     wszSubComponentName: ::windows::core::PCWSTR,
    //     bRepair: bool,
    // ) -> ::windows::core::HRESULT,

    /// The AddToSnapshotSet method adds an original volume or original remote file share to the shadow copy set.
    ///
    /// - \[in\] pwszVolumeName
    /// - \[in\] ProviderId: The provider to be used. GUID_NULL can be used, in which case the default provider will be used.
    /// - \[out\] pidSnapshot: Returned identifier of the added shadow copy.
    pub unsafe fn AddToSnapshotSet(
        &self,
        pwszVolumeName: ::windows::core::PCWSTR,
        ProviderId: ::windows::core::GUID,
        pidSnapshot: *mut ::windows::core::GUID,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddToSnapshotSet)(
            ::windows::core::Interface::as_raw(self),
            pwszVolumeName,
            ProviderId,
            pidSnapshot,
        )
        .ok()
    }

    // pub BackupComplete: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     ppAsync: *mut *mut IVssAsync,
    // ) -> ::windows::core::HRESULT,

    // pub BreakSnapshotSet: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     SnapshotSetId: ::windows::core::PCWSTR,
    // ) -> ::windows::core::HRESULT,

    // pub DeleteSnapshots: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     SourceObjectId: ::windows::core::GUID,
    //     eSourceObjectType: VSS_OBJECT_TYPE,
    //     bForceDelete: BOOL,
    //     plDeletedSnapshots: *mut i32,
    //     pNondeletedSnapshotID: *mut ::windows::core::GUID,
    // ) -> ::windows::core::HRESULT,

    // pub DisableWriterClasses: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     rgWriterClassId: *const ::windows::core::GUID,
    //     cClassId: u32,
    // ) -> ::windows::core::HRESULT,

    // /// [in] rgWriterInstanceId: An array containing one or more writer instance identifiers.
    // /// [in] cInstanceId: The number of entries in the rgWriterInstanceId array.
    // pub DisableWriterInstances: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     rgWriterInstanceId: *const ::windows::core::GUID,
    //     cInstanceId: u32,
    // ) -> ::windows::core::HRESULT,

    /// Commits all shadow copies in this set simultaneously.
    pub unsafe fn DoSnapshotSet(
        &self,
        ppAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoSnapshotSet)(
            ::windows::core::Interface::as_raw(self),
            ppAsync,
        )
        .ok()
    }

    // /// The EnableWriterClasses method enables the specified writers to receive all events.
    // ///
    // /// - \[in\] rgWriterClassId: An array containing one or more writer class identifiers.
    // /// - \[in\] cClassId: The number of entries in the rgWriterClassId array.
    // pub EnableWriterClasses: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     rgWriterClassId: *const ::windows::core::GUID,
    //     cClassId: u32,
    // ) -> ::windows::core::HRESULT,

    // /// The ExposeSnapshot method exposes a shadow copy as a drive letter, mounted folder, or file share.
    // ///
    // /// - \[in\] SnapshotId: Shadow copy identifier.
    // /// - \[in\] wszPathFromRoot
    // /// - \[in\] lAttributes
    // /// - \[in\] wszExpose
    // /// - \[out\] pwszExposed
    // pub ExposeSnapshot: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     SnapshotId: ::windows::core::GUID,
    //     wszPathFromRoot: ::windows::core::PCWSTR,
    //     lAttributes: i32,
    //     wszExpose: ::windows::core::PCWSTR,
    //     pwszExposed: ::windows::core::PCWSTR,
    // ) -> ::windows::core::HRESULT,

    // /// The FreeWriterMetadata method frees system resources allocated
    // /// when IVssBackupComponents::GatherWriterMetadata was called.
    // pub FreeWriterMetadata:
    //     unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,

    // /// The FreeWriterStatus method frees system resources
    // /// allocated during the call to IVssBackupComponents::GatherWriterStatus.
    // pub FreeWriterStatus:
    //     unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,

    pub unsafe fn GatherWriterMetadata(
        &self,
        pAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GatherWriterMetadata)(
            ::windows::core::Interface::as_raw(self),
            pAsync,
        )
        .ok()
    }

    // pub GatherWriterStatus: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     ppAsync: *mut *mut IVssAsync,
    // ) -> ::windows::core::HRESULT,

    // pub GetSnapshotProperties: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     SnapshotId: ::windows::core::GUID,
    //     pProp: *mut VSS_SNAPSHOT_PROP,
    // ) -> ::windows::core::HRESULT,

    // pub GetWriterComponentsCount: unsafe extern "system" fn(
    //     this: *mut ::core::ffi::c_void,
    //     pcComponents: *mut u32,
    // ) -> ::windows::core::HRESULT,

    /// The InitializeForBackup method initializes the backup components metadata in preparation for backup.
    ///
    /// - \[in\] bstrXML: Optional. During imports of transported shadow copies,
    /// this parameter must be the original document generated when creating the
    /// saved shadow copy and saved using IVssBackupComponents::SaveAsXML.
    pub unsafe fn InitializeForBackup(
        &self,
        bstrXML: ::windows::core::BSTR,
    ) -> ::windows::core::Result<()> {
        let p = ::windows::core::Interface::as_raw(self);
        let result = (::windows::core::Interface::vtable(self).InitializeForBackup)(p, bstrXML);

        println!("result in InitializeForBackup: {:018p} {}", p, result.0);
        result.ok()
    }

    pub unsafe fn SetContext(&self, lContext: VSS_SNAPSHOT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContext)(
            ::windows::core::Interface::as_raw(self),
            lContext,
        )
        .ok()
    }

    pub unsafe fn StartSnapshotSet(
        &self,
        pSnapshotSetId: *mut ::windows::core::GUID,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartSnapshotSet)(
            ::windows::core::Interface::as_raw(self),
            pSnapshotSetId,
        )
        .ok()
    }

    pub unsafe fn SetBackupState(
        &self,
        bSelectComponents: bool,
        bBackupBootableSystemState: bool,
        backupType: VSS_BACKUP_TYPE,
        bPartialFileSupport: bool,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackupState)(
            ::windows::core::Interface::as_raw(self),
            bSelectComponents,
            bBackupBootableSystemState,
            backupType,
            bPartialFileSupport,
        )
        .ok()
    }

    pub unsafe fn PrepareForBackup(
        &self,
        ppAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PrepareForBackup)(
            ::windows::core::Interface::as_raw(self),
            ppAsync,
        )
        .ok()
    }

    pub unsafe fn GetSnapshotProperties(
        &self,
        SnapshotId: ::windows::core::GUID,
        pProp: *mut VSS_SNAPSHOT_PROP,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSnapshotProperties)(
            ::windows::core::Interface::as_raw(self),
            SnapshotId,
            pProp,
        )
        .ok()
    }

    pub unsafe fn InitializeForRestore(
        &self,
        bstrXML: ::windows::core::BSTR,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeForRestore)(
            ::windows::core::Interface::as_raw(self),
            bstrXML,
        )
        .ok()
    }

    pub unsafe fn Query(
        &self,
        QueriedObjectId: ::windows::core::GUID,
        eQueriedObjectType: VSS_OBJECT_TYPE,
        eReturnedObjectsType: VSS_OBJECT_TYPE,
        ppEnum: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Query)(
            ::windows::core::Interface::as_raw(self),
            QueriedObjectId,
            eQueriedObjectType,
            eReturnedObjectsType,
            ppEnum,
        )
    }
}

#[link(name = "vssapi")]
extern "system" {
    fn CreateVssBackupComponentsInternal(
        ppwriter: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT;
}

#[inline]
pub unsafe fn CreateVssBackupComponents() -> ::windows::core::Result<IVssBackupComponent> {
    // ::windows_targets::link!("vssapi.dll" "system" fn CreateVssBackupComponentsInternal(ppwriter : *mut * mut::core::ffi::c_void) -> ::windows::core::HRESULT);
    let mut result__ = ::windows::core::zeroed::<IVssBackupComponent>();
    // println!("&mut result: {:018p}", &mut result__);
    let result = CreateVssBackupComponentsInternal(&mut result__);
    // println!("&mut result: {:018p}", &mut result__);
    // println!("&result: {:018p}", &result__);
    // println!("result: {:018p}", result__);
    // println!("result value: {}", *result);
    result.from_abi(result__)
}

#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,

    pub GetWriterComponentsCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pcComponents: *mut u32,
    ) -> ::windows::core::HRESULT,

    pub GetWriterComponents: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iWriter: u32,
        ppWriter: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,

    pub InitializeForBackup: fn(
        this: *mut ::core::ffi::c_void,
        bstrXML: ::windows::core::BSTR,
    ) -> ::windows::core::HRESULT,

    pub SetBackupState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bSelectComponents: bool,
        bBackupBootableSystemState: bool,
        backupType: VSS_BACKUP_TYPE,
        bPartialFileSupport: bool,
    ) -> ::windows::core::HRESULT,

    pub InitializeForRestore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bstrXML: ::windows::core::BSTR,
    ) -> ::windows::core::HRESULT,

    pub SetRestoreState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        restoreType: VSS_RESTORE_TYPE,
    ) -> ::windows::core::HRESULT,

    pub GatherWriterMetadata: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,

    pub GetWriterMetadataCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pcWriters: *mut u32,
    ) -> ::windows::core::HRESULT,

    pub GetWriterMetadata: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iWriter: u32,
        pidInstance: *mut ::windows::core::GUID,
        ppMetadata: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,

    /// The FreeWriterMetadata method frees system resources allocated
    /// when IVssBackupComponents::GatherWriterMetadata was called.
    pub FreeWriterMetadata:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,

    pub AddComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instanceId: ::windows::core::GUID,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,

    pub PrepareForBackup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        ppAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,

    pub AbortBackup:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,

    pub GatherWriterStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        ppAsync: *mut *mut IVssAsync,
    ) -> ::windows::core::HRESULT,

    pub GetWriterStatusCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pcWriters: *mut u32,
    ) -> ::windows::core::HRESULT,

    /// The FreeWriterStatus method frees system resources
    /// allocated during the call to IVssBackupComponents::GatherWriterStatus.
    pub FreeWriterStatus:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,

    pub GetWriterStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iWriter: u32,
        pidInstance: *mut ::windows::core::GUID,
        pidWriter: *mut ::windows::core::GUID,
        pbstrWriter: *mut BSTR,
        pnStatus: *mut VSS_WRITER_STATE,
        phResultFailure: *mut HRESULT,
    ) -> ::windows::core::HRESULT,

    pub SetBackupSucceeded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instanceId: ::windows::core::GUID,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        bSucceded: bool,
    ) -> ::windows::core::HRESULT,

    pub SetBackupOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        wszBackupOptions: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,

    pub SetSelectedForRestore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        bSelectedForRestore: bool,
    ) -> ::windows::core::HRESULT,

    pub SetRestoreOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        wszRestoreOptions: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,

    pub SetAdditionalRestores: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        bAdditionalRestores: bool,
    ) -> ::windows::core::HRESULT,

    pub SetPreviousBackupStamp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        wszPreviousBackupStamp: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,

    pub SaveAsXML: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pbstrXML: *mut BSTR,
    ) -> ::windows::core::HRESULT,

    pub BackupComplete: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        ppAsync: *mut *mut IVssAsync,
    ) -> ::windows::core::HRESULT,

    pub AddAlternativeLocationMapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        wszPath: ::windows::core::PCWSTR,
        wszFilespec: ::windows::core::PCWSTR,
        bRecursive: bool,
        wszDestination: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,

    pub AddRestoreSubcomponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        wszSubComponentLogicalPath: ::windows::core::PCWSTR,
        wszSubComponentName: ::windows::core::PCWSTR,
        bRepair: bool,
    ) -> ::windows::core::HRESULT,

    pub SetFileRestoreStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        status: VSS_FILE_RESTORE_STATUS,
    ) -> ::windows::core::HRESULT,

    pub AddNewTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        wszPath: ::windows::core::PCWSTR,
        wszFileName: ::windows::core::PCWSTR,
        bRecursive: bool,
        wszAlternatePath: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,

    pub SetRangesFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writerId: ::windows::core::GUID,
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: ::windows::core::PCWSTR,
        wszComponentName: ::windows::core::PCWSTR,
        iPartialFile: u32,
        wszRangesFile: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,

    pub PreRestore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        ppAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub PostRestore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        ppAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,

    pub SetContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lContext: VSS_SNAPSHOT_CONTEXT,
    ) -> ::windows::core::HRESULT,

    pub StartSnapshotSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pSnapshotSetId: *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,

    pub AddToSnapshotSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pwszVolumeName: ::windows::core::PCWSTR,
        ProviderId: ::windows::core::GUID,
        pidSnapshot: *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,

    /// Commits all shadow copies in this set simultaneously.
    pub DoSnapshotSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        ppAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,

    pub DeleteSnapshots: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        SourceObjectId: ::windows::core::GUID,
        eSourceObjectType: VSS_OBJECT_TYPE,
        bForceDelete: BOOL,
        plDeletedSnapshots: *mut i32,
        pNondeletedSnapshotID: *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,

    pub ImportSnapshots: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        ppAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,

    pub BreakSnapshotSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        SnapshotSetId: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,

    pub GetSnapshotProperties: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        SnapshotId: ::windows::core::GUID,
        pProp: *mut VSS_SNAPSHOT_PROP,
    ) -> ::windows::core::HRESULT,

    pub Query: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        QueriedObjectId: ::windows::core::GUID,
        eQueriedObjectType: VSS_OBJECT_TYPE,
        eReturnedObjectsType: VSS_OBJECT_TYPE,
        ppEnum: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,

    pub IsVolumeSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        ProviderId: ::windows::core::GUID,
        pwszVolumeName: ::windows::core::PCWSTR,
        pbSupportedByThisProvider: *mut BOOL,
    ) -> ::windows::core::HRESULT,

    pub DisableWriterClasses: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rgWriterClassId: *const ::windows::core::GUID,
        cClassId: u32,
    ) -> ::windows::core::HRESULT,

    /// The EnableWriterClasses method enables the specified writers to receive all events.
    ///
    /// - \[in\] rgWriterClassId: An array containing one or more writer class identifiers.
    /// - \[in\] cClassId: The number of entries in the rgWriterClassId array.
    pub EnableWriterClasses: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rgWriterClassId: *const ::windows::core::GUID,
        cClassId: u32,
    ) -> ::windows::core::HRESULT,

    /// [in] rgWriterInstanceId: An array containing one or more writer instance identifiers.
    /// [in] cInstanceId: The number of entries in the rgWriterInstanceId array.
    pub DisableWriterInstances: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rgWriterInstanceId: *const ::windows::core::GUID,
        cInstanceId: u32,
    ) -> ::windows::core::HRESULT,

    /// The ExposeSnapshot method exposes a shadow copy as a drive letter, mounted folder, or file share.
    ///
    /// - \[in\] SnapshotId: Shadow copy identifier.
    /// - \[in\] wszPathFromRoot
    /// - \[in\] lAttributes
    /// - \[in\] wszExpose
    /// - \[out\] pwszExposed
    pub ExposeSnapshot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        SnapshotId: ::windows::core::GUID,
        wszPathFromRoot: ::windows::core::PCWSTR,
        lAttributes: i32,
        wszExpose: ::windows::core::PCWSTR,
        pwszExposed: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,

    pub RevertToSnapshot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        SnapshotId: ::windows::core::GUID,
        bForceDismount: BOOL,
    ) -> ::windows::core::HRESULT,

    pub QueryRevertStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pwszVolume: ::windows::core::PCWSTR,
        ppAsync: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}

impl ::core::cmp::PartialEq for IVssBackupComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssBackupComponent {}
impl ::core::fmt::Debug for IVssBackupComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssBackupComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssBackupComponent {
    type Vtable = IVssBackupComponent_Vtbl;
}
impl ::core::clone::Clone for IVssBackupComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVssBackupComponent {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x665c1d5f_c218_414d_a05d_7fef5f9d5c86);
}

::windows::imp::interface_hierarchy!(IVssBackupComponent, ::windows::core::IUnknown);

#[cfg(test)]
mod test {
    use std::iter::once;

    use windows::{
        core::{Type, BSTR, PCWSTR},
        Win32::Storage::Vss::{
            VSS_BT_COPY, VSS_CTX_APP_ROLLBACK, VSS_CTX_BACKUP, VSS_CTX_CLIENT_ACCESSIBLE_WRITERS,
        },
    };

    use super::*;

    #[test]
    fn test_vss() {
        let volume = r"\\.\c:"
            .encode_utf16()
            .chain(once(0))
            .collect::<Vec<u16>>();
        unsafe {
            let vssBackup = CreateVssBackupComponents().unwrap();
            vssBackup.InitializeForBackup(BSTR::new()).unwrap();
            vssBackup
                .SetContext(VSS_SNAPSHOT_CONTEXT(
                    VSS_CTX_BACKUP.0 | VSS_CTX_CLIENT_ACCESSIBLE_WRITERS.0 | VSS_CTX_APP_ROLLBACK.0,
                ))
                .unwrap();

            let mut pAsync = ::windows::core::zeroed::<IVssAsync>();
            vssBackup.GatherWriterMetadata(&mut pAsync).unwrap();

            let pAynsc = IVssAsync::from_abi(pAsync).unwrap();

            println!("Gathering metadata from writers...");

            pAynsc.Wait(u32::MAX).unwrap();

            println!("calling StartSnapshotSet...");
            let mut id = ::windows::core::GUID::zeroed();
            vssBackup.StartSnapshotSet(&mut id).unwrap();

            vssBackup
                .AddToSnapshotSet(
                    PCWSTR::from_raw(volume.as_ptr()),
                    ::windows::core::GUID::zeroed(),
                    &mut id,
                )
                .unwrap();

            vssBackup
                .SetBackupState(false, false, VSS_BT_COPY, false)
                .unwrap();

            println!("Preparing for backup...");
            let mut pPrepare = ::windows::core::zeroed::<IVssAsync>();
            vssBackup.PrepareForBackup(&mut pPrepare).unwrap();
            let pPrepare = IVssAsync::from_abi(pPrepare).unwrap();
            pPrepare.Wait(u32::MAX).unwrap();

            println!("Commit all snapshots in this set...");
            let mut pDoShadowCopy = ::windows::core::zeroed::<IVssAsync>();
            vssBackup.DoSnapshotSet(&mut pDoShadowCopy).unwrap();
            let pDoShadowCopy = IVssAsync::from_abi(pDoShadowCopy).unwrap();
            pDoShadowCopy.Wait(u32::MAX).unwrap();

            println!("guid:{:?}", id);
        }
    }
}
