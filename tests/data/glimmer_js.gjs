// 28 lines, 18 code, 7 comments, 4 blanks
import { helper } from '@ember/component/helper';
import { modifier } from 'ember-modifier';

// A single-line comment
const plusOne = helper(([num]) => num + 1);

/**
 * A multi-line comment
 */
const setScrollPosition = modifier((element, [position]) => {
  element.scrollTop = position
});

<template>
  <!-- A HTML-like comment -->
  {{!-- But also glimmer handlebars-like comments are valid in the template --}}
  <div class="scroll-container" {{setScrollPosition @scrollPos}}>
    {{#each @items as |item index|}}
      Item #{{plusOne index}}: {{item}}
    {{/each}}
  </div>
  <style>
    div {
      background-color: #E04E39;
    }
  </style>
</template>
