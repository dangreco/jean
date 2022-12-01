pub const PAM40: [[i32; 27]; 27] = [
	[6, -3, -6, -3, -2, -7, -1, -6, -4, 0, -6, -5, -4, -3, 0, -1, -3, -6, 0, 0, 0, -2, -12, -7, -2, -3, -15],
	[-3, 6, -11, 6, 2, -9, -2, -1, -5, 0, -2, -8, -8, 6, 0, -6, -2, -6, -1, -2, 0, -7, -9, -6, 1, -4, -15],
	[-6, -11, 9, -12, -12, -11, -8, -7, -5, 0, -12, -13, -12, -9, 0, -7, -12, -7, -2, -7, 0, -5, -14, -3, -12, -8, -15],
	[-3, 6, -12, 7, 3, -13, -3, -3, -6, 0, -4, -11, -9, 2, 0, -7, -2, -9, -3, -4, 0, -7, -13, -10, 2, -5, -15],
	[-2, 2, -12, 3, 7, -12, -3, -4, -5, 0, -4, -8, -6, -1, 0, -5, 2, -8, -4, -5, 0, -6, -15, -8, 6, -4, -15],
	[-7, -9, -11, -13, -12, 9, -8, -5, -2, 0, -12, -2, -3, -8, 0, -9, -11, -8, -6, -8, 0, -7, -4, 2, -12, -7, -15],
	[-1, -2, -8, -3, -3, -8, 6, -8, -9, 0, -6, -9, -7, -2, 0, -5, -6, -8, -1, -5, 0, -5, -13, -12, -4, -4, -15],
	[-6, -1, -7, -3, -4, -5, -8, 9, -8, 0, -5, -5, -9, 1, 0, -3, 1, -1, -5, -6, 0, -6, -6, -3, 0, -4, -15],
	[-4, -5, -5, -6, -5, -2, -9, -8, 8, 0, -5, -1, 0, -4, 0, -7, -7, -5, -6, -2, 0, 2, -12, -5, -5, -4, -15],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-6, -2, -12, -4, -4, -12, -6, -5, -5, 0, 6, -7, -1, 0, 0, -6, -2, 1, -3, -2, 0, -8, -10, -8, -3, -4, -15],
	[-5, -8, -13, -11, -8, -2, -9, -5, -1, 0, -7, 7, 1, -6, 0, -6, -4, -8, -7, -6, 0, -2, -5, -6, -6, -5, -15],
	[-4, -8, -12, -9, -6, -3, -7, -9, 0, 0, -1, 1, 11, -7, 0, -7, -3, -3, -5, -3, 0, -1, -11, -10, -4, -4, -15],
	[-3, 6, -9, 2, -1, -8, -2, 1, -4, 0, 0, -6, -7, 7, 0, -5, -3, -5, 0, -1, 0, -7, -7, -4, -2, -3, -15],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-1, -6, -7, -7, -5, -9, -5, -3, -7, 0, -6, -6, -7, -5, 0, 8, -2, -3, -1, -3, 0, -5, -12, -12, -3, -4, -15],
	[-3, -2, -12, -2, 2, -11, -6, 1, -7, 0, -2, -4, -3, -3, 0, -2, 8, -1, -4, -5, 0, -6, -11, -10, 6, -4, -15],
	[-6, -6, -7, -9, -8, -8, -8, -1, -5, 0, 1, -8, -3, -5, 0, -3, -1, 8, -2, -5, 0, -7, -1, -9, -3, -5, -15],
	[0, -1, -2, -3, -4, -6, -1, -5, -6, 0, -3, -7, -5, 0, 0, -1, -4, -2, 6, 1, 0, -5, -4, -6, -4, -2, -15],
	[0, -2, -7, -4, -5, -8, -5, -6, -2, 0, -2, -6, -3, -1, 0, -3, -5, -5, 1, 7, 0, -2, -11, -6, -5, -3, -15],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-2, -7, -5, -7, -6, -7, -5, -6, 2, 0, -8, -2, -1, -7, 0, -5, -6, -7, -5, -2, 0, 7, -14, -6, -6, -4, -15],
	[-12, -9, -14, -13, -15, -4, -13, -6, -12, 0, -10, -5, -11, -7, 0, -12, -11, -1, -4, -11, 0, -14, 13, -4, -13, -9, -15],
	[-7, -6, -3, -10, -8, 2, -12, -3, -5, 0, -8, -6, -10, -4, 0, -12, -10, -9, -6, -6, 0, -6, -4, 10, -8, -7, -15],
	[-2, 1, -12, 2, 6, -12, -4, 0, -5, 0, -3, -6, -4, -2, 0, -3, 6, -3, -4, -5, 0, -6, -13, -8, 6, -4, -15],
	[-3, -4, -8, -5, -4, -7, -4, -4, -4, 0, -4, -5, -4, -3, 0, -4, -4, -5, -2, -3, 0, -4, -9, -7, -4, -4, -15],
	[-15, -15, -15, -15, -15, -15, -15, -15, -15, 0, -15, -15, -15, -15, 0, -15, -15, -15, -15, -15, 0, -15, -15, -15, -15, -15, 1],
];