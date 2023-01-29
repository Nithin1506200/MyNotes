const obj1 = {
  name: "nithin",
  func: function () {
    return this.name;
  },
};
// console.log(obj1.func()); // nithin
const obj2 = {
  name: "jerin",
  func: () => {
    return this.name;
  },
  func2: () => {
    this.name = "tom";
    return this.name;
  },
};
console.log(obj2.func(), obj2.func2(), obj2.func()); // undefined, tom, tom
const obj3 = {
  name: "mrs mary",
  func: obj1.func,
  func2: obj2.func,
  func3: obj2.func2,
};
// console.log(obj3.func(), obj3.func2(), obj3.func3()); // mrs mary , undefined , tom
