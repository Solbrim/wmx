// init interfaces, audio related?
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize};
use windows::Win32::System::Com::StructuredStorage::{STGM_READ, PROPVARIANT};
//  mute/unmute specific application
use windows::Win32::Media::Audio::{
    ISimpleAudioVolume,
    IAudioSessionManager2,
    eMultimedia, eRender,
    IMMDeviceCollection, IMMDeviceEnumerator, DEVICE_STATE_ACTIVE, IAudioSessionEnumerator, IAudioSessionControl2, IMMDevice, MMDeviceEnumerator
};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::core::{InParam, Interface, GUID};
use windows::Win32::System::Com::{CLSCTX_ALL};
use core::ffi::c_void;
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;
use windows::Win32::Devices::FunctionDiscovery::{ PKEY_DeviceInterface_FriendlyName };
use crate::misc_defs::*;

pub fn device_name (device: &IMMDevice) -> anyhow::Result<String> {
    let device_store = unwrap_or_stfu!(unsafe { device.OpenPropertyStore(STGM_READ) }, "device_store");
    let device_store_value: *const PROPERTYKEY = &PKEY_DeviceInterface_FriendlyName;
    let device_name: PROPVARIANT = unwrap_or_stfu!(unsafe { device_store.GetValue(device_store_value) }, "device_name");
    let device_name_to_str = unwrap_or_stfu! (unsafe { device_name.Anonymous.Anonymous.Anonymous.pwszVal.to_string() }, "device_name_str");
    Ok(device_name_to_str)
}

pub fn devices () -> anyhow::Result<IMMDeviceEnumerator> {
    let null_ptr: *const c_void = std::ptr::null();

    let imm_device_enum_init = unsafe {
        CoInitialize (null_ptr)
    };

    let imm_device_enum: IMMDeviceEnumerator = unwrap_or_stfu!(unsafe { CoCreateInstance(
        &MMDeviceEnumerator,
        InParam::null(),
        CLSCTX_ALL,
    ) }, "imm_device_enum_result");

    Ok(imm_device_enum)
}

pub struct SessionIterator {
    session_enum: IAudioSessionEnumerator,
    pub size: usize,
    current_idx: usize,
}

impl SessionIterator {
    pub fn new (device: &IMMDevice) -> SessionIterator {
        let sess_enum = sessions(&device).unwrap();
        let sess_count = unsafe { sess_enum.GetCount() }.unwrap();

        SessionIterator {
            session_enum: sess_enum,
            size: sess_count as usize,
            current_idx: 0,
        }
    }
}

impl Iterator for SessionIterator {
    type Item = IAudioSessionControl2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx < self.size {
            let sess = unsafe { self.session_enum.GetSession(self.current_idx as i32) }.unwrap();
            let sess2: IAudioSessionControl2 = sess.cast().unwrap();
            self.current_idx += 1;

            return Some(sess2);
        }
        return None;
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        return (0, Some(self.size));
    }
}

pub fn default_device () -> anyhow::Result<IMMDevice> {
    let devices = unwrap_or_stfu!(devices(), "devices");
    let device = unwrap_or_stfu!(unsafe {devices.GetDefaultAudioEndpoint(eRender, eMultimedia)}, "default_device");
    return Ok(device);
}

pub fn devices_to_vec (devices_enum: &IMMDeviceEnumerator) -> anyhow::Result<Vec<IMMDevice>> {
    let devices: IMMDeviceCollection = unwrap_or_stfu!(unsafe {
        devices_enum.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
    }, "device_collection");
    let device_count = unwrap_or_stfu!(unsafe { devices.GetCount() }, "device_count");
    let mut res: Vec<IMMDevice> = vec![];
    for n in 0..device_count {
        let device = unwrap_or_stfu!(unsafe {devices.Item(n)}, "device");
        res.push(device);
    }

    return Ok(res);
}

pub fn session_iden (sess2: &IAudioSessionControl2) -> anyhow::Result<String> {
    let sess_iden = unwrap_or_stfu! (unsafe { sess2.GetSessionIdentifier() }, "sess_iden");
    let sess_iden_str = unwrap_or_stfu!(unsafe { sess_iden.to_string() }, "sess_iden_str");
    return Ok(sess_iden_str);
}

pub fn session_display (sess2: &IAudioSessionControl2) -> anyhow::Result<String> {
    let sess_iden = unwrap_or_stfu! (unsafe { sess2.GetDisplayName() }, "sess_iden");
    let sess_iden_str = unwrap_or_stfu!(unsafe { sess_iden.to_string() }, "sess_iden_str");
    return Ok(sess_iden_str);
}

