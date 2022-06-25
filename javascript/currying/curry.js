```
It is a technique in functional programming, transformation of the function of multiple arguments into several functions of a single argument in sequence. 
The translation of function happens something like this,

    function simpleFunction(param1, param2, param3, …..) => function curriedFunction(param1)(param2)(param3)(….

        ```;
function curry(f) {
  // curry(f) does the currying transform
  return function (a) {
    return function (b) {
      return f(a, b);
    };
  };
}

// usage
function sum(a, b) {
  return a + b;
}

let curriedSum = curry(sum);

alert(curriedSum(1)(2)); // 3
