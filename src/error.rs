#[derive(Debug)]
pub enum IpseClientError {
    NoneFile,
    IpfsResp(ipfs_api::response::Error),
    Substrate(substrate_subxt::Error),
    HttpsError(reqwest::Error),
}

impl From<ipfs_api::response::Error> for IpseError {
    fn from(err: ipfs_api::response::Error) -> Self {
        IpseClientError::IpfsResp(err)
    }
}

impl From<substrate_subxt::Error> for IpseError {
    fn from(err: substrate_subxt::Error) -> Self {
        IpseClientError::Substrate(err)
    }
}

impl From<reqwest::Error> for IpseClientError {
    fn from(err: reqwest::Error) -> Self {
        IpseClientError::HttpsError(err)
    }
}
