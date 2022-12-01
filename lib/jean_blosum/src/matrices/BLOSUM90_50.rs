pub const BLOSUM90_50: [[i32; 27]; 27] = [
	[5, -2, -1, -3, -1, -3, 0, -2, -2, 0, -1, -2, -2, -2, 0, -1, -1, -2, 1, 0, 0, -1, -4, -3, -1, -1, -6],
	[-2, 4, -4, 4, 0, -4, -2, -1, -5, 0, -1, -5, -4, 4, 0, -3, -1, -2, 0, -1, 0, -4, -6, -4, 0, -2, -6],
	[-1, -4, 9, -5, -6, -3, -4, -5, -2, 0, -4, -2, -2, -4, 0, -4, -4, -5, -2, -2, 0, -2, -4, -4, -5, -3, -6],
	[-3, 4, -5, 7, 1, -5, -2, -2, -5, 0, -1, -5, -4, 1, 0, -3, -1, -3, -1, -2, 0, -5, -6, -4, 0, -2, -6],
	[-1, 0, -6, 1, 6, -5, -3, -1, -4, 0, 0, -4, -3, -1, 0, -2, 2, -1, -1, -1, 0, -3, -5, -4, 4, -2, -6],
	[-3, -4, -3, -5, -5, 7, -5, -2, -1, 0, -4, 0, -1, -4, 0, -4, -4, -4, -3, -3, 0, -2, 0, 3, -4, -2, -6],
	[0, -2, -4, -2, -3, -5, 6, -3, -5, 0, -2, -5, -4, -1, 0, -3, -3, -3, -1, -3, 0, -5, -4, -5, -3, -2, -6],
	[-2, -1, -5, -2, -1, -2, -3, 8, -4, 0, -1, -4, -3, 0, 0, -3, 1, 0, -2, -2, 0, -4, -3, 1, 0, -2, -6],
	[-2, -5, -2, -5, -4, -1, -5, -4, 5, 0, -4, 1, 1, -4, 0, -4, -4, -4, -3, -1, 0, 3, -4, -2, -4, -2, -6],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-1, -1, -4, -1, 0, -4, -2, -1, -4, 0, 6, -3, -2, 0, 0, -2, 1, 2, -1, -1, 0, -3, -5, -3, 1, -1, -6],
	[-2, -5, -2, -5, -4, 0, -5, -4, 1, 0, -3, 5, 2, -4, 0, -4, -3, -3, -3, -2, 0, 0, -3, -2, -4, -2, -6],
	[-2, -4, -2, -4, -3, -1, -4, -3, 1, 0, -2, 2, 7, -3, 0, -3, 0, -2, -2, -1, 0, 0, -2, -2, -2, -1, -6],
	[-2, 4, -4, 1, -1, -4, -1, 0, -4, 0, 0, -4, -3, 7, 0, -3, 0, -1, 0, 0, 0, -4, -5, -3, -1, -2, -6],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-1, -3, -4, -3, -2, -4, -3, -3, -4, 0, -2, -4, -3, -3, 0, 8, -2, -3, -2, -2, 0, -3, -5, -4, -2, -2, -6],
	[-1, -1, -4, -1, 2, -4, -3, 1, -4, 0, 1, -3, 0, 0, 0, -2, 7, 1, -1, -1, 0, -3, -3, -3, 4, -1, -6],
	[-2, -2, -5, -3, -1, -4, -3, 0, -4, 0, 2, -3, -2, -1, 0, -3, 1, 6, -1, -2, 0, -3, -4, -3, 0, -2, -6],
	[1, 0, -2, -1, -1, -3, -1, -2, -3, 0, -1, -3, -2, 0, 0, -2, -1, -1, 5, 1, 0, -2, -4, -3, -1, -1, -6],
	[0, -1, -2, -2, -1, -3, -3, -2, -1, 0, -1, -2, -1, 0, 0, -2, -1, -2, 1, 6, 0, -1, -4, -2, -1, -1, -6],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-1, -4, -2, -5, -3, -2, -5, -4, 3, 0, -3, 0, 0, -4, 0, -3, -3, -3, -2, -1, 0, 5, -3, -3, -3, -2, -6],
	[-4, -6, -4, -6, -5, 0, -4, -3, -4, 0, -5, -3, -2, -5, 0, -5, -3, -4, -4, -4, 0, -3, 11, 2, -4, -3, -6],
	[-3, -4, -4, -4, -4, 3, -5, 1, -2, 0, -3, -2, -2, -3, 0, -4, -3, -3, -3, -2, 0, -3, 2, 8, -3, -2, -6],
	[-1, 0, -5, 0, 4, -4, -3, 0, -4, 0, 1, -4, -2, -1, 0, -2, 4, 0, -1, -1, 0, -3, -4, -3, 4, -1, -6],
	[-1, -2, -3, -2, -2, -2, -2, -2, -2, 0, -1, -2, -1, -2, 0, -2, -1, -2, -1, -1, 0, -2, -3, -2, -1, -2, -6],
	[-6, -6, -6, -6, -6, -6, -6, -6, -6, 0, -6, -6, -6, -6, 0, -6, -6, -6, -6, -6, 0, -6, -6, -6, -6, -6, 1],
];