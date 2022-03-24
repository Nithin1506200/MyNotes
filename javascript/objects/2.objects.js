// best to declare an object as const
/**
 *
 */
// ! You can create a const object:
const car = { type: "Fiat", model: "500", color: "white" };
// TODO:  You can change a property:
car.color = "red";
// ? You can add a property:
car.owner = "Johnson";

//****************** */
const car = { type: "Fiat", model: "500", color: "white" };
car = { type: "Volvo", model: "EX60", color: "red" };
// ERROR
/*************** */
const cars = ["Saab", "Volvo", "BMW"];

cars = ["Toyota", "Volvo", "Audi"];
// ERROR

/********* */
// You can create a constant array:
const cars = ["Saab", "Volvo", "BMW"];
// You can change an element:
cars[0] = "Toyota";
// You can add an element:
cars.push("Audi");
/*********** */
/********** */

const person = {
  firstName: "John",
  lastName: "Doe",
  id: 5566,
  fullName: function () {
    return this.firstName + " " + this.lastName;
  },
};
