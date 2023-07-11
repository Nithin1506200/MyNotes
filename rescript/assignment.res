/**
# custom window type 
*/
type window
@val external window:window="window"   // @val is for binding a value
@set external setVersion:(window,string)=>unit="version" // @set is for settable 
@set external onMerchantEvent:(window,()=>unit)=>unit="onMerchantEvent"
@set external onEvent:(window,('a)=>unit)=>unit="onUnit"
type parent
@val @scope(("window")) external parent:parent="parent"
@send external windowLogger:(parent,(Dom.element))=>(string)=>unit="logger"
//------------------------------------------------------
/**
Create a document type
*/
type document
@send external getElementById:(document,string)=>option<Dom.element>="getElementById" // Functions attached to a JS objects (other than JS modules) require a special way of binding to them, using send:
@val external doc:document="document"
//-------------------------------------------------
let myEle= doc->getElementById("microapp1-logs") ;
switch myEle {
    | None => 
    Js.log("the element is not found")
    | Some(ele)=>
    Js.log("the element is found")
    let logger=windowLogger(parent,ele)
    logger("hello world")
}
//---------------
window->setVersion("1.0.0")
window->onMerchantEvent(()=>(
    //something to be done
))
window->onEvent((payload)=>(
    //do something
    Js.log("print the payload"++payload)
))