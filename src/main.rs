mod config; // submodule
mod routes;
mod models;
// main module(root module)
// crate module
// 必须显示构建 module tree
fn main() {
    routes::health_route::print_health_route();
    routes::user_route::print_user_route();
    config::print_config();
    println!("main");
}
