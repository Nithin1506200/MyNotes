class Burger2 {
  public buns!: string;
  public patty!: string;
  public cheese!: string;
  public setBuns(bun: string) {
    this.buns = bun;
  }
  public setPatty(patty: string) {
    this.patty = patty;
  }
  public setCheese(cheese: string) {
    this.cheese = cheese;
  }
}
class Burger2builder {
  private burger2: Burger2;
  constructor() {
    this.burger2 = new Burger2();
  }
  public setBuns(bun: string) {
    this.burger2.setBuns(bun);
    return this;
  }
  public setPatty(patty: string) {
    this.burger2.setPatty(patty);
    return this;
  }
  public setCheese(cheese: string) {
    this.burger2.setCheese(cheese);
    return this;
  }
  public build() {
    this.burger2;
  }
}
const myBurger2 = new Burger2builder()
  .setBuns("2")
  .setPatty("soft")
  .setCheese("maccoronni")
  .build();
