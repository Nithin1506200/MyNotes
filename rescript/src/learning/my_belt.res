let someArray=[1,2,3,4,5]
let increment = someArray->Belt.Array.map(e=>e+1) // map

let get_opt=someArray->Belt.Array.get(1) // returns options
let a= switch get_opt {
    | Some(a)=> a
    |None => -1
}

let get_exp=someArray->Belt.Array.getExn(-1) // thros exceptions

// set
// let set: (array<'a>, int, 'a) => bool
// set(arr, n, x) modifies arr in place; it replaces the nth element of arr with x.
// Returns false means not updated due to out of range.
let set_=someArray->Belt.Array.set(1,9) // sets index 1 to 9.. returns false if index is outof range