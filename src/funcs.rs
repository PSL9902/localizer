pub fn finds(a : &mut String) -> Vec<(usize, usize)> {
	let mut ind = 0;
	let mut ind1 = 0;
	let mut flg = false;
	let mut res = Vec::new();
	while ind < a.len() && ind1 < a.len() {
		if !flg {
			loop {
				ind += match &a[ind..].find('{') {Some(x) => x, _ => break,};
				if &a[ind + 1..].find('{') == &Some(0) {
					a.replace_range(ind..ind + 2, "{");
					ind += 1;
					//ind +=match &a[ind..].find('{') {Some(x) => x, _ => break 'main,};
				}
				else
				{
					flg = true;
					break;
				}
			}
		}
		
		loop {
			ind1 += match &a[ind1..].find('}') {Some(x) => x, _ => break,};
			//println!("{:?}", ind1);
			if flg && ind1 > ind {
//				println!("{:?}", (ind, ind1));
				ind1 += 1;
				res.push((ind, ind1));
				flg = false;
			}
			else if &a[ind1+1..].find('}') == &Some(0) {
				a.replace_range(ind1..ind1 + 2, "}");
				if ind1 < ind {ind -= 1;}
				ind1 += 1;
			}
			else {
//				println!("s {:?}", &a[ind .. ind1]);
				break;
			}
		}
		//if ind>=a.len() || ind1>=a.len(){break;}
		//ind = ind + 1;
	}
	res
	//(ind1, ind1.map(|x| a[x..].find('}')).flatten())
	//Some((ind1, ind1 + a[ind1..].find('}')?))
}
pub fn finds_all(a : &mut String) -> Vec<(usize, usize, String)> {
	let mut ind = 0;
	fn d(j : &str, ind : usize) -> (String, bool) {
		let mut j : String = j.split_whitespace().collect();
		if j.len() == 0 {
			return (format!("ind_{}", ind), true);
		}
		match j.find(":") {
			Some(ind) => {j = j[0..ind].to_string();}
			None => ()
		}
		match j.chars().map(|x| x.is_digit(10)).all(|y| y) {
			true => (format!("ind_{}", j), false),
			false => (format!("name_{}", j), false),
		}
	}
	finds(a).into_iter().map(|(x, y)| (x, y, {let h = d(&a[x+1 .. y-1], ind); if h.1 {ind+=1} h.0})).collect()
}
