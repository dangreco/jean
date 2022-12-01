pub const BLOSUM75: [[i32; 27]; 27] = [
	[4, -2, -1, -2, -1, -3, 0, -2, -2, 0, -1, -2, -1, -2, 0, -1, -1, -2, 1, 0, 0, 0, -3, -2, -1, -1, -5],
	[-2, 4, -4, 4, 1, -4, -1, -1, -4, 0, -1, -4, -3, 3, 0, -2, 0, -1, 0, -1, 0, -4, -5, -3, 0, -2, -5],
	[-1, -4, 9, -4, -5, -2, -3, -4, -1, 0, -4, -2, -2, -3, 0, -4, -3, -4, -1, -1, 0, -1, -3, -3, -4, -2, -5],
	[-2, 4, -4, 6, 1, -4, -2, -1, -4, 0, -1, -4, -4, 1, 0, -2, -1, -2, -1, -1, 0, -4, -5, -4, 1, -2, -5],
	[-1, 1, -5, 1, 5, -4, -3, 0, -4, 0, 1, -4, -2, -1, 0, -1, 2, 0, 0, -1, 0, -3, -4, -3, 4, -1, -5],
	[-3, -4, -2, -4, -4, 6, -4, -2, 0, 0, -4, 0, 0, -4, 0, -4, -4, -3, -3, -2, 0, -1, 1, 3, -4, -2, -5],
	[0, -1, -3, -2, -3, -4, 6, -2, -5, 0, -2, -4, -3, -1, 0, -3, -2, -3, -1, -2, 0, -4, -3, -4, -2, -2, -5],
	[-2, -1, -4, -1, 0, -2, -2, 8, -4, 0, -1, -3, -2, 0, 0, -2, 1, 0, -1, -2, 0, -4, -2, 2, 0, -1, -5],
	[-2, -4, -1, -4, -4, 0, -5, -4, 4, 0, -3, 1, 1, -4, 0, -3, -3, -3, -3, -1, 0, 3, -3, -2, -4, -2, -5],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-1, -1, -4, -1, 1, -4, -2, -1, -3, 0, 5, -3, -2, 0, 0, -1, 1, 2, 0, -1, 0, -3, -4, -2, 1, -1, -5],
	[-2, -4, -2, -4, -4, 0, -4, -3, 1, 0, -3, 4, 2, -4, 0, -3, -3, -3, -3, -2, 0, 1, -2, -1, -3, -1, -5],
	[-1, -3, -2, -4, -2, 0, -3, -2, 1, 0, -2, 2, 6, -3, 0, -3, 0, -2, -2, -1, 0, 1, -2, -2, -2, -1, -5],
	[-2, 3, -3, 1, -1, -4, -1, 0, -4, 0, 0, -4, -3, 6, 0, -3, 0, -1, 0, 0, 0, -3, -4, -3, 0, -1, -5],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-1, -2, -4, -2, -1, -4, -3, -2, -3, 0, -1, -3, -3, -3, 0, 8, -2, -2, -1, -1, 0, -3, -5, -4, -2, -2, -5],
	[-1, 0, -3, -1, 2, -4, -2, 1, -3, 0, 1, -3, 0, 0, 0, -2, 6, 1, 0, -1, 0, -2, -2, -2, 3, -1, -5],
	[-2, -1, -4, -2, 0, -3, -3, 0, -3, 0, 2, -3, -2, -1, 0, -2, 1, 6, -1, -1, 0, -3, -3, -2, 0, -1, -5],
	[1, 0, -1, -1, 0, -3, -1, -1, -3, 0, 0, -3, -2, 0, 0, -1, 0, -1, 5, 1, 0, -2, -3, -2, 0, -1, -5],
	[0, -1, -1, -1, -1, -2, -2, -2, -1, 0, -1, -2, -1, 0, 0, -1, -1, -1, 1, 5, 0, 0, -3, -2, -1, -1, -5],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[0, -4, -1, -4, -3, -1, -4, -4, 3, 0, -3, 1, 1, -3, 0, -3, -2, -3, -2, 0, 0, 4, -3, -2, -3, -1, -5],
	[-3, -5, -3, -5, -4, 1, -3, -2, -3, 0, -4, -2, -2, -4, 0, -5, -2, -3, -3, -3, 0, -3, 11, 2, -3, -3, -5],
	[-2, -3, -3, -4, -3, 3, -4, 2, -2, 0, -2, -1, -2, -3, 0, -4, -2, -2, -2, -2, 0, -2, 2, 7, -3, -2, -5],
	[-1, 0, -4, 1, 4, -4, -2, 0, -4, 0, 1, -3, -2, 0, 0, -2, 3, 0, 0, -1, 0, -3, -3, -3, 4, -1, -5],
	[-1, -2, -2, -2, -1, -2, -2, -1, -2, 0, -1, -1, -1, -1, 0, -2, -1, -1, -1, -1, 0, -1, -3, -2, -1, -1, -5],
	[-5, -5, -5, -5, -5, -5, -5, -5, -5, 0, -5, -5, -5, -5, 0, -5, -5, -5, -5, -5, 0, -5, -5, -5, -5, -5, 1],
];
