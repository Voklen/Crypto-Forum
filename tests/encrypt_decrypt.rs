use crypto_forum::encrypt_decrypt::*;

#[test]
fn normal() {
	// Definitions
	let output_file = "test_data/encrypt_decrypt";
	let test_data = [
		94, 194, 233, 166, 33, 213, 39, 55, 208, 122, 71, 138, 180, 163, 54, 115, 81, 88, 96, 77,
		175, 2, 237, 49, 125, 45, 184, 87, 27, 41, 151, 204, 67, 45, 212, 165, 145, 76, 113, 172,
		73, 98, 31, 149, 118, 220, 241, 192, 60, 151, 2, 241, 91, 135, 102, 247, 241, 135, 236, 95,
		86, 71, 62, 197,
	];
	let key = &[
		101, 247, 0, 38, 10, 86, 142, 207, 197, 98, 122, 174, 91, 22, 154, 148, 119, 155, 4, 231,
		168, 12, 41, 33, 110, 20, 4, 82, 171, 110, 60, 103,
	];

	// Actual test code
	encrypt_and_write(output_file, &test_data, key).unwrap();
	let result = read_and_decrypt(output_file, key).unwrap();
	assert_eq!(result, test_data);

	// Clean up after test
	std::fs::remove_file(output_file).unwrap();
}
