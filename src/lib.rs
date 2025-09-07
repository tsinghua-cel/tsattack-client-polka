pub mod attack {
    tonic::include_proto!("attack");
}

use attack::attack_service_client::AttackServiceClient;
use attack::{ReportValidatorInfoRequest, DelayForBlockRequest, ReportDutyRequest, ModifyBlockRequest};

pub struct AttackClient {
    client: AttackServiceClient<tonic::transport::Channel>,
}

impl AttackClient {
    pub async fn connect(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = AttackServiceClient::connect(endpoint.to_string()).await?;
        Ok(Self { client })
    }

    pub async fn report_validator_info(
        &mut self,
        request: ReportValidatorInfoRequest,
    ) -> Result<tonic::Response<attack::ReportValidatorInfoResponse>, tonic::Status> {
        self.client.report_validator_info(request).await
    }

    pub async fn delay_for_block(
        &mut self,
        request: DelayForBlockRequest,
    ) -> Result<tonic::Response<attack::DelayForBlockResponse>, tonic::Status> {
        self.client.delay_for_block(request).await
    }

    pub async fn report_duty(
        &mut self,
        request: ReportDutyRequest,
    ) -> Result<tonic::Response<attack::ReportDutyResponse>, tonic::Status> {
        self.client.report_duty(request).await
    }

    pub async fn modify_block(
        &mut self,
        request: ModifyBlockRequest,
    ) -> Result<tonic::Response<attack::ModifyBlockResponse>, tonic::Status> {
        self.client.modify_block(request).await
    }
}
