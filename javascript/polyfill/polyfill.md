# polyfill

- [polyfill](#polyfill)
- [Array](#array)
  - [forEach](#foreach)
  - [map](#map)
  - [filter](#filter)
  - [reduce](#reduce)

`A polyfill is a piece of code (usually JavaScript on the Web) used to provide modern functionality on older browsers that do not natively support it.`

# Array

## forEach

```javascript
Array.prototype.myForEach = function (cb) {
  for (let item of this) {
    cb(this);
  }
};
```

## map

```javascript
Array.prototype.myMap = function (cb) {
  let newArray = [];
  for (let i of this) {
    newArray.push(cb(i));
  }
  return newArray;
};
```

## filter

```javascript
Array.prototype.MyFilter = function (cb) {
  let newArray = new Array();

  for (let i of this) {
    if (cb(i)) {
      newArray.push(i);
    }
  }
};
```

## reduce

```javascript
Array.prototype.MyReduce = function (cb, { initialValue }) {
  let accumulator;
  let firstIndex = 0;
  if (initialValue === undefined) {
    accumulator = this[0];
    firstIndex = 1;
  } else {
    accumulator = initialValue;
  }
  for (let i = firstIndex; i < this.length; i++) {
    accumulator = cb(accumulator, this[i], i);
  }
  return accumulator;
};
```
