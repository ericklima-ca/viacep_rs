use std::env;

mod api;
mod validator;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: {}", "args not suplied")
    }
    let cep = args[1].clone();
    if validator::is_cep_valid(&cep) {
        match api::Api::get_geodata(cep).await {
            Ok(cep_result) => println!("{:#?}", cep_result),
            Err(err) => eprintln!("Error: {}", err),
        }
    } else {
        panic!("invalid CEP")
    }
}
