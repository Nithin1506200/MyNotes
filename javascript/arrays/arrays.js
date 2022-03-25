// creating an array
const array = [1, 2, 3, 4, 5];
const array2 = new Array("jonny", "kenny", "monny");
// length
array2.length;

/*  array methods
    ! join()
    ! toString() -> joins the array using commas
    ! pop()
    ! push()
    ! shift()
    ! unshift()
    ! slice()
    ! splice()
    




*/
array2.join(" and ");
array2.pop(); // ~ returns length
array2.push("apple");
array2.shift(); // push element at first
array2.unshift();

array2.splice(2, 0, "apple", "mango");
// 2 -> position where new elements should be added
// 0 -> how many elements should be deleted
