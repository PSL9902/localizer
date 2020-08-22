extern crate localizer;
use localizer::Localizer;

/*
#[macro_use]
extern crate lazy_static;
lazy_static! {
	static ref SD : Localizer = Localizer::create();
}*/

fn main()
{
	localizer::set_loc_string(include_str!("lang_set1.toml"));
	//localizator::set_localizer(Localizer::create().file(None).current_lang(Some("ru".to_string())));
	localizer::change_localizer(&|x|{x.set_current_lang(Some("ru".to_string()));});
	println!("ss12A:{:?}", localizer::get_by_key(&"lolsf"));
	let loc = Localizer::create().file(None).current_lang(Some("ru".to_string()));
	println!("ssA:{:?}", loc.get(&"lolsf"));
}
