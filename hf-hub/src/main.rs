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
