let name = {
  firstname: "nithin",
  lastname: "gowda",
};
let printName = function () {
  console.log(this.firstname + " " + this.lastname);
};

let foo = printName.bind(name);
foo();

Function.prototype.myBind = function (...args) {
  let obj = this;
  return function () {
    obj.call(args[0]);
  };
};
let newFoo = printName.myBind(name);
newFoo();
