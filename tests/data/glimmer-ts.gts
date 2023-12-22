// 12 lines, 5 code, 4 comments, 3 blanks
import type { TemplateOnlyComponent } from '@glimmer/component';

// A single-line comment
const localVariable = 'foo';

/**
 * A multi-line comment
 */
const Greet: TemplateOnlyComponent<{ name: string }> = <template>
  <p>Hello, {{@name}}! {{localVariable}}</p>
</template>
