use actix_web::{get, HttpResponse, web, post};

use crate::{app_state::AppState, pizza::Pizza};

#[get("")]
async fn get_pizza(data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(data.get_list_pizza())
}

#[post("")]
async fn post_pizza(pizza: web::Json<Pizza>, data: web::Data<AppState>) -> HttpResponse {
    let pizza = pizza.0;
    match data.add_pizza(pizza) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadGateway().body(e)
    }
    
}

#[cfg(test)]
mod test {
    use actix_web::{test, App, web};
    use reqwest::Method;

    use crate::{pizza_service::post_pizza, pizza::Pizza, app_state::AppState};

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

    #[actix_web::test]
    async fn test_post_pizza_endpoint() {
        let app_state = web::Data::new(AppState::new());
        let app = test::init_service(
            App::new()
            .app_data(app_state)
            .service(
                web::scope("/pizza")
                .service(post_pizza)
            )
        ).await;
        let test_pizza = Pizza::new(
            String::from("testPiza"),
            vec![String::from("aaa"), String::from("bbb")],
            15
        );
        let req = test::TestRequest::default()
        .uri("/pizza")
        .method(Method::POST)
        .set_json(test_pizza)
        .to_request();

        // let response = test::call_service(&app, req).await;
        // assert!(response.status().is_success());
        // assert_eq!(response.status().as_u16(), 200);

        let response: Vec<Pizza> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(1, response.len());
    }
}