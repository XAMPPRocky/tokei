// 14 lines 5 code 5 comments 4 blanks

/calc nav for sets of portfolios,ETFs,indices,..

/one day of ([]time;sym;price) sorted by time
n:10000000;S:-10000?`4
t:([]time:09:30:00.0+til n;sym:n?S;price:n?1.0)

/calc price deltas once
\t update deltas price by sym from`t

/for each portfolio
a:([sym:-100?S]weight:100?1.0)
\t r:select time,sums price*weight from t ij a
