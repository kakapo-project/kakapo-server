use state::error::DomainManagementError;
use data::DomainInfo;

pub trait DomainManagementOps {
    fn get_all_domains(&self) -> Result<Vec<DomainInfo>, DomainManagementError>;
}