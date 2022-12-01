pub const BLOSUM35_50: [[i32; 27]; 27] = [
	[5, -1, -2, -1, -1, -2, 0, -2, -1, 0, 0, -2, 0, -1, 0, -2, 0, -1, 1, 0, 0, 0, -2, -1, -1, 0, -5],
	[-1, 5, -2, 5, 0, -2, 0, 0, -2, 0, 0, -2, -2, 4, 0, -1, 0, -1, 0, -1, 0, -2, -3, -2, 0, -1, -5],
	[-2, -2, 15, -3, -1, -4, -3, -4, -4, 0, -2, -2, -4, -1, 0, -4, -3, -3, -3, -1, 0, -2, -5, -5, -2, -2, -5],
	[-1, 5, -3, 8, 2, -3, -2, 0, -3, 0, -1, -2, -3, 1, 0, -1, -1, -1, -1, -1, 0, -2, -3, -2, 1, -1, -5],
	[-1, 0, -1, 2, 6, -3, -2, -1, -3, 0, 1, -1, -2, -1, 0, 0, 2, -1, 0, -1, 0, -2, -1, -1, 5, -1, -5],
	[-2, -2, -4, -3, -3, 8, -3, -3, 1, 0, -1, 2, 0, -1, 0, -4, -4, -1, -1, -1, 0, 1, 1, 3, -3, -1, -5],
	[0, 0, -3, -2, -2, -3, 7, -2, -3, 0, -1, -3, -1, 1, 0, -2, -2, -2, 1, -2, 0, -3, -1, -2, -2, -1, -5],
	[-2, 0, -4, 0, -1, -3, -2, 12, -3, 0, -2, -2, 1, 1, 0, -1, -1, -1, -1, -2, 0, -4, -4, 0, -1, -1, -5],
	[-1, -2, -4, -3, -3, 1, -3, -3, 5, 0, -2, 2, 1, -1, 0, -1, -2, -3, -2, -1, 0, 4, -1, 0, -3, 0, -5],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[0, 0, -2, -1, 1, -1, -1, -2, -2, 0, 5, -2, 0, 0, 0, 0, 0, 2, 0, 0, 0, -2, 0, -1, 1, 0, -5],
	[-2, -2, -2, -2, -1, 2, -3, -2, 2, 0, -2, 5, 3, -2, 0, -3, -2, -2, -2, 0, 0, 2, 0, 0, -2, 0, -5],
	[0, -2, -4, -3, -2, 0, -1, 1, 1, 0, 0, 3, 6, -1, 0, -3, -1, 0, -1, 0, 0, 1, 1, 0, -2, 0, -5],
	[-1, 4, -1, 1, -1, -1, 1, 1, -1, 0, 0, -2, -1, 7, 0, -2, 1, -1, 0, 0, 0, -2, -2, -2, 0, 0, -5],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-2, -1, -4, -1, 0, -4, -2, -1, -1, 0, 0, -3, -3, -2, 0, 10, 0, -2, -2, 0, 0, -3, -4, -3, 0, -1, -5],
	[0, 0, -3, -1, 2, -4, -2, -1, -2, 0, 0, -2, -1, 1, 0, 0, 7, 2, 0, 0, 0, -3, -1, 0, 4, -1, -5],
	[-1, -1, -3, -1, -1, -1, -2, -1, -3, 0, 2, -2, 0, -1, 0, -2, 2, 8, -1, -2, 0, -1, 0, 0, 0, -1, -5],
	[1, 0, -3, -1, 0, -1, 1, -1, -2, 0, 0, -2, -1, 0, 0, -2, 0, -1, 4, 2, 0, -1, -2, -1, 0, 0, -5],
	[0, -1, -1, -1, -1, -1, -2, -2, -1, 0, 0, 0, 0, 0, 0, 0, 0, -2, 2, 5, 0, 1, -2, -2, -1, 0, -5],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[0, -2, -2, -2, -2, 1, -3, -4, 4, 0, -2, 2, 1, -2, 0, -3, -3, -1, -1, 1, 0, 5, -2, 0, -2, 0, -5],
	[-2, -3, -5, -3, -1, 1, -1, -4, -1, 0, 0, 0, 1, -2, 0, -4, -1, 0, -2, -2, 0, -2, 16, 3, -1, -1, -5],
	[-1, -2, -5, -2, -1, 3, -2, 0, 0, 0, -1, 0, 0, -2, 0, -3, 0, 0, -1, -2, 0, 0, 3, 8, -1, -1, -5],
	[-1, 0, -2, 1, 5, -3, -2, -1, -3, 0, 1, -2, -2, 0, 0, 0, 4, 0, 0, -1, 0, -2, -1, -1, 4, 0, -5],
	[0, -1, -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, 0, 0, 0, 0, -1, -1, 0, -1, -5],
	[-5, -5, -5, -5, -5, -5, -5, -5, -5, 0, -5, -5, -5, -5, 0, -5, -5, -5, -5, -5, 0, -5, -5, -5, -5, -5, 1],
];
