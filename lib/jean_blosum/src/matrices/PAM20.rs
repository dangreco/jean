pub const PAM20: [[i32; 27]; 27] = [
	[6, -5, -8, -4, -3, -9, -3, -8, -6, 0, -8, -7, -6, -5, 0, -2, -5, -8, -1, -1, 0, -3, -16, -9, -4, -4, -19],
	[-5, 6, -14, 6, 0, -12, -4, -2, -7, 0, -3, -10, -12, 6, 0, -8, -4, -9, -2, -4, 0, -9, -11, -7, -1, -6, -19],
	[-8, -14, 10, -16, -16, -15, -11, -8, -7, 0, -16, -17, -16, -13, 0, -9, -16, -9, -4, -9, 0, -7, -18, -5, -16, -11, -19],
	[-4, 6, -16, 8, 2, -17, -4, -5, -9, 0, -6, -15, -13, 1, 0, -9, -4, -12, -5, -6, 0, -9, -17, -13, 0, -7, -19],
	[-3, 0, -16, 2, 8, -16, -5, -6, -6, 0, -5, -10, -8, -3, 0, -7, 0, -11, -5, -7, 0, -8, -19, -9, 6, -6, -19],
	[-9, -12, -15, -17, -16, 9, -10, -7, -3, 0, -16, -4, -5, -10, 0, -11, -15, -10, -7, -10, 0, -9, -6, 1, -16, -9, -19],
	[-3, -4, -11, -4, -5, -10, 7, -10, -13, 0, -8, -12, -10, -4, 0, -7, -8, -11, -3, -7, 0, -7, -17, -16, -6, -6, -19],
	[-8, -2, -8, -5, -6, -7, -10, 9, -11, 0, -8, -7, -13, -1, 0, -5, 0, -3, -7, -8, 0, -7, -8, -4, -2, -6, -19],
	[-6, -7, -7, -9, -6, -3, -13, -11, 9, 0, -7, -2, -2, -6, 0, -10, -9, -6, -8, -3, 0, 1, -16, -7, -7, -6, -19],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-8, -3, -16, -6, -5, -16, -8, -8, -7, 0, 7, -9, -3, -2, 0, -8, -4, -1, -5, -4, 0, -10, -14, -10, -5, -6, -19],
	[-7, -10, -17, -15, -10, -4, -12, -7, -2, 0, -9, 7, 0, -8, 0, -8, -6, -10, -9, -8, 0, -3, -7, -8, -8, -7, -19],
	[-6, -12, -16, -13, -8, -5, -10, -13, -2, 0, -3, 0, 11, -11, 0, -9, -5, -5, -6, -5, 0, -2, -15, -13, -6, -6, -19],
	[-5, 6, -13, 1, -3, -10, -4, -1, -6, 0, -2, -8, -11, 8, 0, -7, -5, -7, -1, -3, 0, -9, -9, -5, -4, -4, -19],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-2, -8, -9, -9, -7, -11, -7, -5, -10, 0, -8, -8, -9, -7, 0, 8, -4, -5, -3, -5, 0, -7, -16, -16, -5, -6, -19],
	[-5, -4, -16, -4, 0, -15, -8, 0, -9, 0, -4, -6, -5, -5, 0, -4, 9, -2, -6, -7, 0, -8, -15, -14, 7, -6, -19],
	[-8, -9, -9, -12, -11, -10, -11, -3, -6, 0, -1, -10, -5, -7, 0, -5, -2, 9, -4, -8, 0, -9, -3, -11, -5, -7, -19],
	[-1, -2, -4, -5, -5, -7, -3, -7, -8, 0, -5, -9, -6, -1, 0, -3, -6, -4, 7, 0, 0, -8, -6, -8, -6, -4, -19],
	[-1, -4, -9, -6, -7, -10, -7, -8, -3, 0, -4, -8, -5, -3, 0, -5, -7, -8, 0, 7, 0, -4, -15, -7, -7, -5, -19],
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[-3, -9, -7, -9, -8, -9, -7, -7, 1, 0, -10, -3, -2, -9, 0, -7, -8, -9, -8, -4, 0, 7, -18, -8, -8, -6, -19],
	[-16, -11, -18, -17, -19, -6, -17, -8, -16, 0, -14, -7, -15, -9, 0, -16, -15, -3, -6, -15, 0, -18, 13, -6, -17, -13, -19],
	[-9, -7, -5, -13, -9, 1, -16, -4, -7, 0, -10, -8, -13, -5, 0, -16, -14, -11, -8, -7, 0, -8, -6, 10, -11, -9, -19],
	[-4, -1, -16, 0, 6, -16, -6, -2, -7, 0, -5, -8, -6, -4, 0, -5, 7, -5, -6, -7, 0, -8, -17, -11, 6, -6, -19],
	[-4, -6, -11, -7, -6, -9, -6, -6, -6, 0, -6, -7, -6, -4, 0, -6, -6, -7, -4, -5, 0, -6, -13, -9, -6, -6, -19],
	[-19, -19, -19, -19, -19, -19, -19, -19, -19, 0, -19, -19, -19, -19, 0, -19, -19, -19, -19, -19, 0, -19, -19, -19, -19, -19, 1],
];
