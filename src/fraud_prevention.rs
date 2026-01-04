//! Fraud prevention data logic.
//! See <https://developer.service.hmrc.gov.uk/guides/fraud-prevention/connection-method/desktop-app-direct>.
use chrono::{SecondsFormat, Utc};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

const DEVICE_ID: &str = "142454b1-6368-444b-97b6-15bfdf4e19a3";

pub trait FraudPreventionRequestBuilder {
    fn add_fraud_prevention_headers(self) -> Self;
}
impl FraudPreventionRequestBuilder for reqwest::RequestBuilder {
    fn add_fraud_prevention_headers(self) -> Self {
        self.header("Gov-Client-Connection-Method", "DESKTOP_APP_DIRECT")
            .header("Gov-Client-Device-ID", DEVICE_ID)
            .header(
                "Gov-Client-Local-IPs",
                local_ips().unwrap_or_else(|_| "null".into()),
            )
            .header(
                "Gov-Client-Local-IPs-Timestamp",
                Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            )
            .header(
                "Gov-Client-MAC-Addresses",
                mac_addresses().unwrap_or_else(|_| "null".into()),
            )
            .header(
                "Gov-Client-Timezone",
                format!("UTC{}", chrono::Local::now().offset()),
            )
            .header("Gov-Client-User-Agent", client_user_agent())
            .header("Gov-Client-User-IDs", client_user_ids())
            //.header("Gov-Client-Multi-Factor", "") // MFA n/a
            .header(
                "Gov-Client-Screens",
                "width=1920&height=1080&scaling-factor=1&colour-depth=16", // cli n/a
            )
            .header("Gov-Client-Window-Size", "width=1920&height=1080") // cli n/a
            .header(
                "Gov-Vendor-License-IDs",
                format!("mtd-vat-cli={}", blake3::hash(b"foss")),
            )
            .header("Gov-Vendor-Product-Name", "mtd-vat-cli")
            .header(
                "Gov-Vendor-Version",
                format!("mtd-vat-cli={}", env!("CARGO_PKG_VERSION")),
            )
    }
}

fn local_ips() -> anyhow::Result<String> {
    let local_ip = local_ip_address::local_ip()?;
    Ok(format!("{local_ip}"))
}

fn mac_addresses() -> anyhow::Result<String> {
    let addresses: String = mac_address::MacAddressIterator::new()?
        .map(|ma| utf8_percent_encode(&ma.to_string(), NON_ALPHANUMERIC).to_string())
        .collect::<Vec<_>>()
        .join(",");
    anyhow::ensure!(!addresses.is_empty());
    Ok(addresses)
}

fn client_user_agent() -> String {
    let os = os_info::get();
    let os_type = format!("{}", os.os_type());
    let os_family = utf8_percent_encode(&os_type, NON_ALPHANUMERIC);
    let os_v = format!("{}", os.version());
    let os_v = utf8_percent_encode(&os_v, NON_ALPHANUMERIC);
    let (manufacturer, model) = device_manufacturer_model();
    let manufacturer = utf8_percent_encode(&manufacturer, NON_ALPHANUMERIC);
    let model = utf8_percent_encode(&model, NON_ALPHANUMERIC);

    format!(
        "os-family={os_family}&os-version={os_v}&device-manufacturer={manufacturer}&device-model={model}",
    )
}

/// Return `(device-manufacturer, device-model)`.
#[allow(unused_mut)] // used on supported platforms
fn device_manufacturer_model() -> (String, String) {
    let mut manufacturer = "unknown".into();
    let mut model = "unknown".into();

    #[cfg(target_os = "linux")]
    if let Ok(mobo) = board_id::BoardId::detect() {
        if let Some(vendor) = mobo.vendor().and_then(|v| String::from_utf8(v.into()).ok()) {
            manufacturer = vendor;
        }
        if let Some(name) = mobo.name().and_then(|v| String::from_utf8(v.into()).ok()) {
            model = name;
        }
    }

    (manufacturer, model)
}

fn client_user_ids() -> String {
    let username = whoami::username().unwrap_or_else(|_| "null".into());
    let username = utf8_percent_encode(&username, NON_ALPHANUMERIC);
    format!("os={username}")
}
