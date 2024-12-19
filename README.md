This Rust workspace demonstrates the use of the huggingface `candle` crate and huggingface `hf_hub` crate.

## Workspace Members

1. hf-hub: a simple usage example of get model from huggingface hub and use it for inference.

## Getting Started

Q1. How can I retrieve and list the model's filenames (Files Tag) in JSON format from the Hugging Face Hub using the `hf_hub` API?


```rust
use hf_hub::api::sync::Api;
use serde_json::json;

fn main() {
    let api = Api::new().unwrap();

    let repo = api.model("sentence-transformers/all-MiniLM-L6-v2".to_string()).info().unwrap();
    // print the repo in raw format(RepoInfo)
    // println!("{:?}", repo); 

    // print the repo in json format
    let json_value = json!({
        "siblings": repo.siblings.iter().map(|s| s.rfilename.clone()).collect::<Vec<_>>(),
        "sha": repo.sha
    });
    
    println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
}
```
