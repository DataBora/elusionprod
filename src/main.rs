use elusion::prelude::*;
use dotenv::dotenv;
use std::env;

fn get_env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn require_env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} environment variable must be set", key))
}

#[tokio::main]
async fn main() -> ElusionResult<()>{

    let scheduler = PipelineScheduler::new("5min", || async {
        
        dotenv().ok();

        let data_dir = get_env_or_default("DATA_DIR", "/usr/src/app/data");
        let password = require_env_var("PASSWORD");
    
        let mut params = HashMap::new();
        params.insert("brandid", "370");
        params.insert("password", &password);
        params.insert("siteid", "993");
        params.insert("Datefrom", "01 jan 2023 00:00");
        params.insert("Dateto", "01 jan 2024 00:00");
        params.insert("user", "john");
    
        let api_url = require_env_var("API_URL");
        let json_path = format!("{}/netsales_2024.json", data_dir);
    
        let api = ElusionApi::new();
        api.from_api_with_params(
            &api_url,
            params,
            &json_path
        ).await?;
    
        let df = CustomDataFrame::new(&json_path, "sales").await?;
        
        let save_sales = df
            .select(["*"])
            .elusion("res").await?;
    
        // save_sales.display().await?;
    
        let sas_write = require_env_var("AZURE_SAS_TOKEN");
        let url_to_folder = require_env_var("AZURE_STORAGE_URL");
    
        save_sales.write_parquet_to_azure_with_sas(
                    "overwrite",
                    &url_to_folder,
                    &sas_write
                ).await?;
        
        Ok(())

    }).await?;

    scheduler.shutdown().await?;

    Ok(())
}