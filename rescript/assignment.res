/**
# custom window type 
*/
// type window
@val external window:Dom.window="window"   // @val is for binding a value
@set external setVersion:(Dom.window,string)=>unit="version" // @set is for settable 
@set external onMerchantEvent:(Dom.window,()=>unit)=>unit="onMerchantEvent"
@set external onEvent:(Dom.window,('a)=>unit)=>unit="onUnit"
type parent
@val @scope(("window")) external parent:parent="parent"
@send external windowLogger:(parent,(Dom.element))=>(string)=>unit="logger"
//-----------------------------------------------------------------//
/**
Create a document type
*/
// type document
@send external getElementById:(Dom.document,string)=>option<Dom.element>="getElementById" // Functions attached to a JS objects (other than JS modules) require a special way of binding to them, using send:
@val external doc:Dom.document="document"



//------------------------Typesafe get-----------------------------//
let myEle= doc->getElementById("microapp1-logs") ;
switch myEle {
    | None => 
    Js.log("the element is not found")
    | Some(ele)=>
    Js.log("the element is found")
    let logger=windowLogger(parent,ele)
    logger("hello world")
}



//--------------------------Window section-------------------------//



window->setVersion("1.0.0")
window->onMerchantEvent(()=>(
    //something to be done
))
window->onEvent((payload)=>(
    //do something
    Js.log("print the payload"++payload)
))

//--------------JOS------------ Currying function using (. )------//


@val @scope("JOS") external emitEventWithLog:(string)=>(string)=>(string)=>unit="emitEventWithLog"
emitEventWithLog( "helllo")("world")( "😃")

@val @scope("JOS") external emitEventWithLog:(. string)=>(. string)=>(. string)=>unit="emitEventWithLog"
emitEventWithLog(. "helllo")(. "world")(. "😃")


// ()->()->("d")->("fd")->("Fds")->emitEventWithLog


//------------------------------ CURRYING 2 ------------------------//

let success=(. s1:string)=>(. s2:string)=>(. )=>Js.log(s1++" "++s2)