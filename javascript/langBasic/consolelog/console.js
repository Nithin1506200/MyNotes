console.log(console);
let foo = 23;
let bar = "hellow";
console.log(foo, bar); // out 23 hellow
console.log({ foo, bar }); // {foo:23 , bar:'hellow'}
// ! time
console.time("Fun");
for (let i = 0; i < 10000; i++) {
  let j = i + i ** 6;
}
console.timeLog("Fun");

//!
console.timeLog("Fun");
for (let i = 0; i < 10000; i++) {
  let j = i + i ** 6;
}
console.timeEnd("Fun");

// ! different comments
// + question comments
// - minus comments
/**
 * +
 * - hellow
 */
// todo todocomments
// * starComments
