# Simple library for localization used in rust projects.

## Examples:
### 1:
```rust
use localizer::*;
const langs_file : &str = "
[langs.ru]
\"ex1\" = \"ПРН\"

[langs.en]
\"ex1\" = \"IKA\"";
localizer::set_loc_string(langs_file);
assert!(localizer::get_by_key("ex1") == "IKA");
localizer::change_localizer(&|x|{x.set_current_lang(Some("ru".to_string()));});
assert!(localizer::get_by_key("ex1") == "ПРН");
```
### 2:
```rust
use localizer::*;
const langs_file : &str = "
[langs.ru]
\"ex1\" = \"ПРН\"

[langs.dsd]
\"ex1\" = \"saПРA\"

[langs.en]
\"ex1\" = \"IKA\"";
localizer::set_loc_string(langs_file);
assert!(localizer::get_by_key3("ex1") == vec!["ПРН".to_string(), "saПРA".to_string(), "IKA".to_string(),]);
```
### 3:
[Tic_Tac_Toe game](https://github.com/PSL9902/rust_Tic_Tac_Toe/tree/master)
### 4:
And more in [examples](https://github.com/PSL9902/localizer/tree/master/examples)
