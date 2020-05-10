// 33 lines, 19 code, 7 comments, 7 blanks
/*

 Multi-line comment with blanks


 *
 */
// Comment
class Person {
  #age: number;
  #name: string; // end of line comment
  #height: number;

  constructor(age: number, name: string, height: number) {
    this.#age = age;
    this.#name = name;
    this.#height = height;
  }
}

let main = () => {
  // Comment with quote "
  let person = new Person(
    5,
    `Phill

   the giant`,
    7
  );
};

main();
