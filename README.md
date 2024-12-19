This Rust workspace demonstrates the use of the huggingface `candle` crate and huggingface `hf_hub` crate.

## Workspace Members

1. hf-hub: A straightforward example of how to retrieve a model from the Hugging Face Hub and use it for inference.

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
***Run the code should be like this;If you encounter an error, please ensure your CLI can reach the Hugging Face hub.***

```markdown
{
  "sha": "fa97f6e7cb1a59073dff9e6b13e2715cf7475ac9",
  "siblings": [
    ".gitattributes",
    "1_Pooling/config.json",
    "README.md",
    "config.json",
    "config_sentence_transformers.json",
    "data_config.json",
    "model.safetensors",
    "modules.json",
    "onnx/model.onnx",
    "onnx/model_O1.onnx",
    "onnx/model_O2.onnx",
    "onnx/model_O3.onnx",
    "onnx/model_O4.onnx",
    "onnx/model_qint8_arm64.onnx",
    "onnx/model_qint8_avx512.onnx",
    "onnx/model_qint8_avx512_vnni.onnx",
    "onnx/model_quint8_avx2.onnx",
    "openvino/openvino_model.bin",
    "openvino/openvino_model.xml",
    "openvino/openvino_model_qint8_quantized.bin",
    "openvino/openvino_model_qint8_quantized.xml",
    "pytorch_model.bin",
    "rust_model.ot",
    "sentence_bert_config.json",
    "special_tokens_map.json",
    "tf_model.h5",
    "tokenizer.json",
    "tokenizer_config.json",
    "train_script.py",
    "vocab.txt"
  ]
}

```
***change the model `repo_id("sentence-transformers/all-MiniLM-L6-v2")` to get the different model's files.***

***check all the model's `repo_id` in the huggingface hub https://huggingface.co/models***