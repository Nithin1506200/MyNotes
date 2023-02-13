class Course {
  private static INSTANCE: Course;
  public static NUMBER: number;
  public nonStaticNumber: number;
  private constructor(nonStaticNumber: number) {
    console.log("the course was inititlized");
    this.nonStaticNumber = nonStaticNumber;
  }
  static instance(nonStaticNumber: number): Course {
    if (!Course.INSTANCE) {
      Course.INSTANCE = new Course(nonStaticNumber);
      Course.NUMBER = 0;
    } else {
      Course.NUMBER += 1;
    }

    console.log("the number is", Course.NUMBER);
    return Course.INSTANCE;
  }
}
const x = Course.instance(4);
const y = Course.instance(5);
console.log(x.nonStaticNumber);
console.log(y.nonStaticNumber);
