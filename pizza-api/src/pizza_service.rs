use actix_web::{get, HttpResponse, web};

use crate::app_state::AppState;

#[get("")]
async fn get_pizza(data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(data.get_list_pizza())
}

#[cfg(test)]
mod test {
    use actix_web::{test, App, web};

    use super::get_pizza;


    #[actix_web::test]
    async fn test_pizza_endpoint() {
        let app = test::init_service(
            App::new().service(
                web::scope("/pizza")
                .service(get_pizza)
            )
        ).await;
        let req = test::TestRequest::default().uri("/pizza")
        .to_request();
        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());
        assert_eq!(response.status().as_u16(), 200);
    }
}