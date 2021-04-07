use localizer::{enums::{res_keeper, Resource}, structs::StandartSerializer, Localizer};
use std::fs::OpenOptions;

fn main() {
    localizer::set_loc_string(include_str!("lang_set1.toml"));
    localizer::change_localizer(&|x| {
        x.get_mut_res().res_into_ld().unwrap();
        x.get_mut_properties()
            .set_current_lang(Some("ru".to_string()));
    });
    println!("ss12A: {:?}", localizer::get_by_key(&"lolsf"));

    let mut loc = Localizer::create();
    let file = OpenOptions::new()
        .read(true)
        .open("./examples/lang_set1.toml");
    loc.set_res(Resource::new_raw_res(
        res_keeper::new_file(file.ok()),
        StandartSerializer::new(),
        None,
    ));

    loc.get_mut_res().get_mut_res().unwrap().stringify();

    loc.get_mut_res().res_into_ld().unwrap();
    loc.get_mut_properties()
        .set_current_lang(Some("ru".to_string()));

    println!("ssA: {:?}", loc.get(&"lolsf"));
}
