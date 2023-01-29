/**
 * Represents a book.
 * @constructor
 * @param {string} title - The title of the book.
 * @param {string} author - The author of the book.
 */
function Book(title, author) {}

/** @constructor */
Person = function () {
  /** @constructor */
  this.Idea = function () {
    this.consider = function () {
      return "hmmm";
    };
  };
};
let x = "sdfhlskdjf";
let str = `this is nice ${x}`;

var p = new Person();
var i = new p.Idea();
i.consider();

/** @namespace */
var chat = {
  /**
   * Refer to this by {@link chat."#channel"}.
   * @namespace
   */
  "#channel": {
    /**
     * Refer to this by {@link chat."#channel".open}.
     * @type {boolean}
     * @defaultvalue
     */
    open: true,
    /**
     * Internal quotes have to be escaped by backslash. This is
     * {@link chat."#channel"."say-\"hello\""}.
     */
    'say-"hello"': function (msg) {},
  },
};

/**
 * Now we define an event in our {@link chat."#channel"} namespace.
 * @event chat."#channel"."op:announce-motd"
 */

/**
 * See {@link MyClass} and [MyClass's foo property]{@link MyClass#foo}.
 * Also, check out {@link http://www.google.com|Google} and
 * {@link https://github.com GitHub}.
 */
function myFunction() {}

// tutorial

/**
 * Class representing a socket connection. See {@link http://www.google.com google }
 * for an overview.
 *
 * @class
 */
function Socket() {}

// basic use case
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

/**
 * ## this function is used for throtle
 * use this for setting examples
 *
 * @example
 * function filter(args){ // define here}
 * let debouncedFilter = throtle(filter,1000);
 * // usage
 * debouncedFilter(args)
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

//

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
