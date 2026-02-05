class Calculator {
  constructor() {
    this.value = 0;
    this.queue = new Promise(() => {}); // Initialize an empty promise chain
  }

  // Helper to schedule a task in the queue
  _enqueue(operation) {
    this.queue = this.queue.then(operation);
    return this;
  }

  add(num) {
    return this._enqueue(() => {
      this.value += num;
    });
  }

  sub(num) {
    return this._enqueue(() => {
      this.value -= num;
    });
  }

  mul(num) {
    return this._enqueue(() => {
      this.value *= num;
    });
  }

  delay(ms) {
    return this._enqueue(() => {
      return new Promise((resolve) => {
        console.log(`...waiting ${ms}ms...`);
        setTimeout(resolve, ms);
      });
    });
  }

  async equals() {
    // Wait for all queued operations to finish
    await this.queue;
    return this.value;
  }
}
let calc = new Calculator();
calc
  .add(10) // 10
  .delay(2000) // Wait 2 seconds
  .sub(2) // 8
  .mul(5) // 40
  .equals()
  .then((result) => console.log(`Final Result: ${result}`));
