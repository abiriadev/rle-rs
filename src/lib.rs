pub fn rle(i: &str) -> Vec<(char, usize)> {
	let mut v = vec![];
	let mut c = None;
	let mut l = 0;

	for ch in i.chars() {
		if let Some(cc) = c {
			if ch != cc {
				c = Some(ch);
				v.push((cc, l));
				l = 0;
			}
		} else {
			c = Some(ch);
		}
		l += 1;
	}

	v
}
