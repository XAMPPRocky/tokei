// 33 lines, 14 code, 12 comments, 7 blanks

/*
 * /* Nested comment
 * // single line comment
 * */

/*

function add(a, b) {
  return a + b;
}
*/

class Rectangle {
  constructor(width, height) {
    this.width = width;
    this.height = height;
  }

  get area() {
    return this.calcArea();
  }

  calcArea() {
    return this.width * this.height;
  }
}

let rect = new Rectangle(20, 20);
console.log(rect.area); // 400

// Comment
