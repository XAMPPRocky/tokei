// 19 lines, 10 code, 7 comments, 3 blanks
import type { TemplateOnlyComponent } from '@glimmer/component';

// A single-line comment
const localVariable = 'foo';

/**
 * A multi-line comment
 */
const Greet: TemplateOnlyComponent<{ name: string }> = <template>
  <!-- A HTML-like comment -->
  {{!-- But also glimmer handlebars-like comments are valid in the template --}}
  <p>Hello, {{@name}}! {{localVariable}}</p>
  <style>
    p {
      background-color: #E04E39;
    }
  </style>
</template>
