use serde::Deserialize;

pub struct Api {}

#[derive(Deserialize, Debug)]
pub struct CEPResult {
    cep: String,
    logradouro: String,
    complemento: String,
    bairro: String,
    localidade: String,
    uf: String,
    ibge: String,
    gia: String,
    ddd: String,
    siafi: String,
}

impl Api {
    pub async fn get_geodata(data: String) -> Result<CEPResult, reqwest::Error> {
        let url = format!("https://viacep.com.br/ws/{}/json/", data);
        let res = reqwest::get(url).await;
        if let Ok(response) = res {
            Ok(response.json::<CEPResult>().await?)
        } else {
            Err(res.err().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_geodata() {
        let result = Api::get_geodata("69035000".to_string()).await;
        assert!(result.is_ok());
    }
}
