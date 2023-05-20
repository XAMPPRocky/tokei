# 21 lines 17 code 2 comments 2 blanks

with
  dr_strange := (select Movie filter .title = "Doctor Strange"),
  benedicts := (select Person filter .name in {
    'Benedict Cumberbatch',
    'Benedict Wong'
  })
update dr_strange
set {
  actors += benedicts
};

# another comment
with new_movie := (insert Movie {
   title := "Avengers: The Kang Dynasty",
   release_year := 2025
 })
 select new_movie {
  title, release_year
};
