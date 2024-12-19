### hf-hub: A straightforward example of how to retrieve a model from the Hugging Face Hub and use it for inference.

#### Getting Started

##### Q1. How can I retrieve and list the model's filenames (Files Tag) in JSON format from the Hugging Face Hub using the `hf_hub` API?
Retrieve the model's files from the Huggingface hub for various purposes, including listing model files to manage it's parameters when your AI App needs to switch between multiple models.

Following code explain the hugging face `ApiRepo::info()` method how to get the model's files and sha date in json format then convert it to `RepoInfo` raw format.



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
***Run the code should return the following JSON format;If you encounter an error, please ensure your CLI can reach the Hugging Face hub.***

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
***Run the code should return the following `RepoInfo` raw format***

```rust
use hf_hub::api::sync::Api;

fn main() {
    let api = Api::new().unwrap();

    let repo = api.model("sentence-transformers/all-MiniLM-L6-v2".to_string()).info().unwrap();
    // print the repo in raw format(RepoInfo)
    println!("{:?}", repo); 

    
}
```
Output should be like this:

```markdown
RepoInfo { siblings: [Siblings { rfilename: ".gitattributes" }, Siblings { rfilename: "1_Pooling/config.json" }, Siblings { rfilename: "README.md" }, Siblings { rfilename: "config.json" }, Siblings { rfilename: "config_sentence_transformers.json" }, Siblings { rfilename: "data_config.json" }, Siblings { rfilename: "model.safetensors" }, Siblings { rfilename: "modules.json" }, Siblings { rfilename: "onnx/model.onnx" }, Siblings { rfilename: "onnx/model_O1.onnx" }, Siblings { rfilename: "onnx/model_O2.onnx" }, Siblings { rfilename: "onnx/model_O3.onnx" }, Siblings { rfilename: "onnx/model_O4.onnx" }, Siblings { rfilename: "onnx/model_qint8_arm64.onnx" }, Siblings { rfilename: "onnx/model_qint8_avx512.onnx" }, Siblings { rfilename: "onnx/model_qint8_avx512_vnni.onnx" }, Siblings { rfilename: "onnx/model_quint8_avx2.onnx" }, Siblings { rfilename: "openvino/openvino_model.bin" }, Siblings { rfilename: "openvino/openvino_model.xml" }, Siblings { rfilename: "openvino/openvino_model_qint8_quantized.bin" }, Siblings { rfilename: "openvino/openvino_model_qint8_quantized.xml" }, Siblings { rfilename: "pytorch_model.bin" }, Siblings { rfilename: "rust_model.ot" }, Siblings { rfilename: "sentence_bert_config.json" }, Siblings { rfilename: "special_tokens_map.json" }, Siblings { rfilename: "tf_model.h5" }, Siblings { rfilename: "tokenizer.json" }, Siblings { rfilename: "tokenizer_config.json" }, Siblings { rfilename: "train_script.py" }, Siblings { rfilename: "vocab.txt" }], sha: "fa97f6e7cb1a59073dff9e6b13e2715cf7475ac9" }

```

***change the model `repo_id("sentence-transformers/all-MiniLM-L6-v2")` to get the different model's files.***

***check all the model's `repo_id` in the huggingface hub https://huggingface.co/models***