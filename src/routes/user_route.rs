use crate::models::user_model::print_user_model;
use crate::models::user_model::print_user_model as log_user_model;
pub fn print_user_route() {
    crate::models::user_model::print_user_model();
    super::health_route::print_health_route();
    print_user_model();
    log_user_model();
    println!("user_route");
}