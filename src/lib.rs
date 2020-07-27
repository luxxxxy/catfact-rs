#[cfg(test)]
mod tests;

use nanoserde::DeJson;
use std::error::Error;
use unescape::unescape;

#[cfg(feature = "catfact_ninja")]
#[derive(DeJson)]
struct CatFact {
	fact: String,
}

#[cfg(feature = "catfact_heroku")]
#[derive(DeJson)]
struct CatFact {
	text: String,
}

/// Returns a cat fact. That's it.
#[cfg(feature = "catfact_ninja")]
pub fn fetch_cat_fact() -> Result<String, Box<dyn Error>> {
	let data: CatFact = DeJson::deserialize_json(
		&unescape(
			minreq::get("https://catfact.ninja/fact")
				.with_timeout(5)
				.send()?
				.as_str()?,
		)
		.unwrap_or("".into()),
	)?;
	Ok(data.fact)
}

/// Returns a cat fact. That's it.
#[cfg(feature = "catfact_heroku")]
pub fn fetch_cat_fact() -> Result<String, Box<dyn Error>> {
	let data: CatFact = DeJson::deserialize_json(
		&unescape(
			minreq::get("https://cat-fact.herokuapp.com/facts/random?animal_type=cat")
				.with_timeout(5)
				.send()?
				.as_str()?,
		)
		.unwrap_or("".into()),
	)?;
	Ok(data.text)
}
