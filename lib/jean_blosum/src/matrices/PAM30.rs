pub const PAM30: [[i32; 27]; 27] = [
	[6, -3, -6, -3, -2, -8, -2, -7, -5, 0, -7, -6, -5, -4, 0, -2, -4, -7, 0, -1, 0, -2, -13, -8, -3, -3, -17],
	[-3, 6, -12, 6, 1, -10, -3, -1, -6, 0, -2, -9, -10, 6, 0, -7, -3, -7, -1, -3, 0, -8, -10, -6, 0, -5, -17],
	[-6, -12, 10, -14, -14, -13, -9, -7, -6, 0, -14, -15, -13, -11, 0, -8, -14, -8, -3, -8, 0, -6, -15, -4, -14, -9, -17],
	[-3, 6, -14, 8, 2, -15, -3, -4, -7, 0, -4, -12, -11, 2, 0, -8, -2, -10, -4, -5, 0, -8, -15, -11, 1, -5, -17],
	[-2, 1, -14, 2, 8, -14, -4, -5, -5, 0, -4, -9, -7, -2, 0, -5, 1, -9, -4, -6, 0, -6, -17, -8, 6, -5, -17],
	[-8, -10, -13, -15, -14, 9, -9, -6, -2, 0, -14, -3, -4, -9, 0, -10, -13, -9, -6, -9, 0, -8, -4, 2, -13, -8, -17],
	[-2, -3, -9, -3, -4, -9, 6, -9, -11, 0, -7, -10, -8, -3, 0, -6, -7, -9, -2, -6, 0, -5, -15, -14, -5, -5, -17],
	[-7, -1, -7, -4, -5, -6, -9, 9, -9, 0, -6, -6, -10, 0, 0, -4, 1, -2, -6, -7, 0, -6, -7, -3, -1, -5, -17],
	[-5, -6, -6, -7, -5, -2, -11, -9, 8, 0, -6, -1, -1, -5, 0, -8, -8, -5, -7, -2, 0, 2, -14, -6, -6, -5, -17],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-7, -2, -14, -4, -4, -14, -7, -6, -6, 0, 7, -8, -2, -1, 0, -6, -3, 0, -4, -3, 0, -9, -12, -9, -4, -5, -17],
	[-6, -9, -15, -12, -9, -3, -10, -6, -1, 0, -8, 7, 1, -7, 0, -7, -5, -8, -8, -7, 0, -2, -6, -7, -7, -6, -17],
	[-5, -10, -13, -11, -7, -4, -8, -10, -1, 0, -2, 1, 11, -9, 0, -8, -4, -4, -5, -4, 0, -1, -13, -11, -5, -5, -17],
	[-4, 6, -11, 2, -2, -9, -3, 0, -5, 0, -1, -7, -9, 8, 0, -6, -3, -6, 0, -2, 0, -8, -8, -4, -3, -3, -17],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-2, -7, -8, -8, -5, -10, -6, -4, -8, 0, -6, -7, -8, -6, 0, 8, -3, -4, -2, -4, 0, -6, -14, -13, -4, -5, -17],
	[-4, -3, -14, -2, 1, -13, -7, 1, -8, 0, -3, -5, -4, -3, 0, -3, 8, -2, -5, -5, 0, -7, -13, -12, 6, -5, -17],
	[-7, -7, -8, -10, -9, -9, -9, -2, -5, 0, 0, -8, -4, -6, 0, -4, -2, 8, -3, -6, 0, -8, -2, -10, -4, -6, -17],
	[0, -1, -3, -4, -4, -6, -2, -6, -7, 0, -4, -8, -5, 0, 0, -2, -5, -3, 6, 0, 0, -6, -5, -7, -5, -3, -17],
	[-1, -3, -8, -5, -6, -9, -6, -7, -2, 0, -3, -7, -4, -2, 0, -4, -5, -6, 0, 7, 0, -3, -13, -6, -6, -4, -17],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-2, -8, -6, -8, -6, -8, -5, -6, 2, 0, -9, -2, -1, -8, 0, -6, -7, -8, -6, -3, 0, 7, -15, -7, -6, -5, -17],
	[-13, -10, -15, -15, -17, -4, -15, -7, -14, 0, -12, -6, -13, -8, 0, -14, -13, -2, -5, -13, 0, -15, 13, -5, -14, -11, -17],
	[-8, -6, -4, -11, -8, 2, -14, -3, -6, 0, -9, -7, -11, -4, 0, -13, -12, -10, -7, -6, 0, -7, -5, 10, -9, -7, -17],
	[-3, 0, -14, 1, 6, -13, -5, -1, -6, 0, -4, -7, -5, -3, 0, -4, 6, -4, -5, -6, 0, -6, -14, -9, 6, -5, -17],
	[-3, -5, -9, -5, -5, -8, -5, -5, -5, 0, -5, -6, -5, -3, 0, -5, -5, -6, -3, -4, 0, -5, -11, -7, -5, -5, -17],
	[-17, -17, -17, -17, -17, -17, -17, -17, -17, 0, -17, -17, -17, -17, 0, -17, -17, -17, -17, -17, 0, -17, -17, -17, -17, -17, 1],
];
