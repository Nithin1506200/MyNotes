class Calculator {
  val: number;
  queue: Promise<void>;
  constructor() {
    this.val = 0;
    this.queue = new Promise<void>(() => {});
  }
  _enqueue(op: () => void): Calculator {
    this.queue = this.queue.then(op);
    return this;
  }
  add(i: number): Calculator {
    return this._enqueue(() => {
      this.val += i;
    });
  }

  sub(i: number): Calculator {
    return this._enqueue(() => {
      this.val -= i;
    });
  }

  mul(i: number): Calculator {
    return this._enqueue(() => {
      this.val *= i;
    });
  }

  delay(ms: number): Calculator {
    return this._enqueue(() => {
      return new Promise<void>((resolve) => {
        console.log(`...waiting ${ms}ms...`);
        setTimeout(resolve, ms);
      });
    });
  }

  async equals(): Promise<number> {
    await this.queue;
    return this.val;
  }
}
