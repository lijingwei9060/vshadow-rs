use vshadow_rs::vssclient::VssClient;
use windows::{core::GUID, Win32::Storage::Vss::VSS_CTX_ALL};

fn main() {
    let mut client = VssClient::default();
    client.initialize(VSS_CTX_ALL, None, false).unwrap();
    let props = client.query_snapshot_set(GUID::zeroed()).unwrap();

    for i in 0..props.len() {
        println!("{i:05}--------------------------------------------------------------");
        println!("{:#?}", props[i]);
    }
}
