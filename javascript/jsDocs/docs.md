# js docs

- [js docs](#js-docs)
  - [init](#init)
  - [@links](#links)
  - [@author](#author)
  - [@constant](#constant)
  - [@example](#example)
  - [basic usecase](#basic-usecase)

## init

```javascript
/**
 *
 *
 */
 functio myfun(){

 }
```

## @links

```javascript
/**
 * See {@link MyClass} and [MyClass's foo property]{@link MyClass#foo}.
 * Also, check out {@link http://www.google.com|Google} and
 * {@link https://github.com GitHub}.
 */
function myFunction() {}
```

## @author

```javascript
/**
 * @author Jane Smith <jsmith@example.com>
 */
function MyClass() {}
```

## @constant

```javascript
/** @constant
    @type {string}
    @default
*/
const RED = "FF0000";

/** @constant {number} */
var ONE = 1;
```

## @example

```javascript
/**
 * Solves equations of the form a * x = b
 * @example
 * // returns 2
 * globalNS.method1(5, 10);
 * @example
 * // returns 3
 * globalNS.method(5, 15);
 * @returns {Number} Returns the value of x for the equation.
 */
globalNS.method1 = function (a, b) {
  return b / a;
};
```

## basic usecase

```javascript
/** Class representing a point. */
class Point {
  /**
   * Create a point.
   * @param {number} x - The x value.
   * @param {number} y - The y value.
   */
  constructor(x, y) {
    // ...
  }

  /**
   * Get the x value.
   * @return {number} The x value.
   */
  getX() {
    // ...
  }

  /**
   * Get the y value.
   * @return {number} The y value.
   */
  getY() {
    // ...
  }

  /**
   * Convert a string containing two comma-separated numbers into a point.
   * @param {string} str - The string containing two comma-separated numbers.
   * @return {Point} A Point object.
   */
  static fromString(str) {
    // ...
  }
}
```

```javascript
/**
 * ## this function is used for throtle
 * use this for setting examples
 *
 *
 *
 * @author Nithin N <nithingowdan77@gmail.com>
 * @param {function} cb - callback function to be passed
 * @param {Number} wait - time in ms to be wait
 * @return {function}
 */
function throtle(cb, wait = 1000) {
  let shouldwait = false;
  let cacheArgs;
  return (...args) => {
    cacheArgs = args;
    if (shouldwait) {
      return;
    }
    cb(cacheArgs);
    shouldwait = true;
    setTimeout(() => {
      shouldwait = false;
    });
  };
}
```
