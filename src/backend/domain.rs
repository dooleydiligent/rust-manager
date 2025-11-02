use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DomainInfo {
    pub name: String,
    pub full_name: String,
    pub uuid: String,
    pub state: String,
    pub time: u64,
    pub memory: u64,
    pub max_mem: u64,
}

#[cfg(feature = "server")]
pub mod server {
    use super::DomainInfo;
    use axum::{http::StatusCode, Json};
    use virt::connect::Connect;

    // ---------------------------------------------------------------------
    // GET /api/domains – query libvirt for all domains
    // ---------------------------------------------------------------------
    pub async fn get_domains() -> Result<Json<Vec<DomainInfo>>, (StatusCode, String)> {
        // 1. Open a connection to the local hypervisor (qemu)
        let conn = Connect::open(Some("qemu:///system"))
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        // 2. List all domains (including inactive ones)
        let domains = conn.list_all_domains(0).unwrap();

        // 3. Build a serialisable vector
        let mut out = Vec::new();
        for dom in domains {
            let name = dom.get_name().unwrap();
            let uuid = dom.get_uuid_string().unwrap();

            let info = dom.get_info().unwrap();
            let state = match info.state {
                0 => &"No State",
                1 => &"Running",
                2 => &"Blocked",
                3 => &"Paused",
                4 => &"Shutdown",
                5 => &"Shutoff",
                6 => &"Crashed",
                7 => &"Suspended",
                _ => &"Unknown",
            };

            out.push(DomainInfo {
                name,
                full_name: dom.get_hostname(0).unwrap_or_default(),
                uuid,
                state: state.to_string(),
                time: info.cpu_time,
                memory: info.memory,
                max_mem: info.max_mem,
            });
        }

        Ok(Json(out))
    }
}
