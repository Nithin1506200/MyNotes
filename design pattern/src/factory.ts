class Burger1 {
  private ingredients: string[];
  constructor(ingredients: string[]) {
    this.ingredients = ingredients;
  }
  public print() {
    console.log(this.ingredients);
  }
}
class BurgerFactory {
  public createCheeseBurger(): Burger1 {
    return new Burger1(["bun", "cheese", "sause"]);
  }
  public createDeluxeCheeseBurger(): Burger1 {
    return new Burger1(["bun", "cheese", "sause", "tomatoe"]);
  }
  public createVeganBurger(): Burger1 {
    return new Burger1(["bun", "special-sause", "vegie"]);
  }
}
const burgerFactory = new BurgerFactory();
const cheeseBurger = burgerFactory.createCheeseBurger();
const createCheeseBurger = burgerFactory.createDeluxeCheeseBurger();
const veganBurger = burgerFactory.createVeganBurger();
console.log(cheeseBurger.print());
