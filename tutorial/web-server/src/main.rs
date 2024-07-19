use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

// handler function
async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title> Calculator </title>
                <form action="/sum" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute Sum</button>
                </form>
            "#,
        )
}

// 支持从几乎任何种类的数据格式中解析数据 (JSON YAML TOML)
#[derive(Deserialize)]
struct SumParameters {
    n: u64,
    m: u64,
}

async fn post_sum(form: web::Form<SumParameters>) -> HttpResponse {
    let sum = form.n + form.m;
    let response = format!("The sum of the numbers {} and {} is <b>{}</n>\n", form.n, form.m, sum);
    HttpResponse::Ok().content_type("text/html").body(response)
}

// 属性宏, 用于启动异步运行时并做一些错误处理
#[actix_web::main]
// std::io::Result, 是 Result<T, E = std::io::Error> 的别名, 用于处理 IO 可能出现的错误
async fn main() -> std::io::Result<()> {
    // || {} 是闭包表达式
    HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
        .route("/sum", web::post().to(post_sum))
    })
    .bind("127.0.0.1:17777")?
    .run()
    .await
}
