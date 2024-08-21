// 27 lines, 18 code, 6 comments, 3 blanks
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
