use anyhow::Result;
use hf_hub::{api::sync::Api, Repo, RepoType};

#[tokio::main]
async fn main() -> Result<()> {
    // init Hugging Face API client
    let api = Api::new()?;
    
    // model id
    let model_id = "facebook/opt-125m"; 
    let _repo = Repo::new(model_id.to_string(), RepoType::Model);
    
    // download model
    println!("downloading model {}", model_id);
    let model_path = api.model(model_id.to_string()).get("config.json")?;
    
    println!("downloaded model to {}", model_path.display());
    
    Ok(())
}