// 18 lines, 10 code, 6 comments, 2 blanks
import type { TemplateOnlyComponent } from '@glimmer/component';

// A single-line comment
const localVariable = 'foo';

/**
 * A multi-line comment
 */
const Greet: TemplateOnlyComponent<{ name: string }> = <template>
  <!-- A HTML-like comment -->
  <p>Hello, {{@name}}! {{localVariable}}</p>
  <style>
    p {
      background-color: #E04E39;
    }
  </style>
</template>
