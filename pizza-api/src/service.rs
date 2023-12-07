use actix_web::{get, HttpResponse};

#[get("/pizza")]
async fn get_pizza() -> HttpResponse {
    HttpResponse::NoContent().body("this is the /pizza endpoint")
}

#[cfg(test)]
mod test {
    use actix_web::{test, App};

    use super::get_pizza;


    #[actix_web::test]
    async fn test_pizza_endpoint() {
        let app = test::init_service(
            App::new().service(get_pizza)
        ).await;
        let req = test::TestRequest::default().uri("/pizza")
        .to_request();
        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());
        assert_eq!(response.status().as_u16(), 200);
    }
}