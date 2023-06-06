use chrono::{DateTime, Utc};

use windows::{
    core::GUID,
    Win32::Storage::Vss::{VSS_SNAPSHOT_PROP, VSS_SNAPSHOT_STATE, VSS_VOLUME_SNAPSHOT_ATTRIBUTES},
};

use crate::utils::{
    get_string_for_snapshot_state, i64_to_date, u16_to_string, volsnap_attrs_to_str,
};

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
