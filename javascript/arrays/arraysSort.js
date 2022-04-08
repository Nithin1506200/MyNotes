const x = [1, 25, 3, 21, 100];
x.sort(); // out [1,100,21,25,3]

// sort is default considired as string

//! sort ascending order
x.sort(function (a, b) {
  return a - b;
});
// or
x.sort((a, b) => a - b);

//sort in descending order 
x.sort((a, b) => b - a);
console.log(x);

// sort for different objs
const cars = [
  { name: "bmw", year: "2002" },
  { name: "kmw", year: "2000" },
  { name: "amw", year: "1901" },
  { name: "fmw", year: "2001" },
];
cars.sort((a, b) => {
  // sort using ascending name
  if (a.name > b.name) {
    return 1;
  } else return -1;
});
cars.sort((a, b) => a - b); // sort using date object
console.log(cars);
