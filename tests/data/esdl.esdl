# 20 lines 13 code 4 comments 3 blanks

# no module block
type default::Movie {
    required property title -> str;
    # the year of release
    property year -> int64;
    required link director -> default::Person;
    required multi link actors -> default::Person;
}

type default::Person {
    required property first_name -> str;
    required property last_name -> str;
}

abstract link friends_base {
    # declare a specific title for the link
    annotation title := 'Close contacts';
}
