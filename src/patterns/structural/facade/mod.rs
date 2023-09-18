mod facade;
mod cashier;
mod doctor;
mod pharmacy;

#[cfg(test)]
mod tests {
	use crate::patterns::structural::facade::facade::Facade;
	use super::facade::Hospital;

	#[test]
	fn base() {
		let hospital = Hospital::new();
		hospital.treat();
	}
}