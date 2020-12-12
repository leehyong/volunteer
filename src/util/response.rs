use crate::import::*;
use tide::ResponseBuilder;

pub struct ResponseUtil;


impl ResponseUtil {
    fn code(status_code: StatusCode) -> u8 {
        if status_code.is_informational()
            || status_code.is_redirection()
            || status_code.is_success() {
            0
        } else {
            1
        }
    }

    fn json_response(status_code: StatusCode) -> Response {
        // 构造json格式的返回数据
        let mut rsp = Response::new(status_code);
        rsp.set_content_type(Mime::from_extension("json").unwrap());
        rsp
    }

    pub fn response_ok<T>(status_code: StatusCode, data: T) -> TideResult
        where T: Send + Sync + Serialize + 'static {
        let mut rsp = Self::json_response(status_code);
        rsp.set_body(json!({
            "code":Self::code(status_code),
            "data":data,
            "message":""
        }).to_string());
        return Ok(rsp);
    }

    pub fn ok<T>(data: T) -> TideResult
        where T: Send + Sync + Serialize + 'static
    {
        Self::response_ok(StatusCode::Ok, data)
    }

    pub fn error<E:Serialize + Send + 'static>(err: E) -> TideResult {
        let mut rsp = Self::json_response(StatusCode::BadRequest);
        let jv = json!({
                    "code":1,
                    "data":"",
                    "message":err
                });
        let s = jv.to_string();
        error!("{}", &s);
        rsp.set_body(s);
        Ok(rsp)
    }
}
