<!-- 41 lines 15 code 21 comments 5 blanks -->
<script>
/*
Javascript multi-line

Comment
*/
 let count = 0;

 // Single line comment
 function handleClick() {
     count += 1;
 }
</script>

<!---

multi line comment

--->

<button on:click={handleClick}>
    Clicked {count} {count === 1 ? 'time' : 'times'}
</button>

<style>
/*

   CSS

   multi line


   comment
*/
 button {
     border-radius: 50;
     background-color: darkorange;
 }

</style>
