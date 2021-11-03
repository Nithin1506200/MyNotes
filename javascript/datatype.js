datatype: primitive 


Number.MAX_VALUE
Number.MIN_VALUE



the maximum int is = 9007199254740991

for larger num use BigInt or use n at last 
let numm=1235456546542165415121n
numm+2n = if we dont use 2n there will be an error



hexadecimal 0xff32

/****************************************************** */

array

let x=[1,2,3,4,5,6,7,8]
let x=new array(1,2,3,5,6,3)
x.reverse()

x.pop()
x.shift() => removes and returns first element
x.push("value")
x.unshift("sdf") => puts new element at beginning and returns length of new array

x.splice(2,0,"value")
//Removes elements from an array and, if necessary, inserts new elements in their place, returning the deleted elements
// 2 The first parameter (2) defines the position where new elements should be added (spliced in).
//The second parameter (0) defines how many elements should be removed.
//The rest of the parameters ("Lemon" , "Kiwi") define the new elements to be added.
//The splice() method returns an array with the deleted items: 
using splice to delete
x.splice(0,2)

delete x[n]
x.slice(3)
The slice() method creates a new array. It does not remove any elements from the source array.

x.concat(arry1,arr2,srring)



x.entries()
The entries() method returns an Array Iterator object with key/value pairs.

For each item in the original array, the new iteration object will contain an array with the index as the key, and the item value as the value:

[0, "Banana"]
[1, "Orange"]
[2, "Apple"]
[3, "Mango"]

entries() does not change the original array.

/******************************** */
x.sum(fun) if any element pass the test
x.every() if all element pass the test
const ages = [32, 33, 16, 40];

ages.every(checkAge)

function checkAge(age) {
  return age > 18;
}

The every() method returns true if all elements in an array pass a test (provi
/*********** */
x.filter(functiontovalidate)  /*** 
The filter() method creates an array filled with all array elements that pass a test (provided by a function).
filter() does not execute the function for empty array elements.
filter() does not change the original array.
/********* */
x.fill("var") replace all elements in x by var


x.map(func)