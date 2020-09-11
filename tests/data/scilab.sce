// 17 lines, 10 code, 4 comments, 3 blanks

// Hello
s = "Hello World";
disp(s);

a = [1 3 5 7 9 11; 2 4 6 8 10 12];
matrix(a, 4, 3);
matrix(a, [3 4]);
matrix(a, 3, -1);
// into an hypermatrix
matrix(a, 3, 2, 2);

// Making plot
x = [0:0.1:20];
y = sin(x);
plot(x,y);