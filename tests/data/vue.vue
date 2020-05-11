<!-- 36 lines, 24 code, 9 comments, 3 blanks -->
<template>
  <div id="app">
    <button v-on:click="clicked">
      <!-- Button that increments count -->
      Clicked {{ count }} {{ count == 1 ? "time" : "times" }}
    </button>
  </div>
</template>

<script>
/*
 Javascript Section
 */
// Single line
export default {
  data() {
    return {
      count: 0,
    };
  },

  clicked() {
    this.count++;
  },
};
</script>

<style>
/*
 Styling Section
 */
button {
  background-color: darkorange;
}
</style>
