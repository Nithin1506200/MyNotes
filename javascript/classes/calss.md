# class

- [class](#class)
  - [static](#static)

## static

The static keyword defines static methods for classes.

Static methods are called directly on the class (Car from the example above) - without creating an instance/object (mycar) of the class.

```javascript
class Car {
  constructor(brand) {
    this.carname = brand;
  }
  static hello(x) {
    return "Hello " + x.carname;
  }
}

mycar = new Car("Ford");

car.hello(mycar);
```
