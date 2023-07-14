/**
# custom window type 
*/
// type window
@val external window:Dom.window="window"   // @val is for binding a value
@set external setVersion:(Dom.window,string)=>unit="version" // @set is for settable 
@set external onMerchantEvent:(Dom.window,()=>unit)=>unit="onMerchantEvent"
@set external onEvent:(Dom.window,('a)=>unit)=>unit="onEvent"
type parent
@val @scope(("window")) external parent:parent="parent"
type payload = 
|
@deriving(abstract)
P1({
    event:string,
    
    payload: {
        "service":string
    }}) | P2({"action":string})
type obj1 = {
    eventFunction:string

}
type senderObj =
| 
@deriving(abstract)
Obj1({
    eventFunction:string,
    eventName?:string,
    payload:payload,
    receiver:string
}) | String(string);
@send external windowLogger:(parent,(option<Dom.element>))=>(. string)=>(. string)=>(. senderObj)=>unit="logger"
//-----------------------------------------------------------------//
/**
Create a document type
*/
// type document
@send external getElementById:(Dom.document,string)=>option<Dom.element>="getElementById" // Functions attached to a JS objects (other than JS modules) require a special way of binding to them, using send:
@val external doc:Dom.document="document"


//------------------------Typesafe getElementById-----------------------------//


let myEle= doc->getElementById("microapp1-logs") ;
switch myEle {
    | None => 
    Js.log("the element is not found")
    | Some(ele)=>
    Js.log("the element is found")
  
   
}
let logger=windowLogger(parent,myEle)


//--------------------------Window section-------------------------//



window->setVersion("1.0.0")
window->onMerchantEvent(()=>(
    //something to be done
))
window->onEvent((payload)=>(
    //do something
    // Js.log("print the payload"++payload)
     logger(. "external")(. "Event from child microapp i.e. microapp2 through onEvent enent listenter! ")(. payload)
))
//----------JOS
@scope("JOS")
module Jos = {
    // Module contents
    @val @scope("JOS")
    external addEventListner:(string)=>(. unit)=>unit="addEventListner"
    @val @scope("JOS")
    external parent:string="parent"
    @val @scope("JOS")
    external emitEventWithLog:(. string)=>(. string)=>(. string)=>(. 'a)=>(. 'b)=>unit ="emitEventWithLog"
    @val @scope("JOS")
    external self:string="self"
}

Jos.addEventListner("onMerchantEvent")(. )
Jos.addEventListner("onEvent")(. )
let payload:payload=P2({"action":"DUI_READY"})
// let ob=Obj1(~payload=payload,~receiver:Jos.parent,~eventFunction="onEvent")
logger(. "external")(. "Informing parent i.e " ++Jos.parent++"that microapp bundles are loaded by sending DUI_READY action to jos")(. Obj1({
    payload,
    receiver:Jos.parent,
    eventFunction:"onEvent"}
))
Jos.emitEventWithLog(. Jos.parent)(. "onEvent")(. "todo")(. ()=>())(. ()=>() )

logger(. "internal")(. "Microapp Initialised!!")(. String(""))
let payload = P1({
  event: "initiate_result",
  payload: {
    "service": Jos.self,
  },
});

logger(. "external")(. "informing parent i.e. "++ Jos.parent ++ "that microapp is initialised and preprocessing background calls is completed by sendeing initiate result event!")(. Obj1({
    eventFunction:"onEvent",
    payload,
    receiver:Jos.parent
}))
Jos.emitEventWithLog(. Jos.parent)(. "onEvent")(. "todo" )(. ()=>())(. ()=>())



@val external setTimeout:('a,int)=>unit="setTimeout"
setTimeout(()=>{
    logger(. "internal")(. "Click on Make Request button to make a request to microapp2")(. String(""));
},0)

let requestButton=doc->getElementById("requestButton")->Option.getExn;


@send external addEventListner : (Dom.element,string,Dom.event=>unit)=>unit="addEventListner"
requestButton->addEventListner( "click",(event)=>{

    let success=(. code:string)=>(. result:string)=>(. )=>{
    logger(. "external")(. "Response from microapp2!")(. String(result))


}
    logger(. "external")(. "sending request to microapp2!")(. Obj1({
        eventFunction:"onMerchantEvent",
        eventName:"Process",
        payload,
        receiver:"microapp2"
    }))
    Jos.emitEventWithLog(. "microapp2")(. "onMerchantEvent")(. "todo")(. ()=>())(. ()=>())})
// the comments are not logged