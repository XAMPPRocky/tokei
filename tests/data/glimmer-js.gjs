// 21 lines, 13 code, 4 comments, 4 blanks
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
  <div class="scroll-container" {{setScrollPosition @scrollPos}}>
    {{#each @items as |item index|}}
      Item #{{plusOne index}}: {{item}}
    {{/each}}
  </div>
</template>
