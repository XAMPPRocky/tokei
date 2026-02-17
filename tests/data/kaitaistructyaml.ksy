# 21 lines 16 code 3 comments 2 blanks

# first example from here:
# https://doc.kaitai.io/user_guide.html#fixed-size-struct

meta:
  id: animal_record
  endian: be
seq:
  - id: uuid
    size: 16
  - id: name
    type: str
    size: 24
    encoding: UTF-8
  - id: birth_year
    type: u2
  - id: weight
    type: f8
  - id: rating
    type: s4
