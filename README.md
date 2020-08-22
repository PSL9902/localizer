##Simple library for localization used in rust projects.
##Example:

```rust
use localizer::*;
const langs_file : &str = "
[langs.ru]
\"ex1\" = \"ПРН\"

[langs.en]
\"ex1\" = \"IKA\"";
localizer::set_loc_string(langs_file);
assert!(localizer::get_by_key(&"ex1") == "IKA");
localizer::change_localizer(&|x|{x.set_current_lang(Some("ru".to_string()));});
assert!(localizer::get_by_key(&"ex1") == "ПРН");
```
