-- 12 lines 4 code 5 comments 3 blanks


SELECT * FROM Users
WHERE FirstName is not null; -- select rows where the user has a first name

/* this is the beginning of a block comment
	insert a new user into the Users table 
	-- line comment in a block comment
*/
INSERT INTO Users (FirstName, LastName)
VALUES ("John", "Does");