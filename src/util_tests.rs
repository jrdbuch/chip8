use super::*;

fn vec_compare(va: &[u8], vb: &[u8]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| *a == *b)
}

#[test]
fn test_to_binary_encoded_decimal() {
	let val = 123;

	let res = to_binary_encoded_decimal(val, 3);
	let expect: Vec<u8> = vec![1, 2, 3];
	assert!(vec_compare(&res, &expect));
	
	let res = to_binary_encoded_decimal(val, 5);
	let expect: Vec<u8> = vec![0, 0, 1, 2, 3];
	assert!(vec_compare(&res, &expect));
	
	let res = to_binary_encoded_decimal(val, 2);
	let expect: Vec<u8> = vec![2, 3];
	assert!(vec_compare(&res, &expect));
}

#[test]
fn test_split_bytes() {
	let b: u16 = 0x1A2C;

	let res = split_bytes(b);

	assert_eq!(res.0, 0x1A);
	assert_eq!(res.1, 0x2C);
}