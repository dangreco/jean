pub const PAM390: [[i32; 27]; 27] = [
	[1, 1, -2, 1, 1, -3, 1, -1, 0, 0, -1, -2, -1, 0, 0, 1, 0, -1, 1, 1, 0, 0, -6, -4, 0, 0, -9],
	[1, 2, -4, 3, 2, -5, 1, 1, -2, 0, 1, -3, -2, 2, 0, 0, 2, 0, 1, 0, 0, -2, -6, -4, 2, 0, -9],
	[-2, -4, 17, -5, -5, -4, -3, -4, -2, 0, -5, -6, -5, -4, 0, -3, -5, -4, 0, -2, 0, -2, -9, 1, -5, -3, -9],
	[1, 3, -5, 3, 3, -5, 1, 1, -2, 0, 1, -4, -2, 2, 0, 0, 2, 0, 1, 0, 0, -2, -7, -5, 3, 0, -9],
	[1, 2, -5, 3, 3, -5, 1, 1, -2, 0, 1, -3, -2, 2, 0, 0, 2, 0, 0, 0, 0, -1, -7, -5, 3, 0, -9],
	[-3, -5, -4, -5, -5, 12, -5, -2, 2, 0, -5, 3, 1, -4, 0, -5, -4, -5, -3, -3, 0, 0, 2, 10, -5, -2, -9],
	[1, 1, -3, 1, 1, -5, 5, -1, -2, 0, -1, -4, -2, 1, 0, 0, 0, -2, 1, 1, 0, -1, -7, -5, 0, 0, -9],
	[-1, 1, -4, 1, 1, -2, -1, 6, -2, 0, 1, -2, -2, 1, 0, 0, 3, 2, 0, -1, 0, -2, -3, 0, 2, 0, -9],
	[0, -2, -2, -2, -2, 2, -2, -2, 4, 0, -2, 3, 3, -2, 0, -1, -2, -2, -1, 0, 0, 4, -5, 0, -2, 0, -9],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-1, 1, -5, 1, 1, -5, -1, 1, -2, 0, 4, -3, 0, 1, 0, 0, 1, 4, 0, 0, 0, -2, -3, -5, 1, 0, -9],
	[-2, -3, -6, -4, -3, 3, -4, -2, 3, 0, -3, 7, 4, -3, 0, -2, -2, -3, -2, -1, 0, 3, -2, 0, -2, -1, -9],
	[-1, -2, -5, -2, -2, 1, -2, -2, 3, 0, 0, 4, 5, -1, 0, -2, -1, 0, -1, 0, 0, 2, -4, -1, -1, 0, -9],
	[0, 2, -4, 2, 2, -4, 1, 1, -2, 0, 1, -3, -1, 1, 0, 0, 1, 1, 1, 0, 0, -1, -4, -3, 1, 0, -9],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[1, 0, -3, 0, 0, -5, 0, 0, -1, 0, 0, -2, -2, 0, 0, 5, 0, 0, 1, 1, 0, -1, -6, -5, 0, 0, -9],
	[0, 2, -5, 2, 2, -4, 0, 3, -2, 0, 1, -2, -1, 1, 0, 0, 3, 1, 0, 0, 0, -1, -5, -4, 3, 0, -9],
	[-1, 0, -4, 0, 0, -5, -2, 2, -2, 0, 4, -3, 0, 1, 0, 0, 1, 6, 0, 0, 0, -2, 3, -4, 1, 0, -9],
	[1, 1, 0, 1, 0, -3, 1, 0, -1, 0, 0, -2, -1, 1, 0, 1, 0, 0, 1, 1, 0, -1, -3, -3, 0, 0, -9],
	[1, 0, -2, 0, 0, -3, 1, -1, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, -5, -3, 0, 0, -9],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[0, -2, -2, -2, -1, 0, -1, -2, 4, 0, -2, 3, 2, -1, 0, -1, -1, -2, -1, 0, 0, 4, -6, -2, -1, 0, -9],
	[-6, -6, -9, -7, -7, 2, -7, -3, -5, 0, -3, -2, -4, -4, 0, -6, -5, 3, -3, -5, 0, -6, 26, 1, -6, -4, -9],
	[-4, -4, 1, -5, -5, 10, -5, 0, 0, 0, -5, 0, -1, -3, 0, -5, -4, -4, -3, -3, 0, -2, 1, 13, -4, -2, -9],
	[0, 2, -5, 3, 3, -5, 0, 2, -2, 0, 1, -2, -1, 1, 0, 0, 3, 1, 0, 0, 0, -1, -6, -4, 3, 0, -9],
	[0, 0, -3, 0, 0, -2, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -2, 0, -1, -9],
	[-9, -9, -9, -9, -9, -9, -9, -9, -9, 0, -9, -9, -9, -9, 0, -9, -9, -9, -9, -9, 0, -9, -9, -9, -9, -9, 1],
];
