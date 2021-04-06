use localizer::{enums::res_keeper, enums::Resource, structs::StdandartSerializer, Localizer};
use std::fs::OpenOptions;

fn main() {
    const LANGS_FILE: &str = "
	[langs.ru]
	\"ex1\" = \"ПРН{}\"
	
	[langs.en]
	\"ex1\" = \"IKA{}\"";
    localizer::set_loc_string(LANGS_FILE);
    localizer::change_localizer(&|x| {
        x.get_mut_res().res_into_ld().unwrap();
        x.get_mut_properties()
            .set_current_lang(Some("ru".to_string()));
    });
    println!("{:?}", localizer::get_by_key(&"ex1"));

    let mut loc = Localizer::create();
    let file = OpenOptions::new()
        .read(true)
        .open("./examples/lang_set.toml");
    loc.set_res(Resource::new_raw_res(
        res_keeper::new_file(file.ok()),
        StdandartSerializer::new(),
        None,
    ));

    loc.get_mut_res().get_mut_res().unwrap().stringify();

    loc.get_mut_res().res_into_ld().unwrap();
    loc.get_mut_properties()
        .set_current_lang(Some("ru".to_string()));

    println!("{:?}", loc.get(&"table"));
}
