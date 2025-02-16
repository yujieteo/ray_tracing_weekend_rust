fn main() {
	// These are not mutable
    let (image_width, image_height) : (i32, i32) = (255, 255);

	// Render
	println!("P3\n{} {}\n255", image_width, image_height);

	for j in 0..image_height {
		for i in 0..image_width {
			eprintln!("\rScanlines remaining: {} ", (image_height - j));
			// Type declared on level of functions
			let r = i as f64 / (image_width - 1) as f64;
			let g = j as f64 / (image_height - 1) as f64;
			let b : f64 = 0.0;
			
			// This seems very computer friendly, but it feels clunky.
			// I did not like this design choice.
			let ir = (255.999 * r) as i64;
			let ig = (255.999 * g) as i64;
			let ib = (255.999 * b) as i64;
		
			// Be careful, a new line is already made. No need to double add. This is different from C++
			println!("{} {} {}", ir, ig, ib);
		}
	}

}