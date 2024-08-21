// 17 lines, 10 code, 5 comments, 3 blanks
import type { TemplateOnlyComponent } from '@glimmer/component';

// A single-line comment
const localVariable = 'foo';

/**
 * A multi-line comment
 */
const Greet: TemplateOnlyComponent<{ name: string }> = <template>
  <p>Hello, {{@name}}! {{localVariable}}</p>
  <style>
    p {
      background-color: #E04E39;
    }
  </style>
</template>
