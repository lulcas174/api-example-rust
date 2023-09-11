// Importamos algumas coisas da biblioteca do Actix utilizando o use
// os :: significa que estamos importando um módulo dentro de outro módulo
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// o #[get("/")] é uma macro que indica que a função hello() será chamada quando o usuário acessar a rota /
// Isso significa que essa função "HELLO" pode ser chamada em outro lugar apenas com o nome dela
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}