const x = [1, 2, 3, 4, 5, 2, 3];
// 2,3

/**
 *
 * @param {array} x
 */
function findDuplicate(x) {
  const hp = new Map();
  const set = new Set();
  for (let i of x) {
    if (!hp.get(i)) {
      //   console.log(i);
      hp.set(i, 1);
    } else {
      set.add(i);
      hp.set(i, hp.get(i) + 1);
      //   print(i);
    }
  }
  return set;
}

// console.log(findDuplicate(x));

/**
 *
 * @param {array} x
 * @param {number} sum
 */
function twoSum(x, sum) {
  const hp = new Map();
  for (let i in x) {
    const difference = Math.abs(x[i] - sum);
    if (hp.get(difference)) {
      return [hp.get(difference), i];
    } else {
      hp.set(x[i], i);
    }
  }
}

console.log(twoSum([1, 2, 3, 4], 4));
