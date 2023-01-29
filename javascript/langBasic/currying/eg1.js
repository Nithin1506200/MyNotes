/* write a function to represent 
    summ(a)(b)(c)()=a+b+c
    summ(a)(b)()=a+b*/

function summ(a) {
  return (b) => {
    if (b === undefined) {
      return a;
    } else {
      return summ(a + b);
    }
  };
}
console.log(summ(1)(2)());