pub fn session_group (sess2: &IAudioSessionControl2) -> anyhow::Result<GUID> {
    let sess_group = unwrap_or_stfu!(unsafe { sess2.GetGroupingParam() }, "sess_grouping");
    return Ok(sess_group);
}

pub fn session_matches (input: &String, sess2: &IAudioSessionControl2) -> anyhow::Result<bool> {
    let iden = unwrap_or_stfu!(session_iden(sess2), "iden");
    return Ok(iden.to_lowercase().contains(input.to_lowercase().as_str()));
}

pub fn sessions (device: &IMMDevice) -> anyhow::Result<IAudioSessionEnumerator> {
    let prop_var_null: *const PROPVARIANT = core::ptr::null();
    let audio_session_mgr_2: IAudioSessionManager2 = unwrap_or_stfu!(unsafe { device.Activate(CLSCTX_ALL, prop_var_null) }, "audio_session_mgr_result");
    let sess_enum = unwrap_or_stfu!(unsafe { audio_session_mgr_2.GetSessionEnumerator() }, "sess_enum_result");
    Ok(sess_enum)
}

pub fn mute_session (sess2: &IAudioSessionControl2) {
    let simp: ISimpleAudioVolume = sess2.cast().unwrap();
    let sess_grouping = unwrap_or_return!(session_group(&sess2), "sess_grouping");
    let sess_grouping_ptr: *const GUID = &sess_grouping as *const GUID;

    unwrap_or_return!(unsafe {simp.SetMute(true, sess_grouping_ptr)}, "mute");
}

pub fn unmute_session (sess2: &IAudioSessionControl2) {
    let simp: ISimpleAudioVolume = sess2.cast().unwrap();
    let sess_grouping = unwrap_or_return!(session_group(&sess2), "sess_grouping");
    let sess_grouping_ptr: *const GUID = &sess_grouping as *const GUID;
    

    unwrap_or_return!(unsafe {simp.SetMute(false, sess_grouping_ptr)}, "mute");
}

pub fn set_device_volume(device: &IMMDevice, volume: f32) -> anyhow::Result<()> {
    if volume > 1.0 || volume < 0.0 {
        return Err(StringError::anyhow(format!("Volume must be between 0.0 and 1.0; received {}", volume)));
    }
    let prop_var_null: *const PROPVARIANT = core::ptr::null();
    let endpoint_vol: IAudioEndpointVolume = unwrap_or_stfu!(unsafe {device.Activate(CLSCTX_ALL, prop_var_null) }, "IAudioEndpointVolume");
    let null_guid: *const GUID = core::ptr::null();
    unwrap_win!(unsafe {endpoint_vol.SetMasterVolumeLevelScalar(volume, null_guid)}, "set_master_volume_level");

    Ok(())
}

pub fn get_device_volume(device: &IMMDevice) -> anyhow::Result<f32> {

    let prop_var_null: *const PROPVARIANT = core::ptr::null();
    let endpoint_vol: IAudioEndpointVolume = unwrap_or_stfu!(unsafe {device.Activate(CLSCTX_ALL, prop_var_null) }, "IAudioEndpointVolume");
    let volume = unwrap_or_stfu!(unsafe {endpoint_vol.GetMasterVolumeLevelScalar()}, "volume");

    Ok(volume)
}

/// sets the volume for the session, e.g. firefox
/// see https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nn-endpointvolume-iaudioendpointvolume for setting a device volume
pub fn set_session_volume(sess2: &IAudioSessionControl2, volume: f32) -> anyhow::Result<()> {
    if volume > 1.0 || volume < 0.0 {
        return Err(StringError::anyhow(format!("Volume must be between 0.0 and 1.0; received {}", volume)));
    }

    let simp: ISimpleAudioVolume = sess2.cast().unwrap();
    let sess_grouping = unwrap_or_stfu!(session_group(&sess2), "sess_grouping");
    let sess_grouping_ptr: *const GUID = &sess_grouping as *const GUID;

    unwrap_or_stfu!(unsafe {simp.SetMasterVolume(volume, sess_grouping_ptr)}, "set_master_volume");

    Ok(())
}

pub fn get_session_volume(sess2: &IAudioSessionControl2) -> anyhow::Result<f32> {
    let simp: ISimpleAudioVolume = sess2.cast().unwrap();

    let vol = unwrap_or_stfu!(unsafe {simp.GetMasterVolume()}, "get_master_volume");
    Ok(vol)
}
