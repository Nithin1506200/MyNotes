

@deriving(abstract)
type test_abstract= {
    event:string,
    hello_world:string,
}

let x=test_abstract(~event="hel",~hello_world="world")
let event_=eventGet(x)// get event

type test_abstract2 = {
    name:string,
    dob:string
} 
let x_:test_abstract2={name:"nithin",dob:"15/Jun/2000"}

type enum_abstract =  T1({a:string,b:string}) | T2({c:string,d:string})

let e1_:enum_abstract=T1({a:"hello",b:"world"})
let e2_:enum_abstract=T2({c:"hello",d:"world"})


let foo:(~val:enum_abstract)=>unit =(~val)=>{
  switch(val){
        |T1({a,b})=> (
            Js.log(a ++ " " ++ b)
        )
        |T2({c,d})=> ()
    }
    ()
}

//-----------------------------JS CONVERTOR--------------------¯-------//

@deriving(accessors)
type animal =
    | Dog(string)
    | Cat
let tommy:animal= dog("tommy")