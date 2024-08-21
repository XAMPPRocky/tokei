// 29 lines 17 code 6 comments 5 blanks

import Component from '@glimmer/component';

/** 
 * @param bar - a thing
 */
function foo(bar) {
  console.log(bar); // we foo'd the bar
}

const Greet = <template>
  {{! a Handlebars comment in the template }}
  <div>A template!</div>
  <!-- an HTML comment in the template -->
  <p>{{@greeting}}</p>
</template>;

class Example extends Component {
  get greeting() {
    // counts
    return `Hello, ${this.args.name}`; // does not count
  }
  
  <template>
    <Greet @greeting={{this.greeting}} /> {{! with an HBS comment }}
    <div>{{yield @extra}}</div> <!-- with an HTML comment -->
  </template>
}
