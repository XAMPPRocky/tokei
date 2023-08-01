// 47 lines 31 code 9 comments 6 blanks

import Component from '@glimmer/component';
import type { TOC } from '@ember/component/template-only';

/** 
 * @param bar - a thing
 */
function foo(bar: string): void {
  console.log(bar); // we foo'd the bar
}

const Greet: TOC<{
  Args: {
    /** the greeting to show */
    greeting: string;
  };
}> = <template>
  {{! a Handlebars comment in the template }}
  <div>A template!</div>
  <!-- an HTML comment in the template -->
  <p>{{@greeting}}</p>
</template>;

interface ExampleSig<T> {
  Args: {
    /** The name of the person to greet */
    name: string;
    /** Extra info to pass along */
    extra: T;
  };
  Blocks: {
    default: [T];
  };
}

class Example extends Component<CommonSig> {
  get greeting() {
    // counts
    return `Hello, ${this.args.name}`; // does not count
  }
  
  <template>
    <Greet @greeting={{this.greeting}} /> {{! with an HBS comment }}
    <div>{{yield @extra}}</div> <!-- with an HTML comment -->
  </template>
}
