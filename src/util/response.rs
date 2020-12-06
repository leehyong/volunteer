use crate::import::*;

#[derive(Serialize)]
pub struct ResponseUtil<T: Send + Sync + Serialize + 'static> {
    code: u8,
    msg: String,
    data: T,
}


impl<T: Send + Sync + 'static + Serialize> ResponseUtil<T> {
    pub fn new(status_code: StatusCode, data: T, msg: String) -> TideResult {
        let mut rsp = Response::new(status_code);
        rsp.set_content_type(Mime::from_extension("json").unwrap());
        let code = {
            if status_code.is_informational()
                || status_code.is_redirection()
                || status_code.is_success() {
                0
            } else {
                1
            }
        };
        let rdata = Self {
            code,
            data,
            msg,
        };
        rsp.set_body(json!(rdata).to_string());
        return Ok(rsp);
    }

    pub fn ok(data: T) -> TideResult {
        Self::new(StatusCode::Ok, data, "success".to_owned())
    }

    pub fn ok_with_msg(data: T, msg: String) -> TideResult {
        Self::new(StatusCode::Ok, data, msg)
    }
    pub fn error(data: T) -> TideResult {
        Self::new(StatusCode::BadRequest, data, "error".to_owned())
    }

    pub fn error_with_msg(data: T, msg: String) -> TideResult {
        Self::new(StatusCode::BadRequest, data, msg)
    }
}

