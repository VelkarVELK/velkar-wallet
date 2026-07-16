use velkar_grpc_core::protowire::{VelkardRequest, VelkardResponse, velkard_request, velkard_response};

pub(crate) trait Matcher<T> {
    fn is_matching(&self, response: T) -> bool;
}

impl Matcher<&velkard_response::Payload> for velkard_request::Payload {
    fn is_matching(&self, response: &velkard_response::Payload) -> bool {
        use velkard_request::Payload;
        match self {
            // TODO: implement for each payload variant supporting request/response pairing
            Payload::GetBlockRequest(request) => {
                if let velkard_response::Payload::GetBlockResponse(response) = response {
                    if let Some(block) = response.block.as_ref() {
                        if let Some(verbose_data) = block.verbose_data.as_ref() {
                            return verbose_data.hash == request.hash;
                        }
                        return true;
                    } else if let Some(error) = response.error.as_ref() {
                        // the response error message should contain the requested hash
                        return error.message.contains(request.hash.as_str());
                    }
                }
                false
            }

            _ => true,
        }
    }
}

impl Matcher<&VelkardResponse> for VelkardRequest {
    fn is_matching(&self, response: &VelkardResponse) -> bool {
        if let Some(ref response) = response.payload
            && let Some(ref request) = self.payload
        {
            return request.is_matching(response);
        }
        false
    }
}
