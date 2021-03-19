use actix_web::{post,web,HttpResponse,Responder};
use log::info;
use pam::Authenticator;
use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize)]
pub struct Request{
    username: String,
    password: String,
}

#[derive(Debug,Serialize)]
pub struct Response{
    result: bool,
}



#[post("/validate_password")]
pub async fn validate_password(request:web::Json<Request>) -> impl Responder{
    info!("validating password for {}", request.username);

    if authenticate(&request.username,&request.password){
        HttpResponse::Ok().json(Response{result: true});
    }else {
        HttpResponse::Ok().json(Response{result: false});
    }
}

pub fn authenticate(username: &str, password: &str) ->bool{
    let mut authenticator = 
        Authenticator::with_password("login").expect("Fail to init with client");

    authenticator.get_handler().set_credentials(username,password);

    authenticator.authenticate().is_ok()
}