# 28 lines 21 code 3 comments 4 blanks

select User {
    name,
    friends: {
        name
    },
    has_i := .friends.name ilike '%i%',
    has_o := .friends.name ilike '%o%',
} filter .has_i or .has_o;

select <User>{} ?? User {name};

# update the user with the name 'Alice Smith'
with module example
update User
filter .name = 'Alice Smith'
set {
    name := 'Alice J. Smith'
};

# update all users whose name is 'Bob'
with module example
update User
filter .name like 'Bob%'
set {
    name := User.name ++ '*'
};
