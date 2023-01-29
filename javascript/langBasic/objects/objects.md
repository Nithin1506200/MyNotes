# objects

- [objects](#objects)
- [this keywoard](#this-keywoard)
  - [using arrow function no need of this keyword](#using-arrow-function-no-need-of-this-keyword)

# this keywoard

```javascript
const myObj = {
  name: "nithin",
  getname() {
    return this.name;
  },
};
console.log(myObj.getname());
```

```sh
'nithin'
```

## using arrow function no need of this keyword

```javascript
const myObj = {
  name: "nithin",
  getname: () => {
    return this.name;
  },
};
console.log(myObj.getname());
```

```sh
undefined
```
