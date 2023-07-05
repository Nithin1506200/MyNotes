/**
 * let a type be where it dynamically changes parameter
 */
type Info = {
  name: string;
} & (
  | {
      gender: "male";
      salary: number;
    }
  | {
      gender: "female";
      weight: number;
    }
);

const f: Info = { name: "nithin", gender: "female", weight: 5 };
const m: Info = { name: "nithin", gender: "male", salary: 5 };
